//! calculate and collect rent from Accounts
use solana_sdk::{
    account::{AccountSharedData, ReadableAccount, WritableAccount},
    clock::Epoch,
    epoch_schedule::EpochSchedule,
    genesis_config::GenesisConfig,
    incinerator,
    pubkey::Pubkey,
    rent::Rent,
    sysvar,
};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, AbiExample)]
pub struct RentCollector {
    pub epoch: Epoch,
    pub epoch_schedule: EpochSchedule,
    pub slots_per_year: f64,
    pub rent: Rent,
}

impl Default for RentCollector {
    fn default() -> Self {
        Self {
            epoch: Epoch::default(),
            epoch_schedule: EpochSchedule::default(),
            // derive default value using GenesisConfig::default()
            slots_per_year: GenesisConfig::default().slots_per_year(),
            rent: Rent::default(),
        }
    }
}

impl RentCollector {
    pub fn new(
        epoch: Epoch,
        epoch_schedule: &EpochSchedule,
        slots_per_year: f64,
        rent: &Rent,
    ) -> Self {
        Self {
            epoch,
            epoch_schedule: *epoch_schedule,
            slots_per_year,
            rent: *rent,
        }
    }

    pub fn clone_with_epoch(&self, epoch: Epoch) -> Self {
        Self {
            epoch,
            ..self.clone()
        }
    }

    // updates this account's carats and status and returns
    //  the account rent collected, if any
    // This is NOT thread safe at some level. If we try to collect from the same account in parallel, we may collect twice.
    #[must_use = "add to Bank::collected_rent"]
    pub fn collect_from_existing_account(
        &self,
        address: &Pubkey,
        account: &mut AccountSharedData,
        rent_for_sysvars: bool,
    ) -> u64 {
        if account.executable() // executable accounts must be rent-exempt balance
            || account.rent_epoch() > self.epoch
            || (!rent_for_sysvars && sysvar::check_id(account.owner()))
            || *address == incinerator::id()
        {
            0
        } else {
            let slots_elapsed: u64 = (account.rent_epoch()..=self.epoch)
                .map(|epoch| self.epoch_schedule.get_slots_in_epoch(epoch + 1))
                .sum();

            // avoid infinite rent in rust 1.45
            let years_elapsed = if self.slots_per_year != 0.0 {
                slots_elapsed as f64 / self.slots_per_year
            } else {
                0.0
            };

            let (rent_due, exempt) =
                self.rent
                    .due(account.carats(), account.data().len(), years_elapsed);

            if exempt || rent_due != 0 {
                if account.carats() > rent_due {
                    account.set_rent_epoch(
                        self.epoch
                            + if exempt {
                                // Rent isn't collected for the next epoch
                                // Make sure to check exempt status later in current epoch again
                                0
                            } else {
                                // Rent is collected for next epoch
                                1
                            },
                    );
                    let _ = account.checked_sub_carats(rent_due); // will not fail. We check above.
                    rent_due
                } else {
                    let rent_charged = account.carats();
                    *account = AccountSharedData::default();
                    rent_charged
                }
            } else {
                // maybe collect rent later, leave account alone
                0
            }
        }
    }

    #[must_use = "add to Bank::collected_rent"]
    pub fn collect_from_created_account(
        &self,
        address: &Pubkey,
        account: &mut AccountSharedData,
        rent_for_sysvars: bool,
    ) -> u64 {
        // initialize rent_epoch as created at this epoch
        account.set_rent_epoch(self.epoch);
        self.collect_from_existing_account(address, account, rent_for_sysvars)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::account::Account;

    #[test]
    fn test_collect_from_account_created_and_existing() {
        let old_carats = 1000;
        let old_epoch = 1;
        let new_epoch = 3;

        let (mut created_account, mut existing_account) = {
            let account = AccountSharedData::from(Account {
                carats: old_carats,
                rent_epoch: old_epoch,
                ..Account::default()
            });

            (account.clone(), account)
        };

        let rent_collector = RentCollector::default().clone_with_epoch(new_epoch);

        // collect rent on a newly-created account
        let collected = rent_collector.collect_from_created_account(
            &solana_sdk::pubkey::new_rand(),
            &mut created_account,
            true,
        );
        assert!(created_account.carats() < old_carats);
        assert_eq!(created_account.carats() + collected, old_carats);
        assert_ne!(created_account.rent_epoch(), old_epoch);

        // collect rent on a already-existing account
        let collected = rent_collector.collect_from_existing_account(
            &solana_sdk::pubkey::new_rand(),
            &mut existing_account,
            true,
        );
        assert!(existing_account.carats() < old_carats);
        assert_eq!(existing_account.carats() + collected, old_carats);
        assert_ne!(existing_account.rent_epoch(), old_epoch);

        // newly created account should be collected for less rent; thus more remaining balance
        assert!(created_account.carats() > existing_account.carats());
        assert_eq!(created_account.rent_epoch(), existing_account.rent_epoch());
    }

    #[test]
    fn test_rent_exempt_temporal_escape() {
        let mut account = AccountSharedData::default();
        let epoch = 3;
        let huge_carats = 123_456_789_012;
        let tiny_carats = 789_012;
        let mut collected;
        let pubkey = solana_sdk::pubkey::new_rand();

        account.set_carats(huge_carats);
        assert_eq!(account.rent_epoch(), 0);

        // create a tested rent collector
        let rent_collector = RentCollector::default().clone_with_epoch(epoch);

        // first mark account as being collected while being rent-exempt
        collected = rent_collector.collect_from_existing_account(&pubkey, &mut account, true);
        assert_eq!(account.carats(), huge_carats);
        assert_eq!(collected, 0);

        // decrease the balance not to be rent-exempt
        account.set_carats(tiny_carats);

        // ... and trigger another rent collection on the same epoch and check that rent is working
        collected = rent_collector.collect_from_existing_account(&pubkey, &mut account, true);
        assert_eq!(account.carats(), tiny_carats - collected);
        assert_ne!(collected, 0);
    }

    #[test]
    fn test_rent_exempt_sysvar() {
        let tiny_carats = 1;
        let mut account = AccountSharedData::default();
        account.set_owner(sysvar::id());
        account.set_carats(tiny_carats);

        let pubkey = solana_sdk::pubkey::new_rand();

        assert_eq!(account.rent_epoch(), 0);

        let epoch = 3;
        let rent_collector = RentCollector::default().clone_with_epoch(epoch);

        // old behavior: sysvars are special-cased
        let collected = rent_collector.collect_from_existing_account(&pubkey, &mut account, false);
        assert_eq!(account.carats(), tiny_carats);
        assert_eq!(collected, 0);

        // new behavior: sysvars are NOT special-cased
        let collected = rent_collector.collect_from_existing_account(&pubkey, &mut account, true);
        assert_eq!(account.carats(), 0);
        assert_eq!(collected, 1);
    }
}
