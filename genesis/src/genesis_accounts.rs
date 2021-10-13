use crate::{
    stakes::{create_and_add_stakes, StakerInfo},
    unlocks::UnlockInfo,
};
use solana_sdk::{genesis_config::GenesisConfig, native_token::CARATS_PER_GEMA};

// 9 month schedule is 100% after 9 months
const UNLOCKS_ALL_AT_9_MONTHS: UnlockInfo = UnlockInfo {
    cliff_fraction: 1.0,
    cliff_years: 0.75,
    unlocks: 0,
    unlock_years: 0.0,
    custodian: "Mc5XB47H3DKJHym5RLa9mPzWv5snERsF3KNv5AauXK8",
};

// 9 month schedule is 50% after 9 months, then monthly for 2 years
const UNLOCKS_HALF_AT_9_MONTHS: UnlockInfo = UnlockInfo {
    cliff_fraction: 0.5,
    cliff_years: 0.75,
    unlocks: 24,
    unlock_years: 2.0,
    custodian: "Mc5XB47H3DKJHym5RLa9mPzWv5snERsF3KNv5AauXK8",
};

// no lockups
const UNLOCKS_ALL_DAY_ZERO: UnlockInfo = UnlockInfo {
    cliff_fraction: 1.0,
    cliff_years: 0.0,
    unlocks: 0,
    unlock_years: 0.0,
    custodian: "Mc5XB47H3DKJHym5RLa9mPzWv5snERsF3KNv5AauXK8",
};

pub const CREATOR_STAKER_INFOS: &[StakerInfo] = &[
    StakerInfo {
        name: "impossible pizza",
        staker: "uE3TVEffRp69mrgknYr71M18GDqL7GxCNGYYRjb3oUt",
        carats: 5_000_000 * CARATS_PER_GEMA,
        withdrawer: Some("59SLqk4ete5QttM1WmjfMA7uNJnJVFLQqXJSy9rvuj7c"),
    },
    StakerInfo {
        name: "nutritious examination",
        staker: "9noVEZreMmgQvE8iyKmxy7CGTJ2enELyuJ1qxFtXrfJB",
        carats: 5_000_000 * CARATS_PER_GEMA,
        withdrawer: Some("ERnx3Csgu3LjrGGrCeCUZzuHguRu6XabT1kufSB1NDWi"),
    },
    StakerInfo {
        name: "tidy impression",
        staker: "BU7LA4kYvicfPCp22EM2Tth3eaeWAXYo6yCgWXQFJ42z",
        carats: 5_000_000 * CARATS_PER_GEMA,
        withdrawer: Some("5eKcGy7ZCPJdQSQGVnfmT7kGz6MKPMKaNaMEYJbmwhuT"),
    },
    StakerInfo {
        name: "dramatic treatment",
        staker: "BrNFrFeuev8TosKhRe2kvVZTYrcUuYaqCfptWutxs17B",
        carats: 1_205_602 * CARATS_PER_GEMA,
        withdrawer: Some("2pKqwFKfKj2nGrknPNDSP8vXGYrgAjd28fT6yLew8sT3"),
    },
    StakerInfo {
        name: "angry noise",
        staker: "34HCVh8Yx4jNkaeLUQEKibFKUZDPQMjWzkXy8qUfdhS4",
        carats: 5_000_000 * CARATS_PER_GEMA,
        withdrawer: Some("Hw3sP6PreBtFCnwXbNvUypMhty62GXibjfiZ1zHBXFk6"),
    },
    StakerInfo {
        name: "hard cousin",
        staker: "AyZb3xrZE8wnS6gYBdsJg5v8CjyrX2ZGXU2zMakCFyYd",
        carats: 5_000_000 * CARATS_PER_GEMA,
        withdrawer: Some("9j3WzBSZRHrD2DbzFTUVVi81QX6boVvUTpGWcSiMwD5W"),
    },
    StakerInfo {
        name: "lopsided skill",
        staker: "7SbpY8LmZUb5XRqDbyoreUrSVVV9c39wkpEz81kEAXu5",
        carats: 5_000_000 * CARATS_PER_GEMA,
        withdrawer: Some("EJyZGbQ1PmpcWxfqGME6SUNHfurh1zggDqCT7rV9xLzL"),
    },
    StakerInfo {
        name: "red snake",
        staker: "C9CfFpmLDsQsz6wt7MrrZquNB5oS4QkpJkmDAiboVEZZ",
        carats: 3_655_292 * CARATS_PER_GEMA,
        withdrawer: Some("JBGnGdLyo7V2z9hz51mnnbyDp9sBACtw5WYH9YRG8n7e"),
    },
    StakerInfo {
        name: "jolly year",
        staker: "5WbxKiW7bghkr8JN6ZAv2TQt4PfJFvtuqNaN8gyQ5UzU",
        carats: 5_000_000 * CARATS_PER_GEMA,
        withdrawer: Some("43XAfG3AFiF1ockdh7xp91fpFyZkbWSZq9ZFBCGUVV41"),
    },
    StakerInfo {
        name: "typical initiative",
        staker: "Gc8XnHU6Nnriwt9RbEwi7PTosx4YanLyXak9GTbB8VaH",
        carats: 5_000_000 * CARATS_PER_GEMA,
        withdrawer: Some("7s2GVwFo8VSrCwX9Tztt42ueiEaUtJ6zCEHU8XGvuf5E"),
    },
    StakerInfo {
        name: "deserted window",
        staker: "AMmYEynkd78uNTZDFFrMw6NKjWTgqW7M8EFjvajk23VR",
        carats: 3_655_292 * CARATS_PER_GEMA,
        withdrawer: Some("23PJYLS1WFLqhXnXq2Hobc17DbvZaoinoTZYLyGRT8E2"),
    },
    StakerInfo {
        name: "eight nation",
        staker: "4qWoqt71p7h6siSDS6osu4oVWpw8R7E6uYYiY7Z6oJbH",
        carats: 103_519 * CARATS_PER_GEMA,
        withdrawer: Some("6bFjx3egMjVsGKFb445564a4bwgibwbUB2tVFsJcdPv7"),
    },
    StakerInfo {
        name: "earsplitting meaning",
        staker: "GYitoBY53E9awc56NWHJ2kxMwj4do5GSmvTRowjGaRDw",
        carats: 5_000_000 * CARATS_PER_GEMA,
        withdrawer: Some("jXMEkVQQpoqebVMGN7DfpvdRLwJDEkoVNrwPVphNm7i"),
    },
    StakerInfo {
        name: "alike cheese",
        staker: "Drg9uSvSEfjtn15jqmmrEQnA4pvU1ToYSGSa1Dv9C6Fk",
        carats: 3_880_295 * CARATS_PER_GEMA,
        withdrawer: Some("BxmwgfnyAqZnqRCJGdsEea35pcc92GFTcyGeSj4RNfJJ"),
    },
    StakerInfo {
        name: "noisy honey",
        staker: "95HsPFFvwbWpk42MKzenauSoULNzk8Tg6fc6EiJhLsUZ",
        carats: 5_000_000 * CARATS_PER_GEMA,
        withdrawer: Some("Aj3K933zdRQhYEJi2Yjz8hJWXN3Z3hrKJQtPtE8VmUnq"),
    },
];

pub const SERVICE_STAKER_INFOS: &[StakerInfo] = &[
    StakerInfo {
        name: "wretched texture",
        staker: "B1hegzthtfNQxyEPzkESySxRjMidNqaxrzbQ28GaEwn8",
        carats: 225_000 * CARATS_PER_GEMA,
        withdrawer: Some("HWzeqw1Yk5uiLgT2uGUim5ocFJNCwYUFbeCtDVpx9yUb"),
    },
    StakerInfo {
        name: "unbecoming silver",
        staker: "4AcoZa1P8fF5XK21RJsiuMRZPEScbbWNc75oakRFHiBz",
        carats: 28_800 * CARATS_PER_GEMA,
        withdrawer: None,
    },
    StakerInfo {
        name: "inexpensive uncle",
        staker: "AkJ7yssRqS3X4UWLUsPTxbP6LfVgdPYBWH4Jgk5EETgZ",
        carats: 300_000 * CARATS_PER_GEMA,
        withdrawer: Some("6mudxxoe5VyXXNXsJ3NSGSTGESfG2t86PBCQGbouHpXX"),
    },
    StakerInfo {
        name: "hellish money",
        staker: "4DVkqvRP8y26JvzNwsnQEQuC7HASwpGs58GsAT9XJMVg",
        carats: 200_000 * CARATS_PER_GEMA,
        withdrawer: Some("ASJpWZAxY96kbciLqzb7sg45gsH32yPzGcxjn7HPcARn"),
    },
    StakerInfo {
        name: "full grape",
        staker: "B2EWnwgmNd3KMpD71yZMijhML1jd4TYp96zJdhMiWZ7b",
        carats: 450_000 * CARATS_PER_GEMA,
        withdrawer: Some("9oaCkokBBhgBsgyg4sL7fMJyQseaJb1TbADZeoPdpWdc"),
    },
    StakerInfo {
        name: "nice ghost",
        staker: "HtQS1CH3nsUHmnLpenj5W6KHzFWTf3mzCn1mTqK7LkB7",
        carats: 650_000 * CARATS_PER_GEMA,
        withdrawer: Some("4YnNnycEZXCkuVs2hDthdNxMD4E8wc7ZPgyAK7Lm1uZc"),
    },
];

pub const FOUNDATION_STAKER_INFOS: &[StakerInfo] = &[
    StakerInfo {
        name: "lyrical supermarket",
        staker: "4xh7vtQCTim3vgpQ1dQQWjtKrBSkbtL3s15FimXVJAAP",
        carats: 5_000_000 * CARATS_PER_GEMA,
        withdrawer: Some("C7WS9ic7KN9XNcLsNoMvzTvbzURM3rFGDEQN7qJMWNLn"),
    },
    StakerInfo {
        name: "frequent description",
        staker: "95Nf8XfoecteSXU9nbcvzkrFQdu6FqPaH3EvhwLaC83t",
        carats: 57_500_000 * CARATS_PER_GEMA,
        withdrawer: Some("FdGYQdiRky8NZzN9wZtczTBcWLYYRXrJ3LMDhqDPn5rM"),
    },
];

pub const GRANTS_STAKER_INFOS: &[StakerInfo] = &[
    StakerInfo {
        name: "rightful agreement",
        staker: "8w5cgUQfXAZZWyVgenPHpQ1uABXUVLnymqXbuZPx7yqt",
        carats: 5_000_000 * CARATS_PER_GEMA,
        withdrawer: Some("EDwSQShtUWQtmFfN9SpUUd6hgonL7tRdxngAsNKv9Pe6"),
    },
    StakerInfo {
        name: "tasty location",
        staker: "9eyXtP43dCp59oyvWG2R7WQCeJ2bA6TWoLzXg1KTDfQQ",
        carats: 15_000_000 * CARATS_PER_GEMA,
        withdrawer: Some("9BgvWHerNACjnx6ZpK51k2LEsnwBP3gFwWDzhKkHKH1m"),
    },
];

pub const COMMUNITY_STAKER_INFOS: &[StakerInfo] = &[
    StakerInfo {
        name: "shrill charity",
        staker: "Eo1iDtrZZiAkQFA8u431hedChaSUnPbU8MWg849MFvEZ",
        carats: 5_000_000 * CARATS_PER_GEMA,
        withdrawer: Some("8CUUMKYNGxdgYio5CLHRHyzMEhhVRMcqefgE6dLqnVRK"),
    },
    StakerInfo {
        name: "legal gate",
        staker: "7KCzZCbZz6V1U1YXUpBNaqQzQCg2DKo8JsNhKASKtYxe",
        carats: 30_301_032 * CARATS_PER_GEMA,
        withdrawer: Some("92viKFftk1dJjqJwreFqT2qHXxjSUuEE9VyHvTdY1mpY"),
    },
    StakerInfo {
        name: "cluttered complaint",
        staker: "2J8mJU6tWg78DdQVEqMfpN3rMeNbcRT9qGL3yLbmSXYL",
        carats: 153_333_633 * CARATS_PER_GEMA + 41 * CARATS_PER_GEMA / 100,
        withdrawer: Some("7kgfDmgbEfypBujqn4tyApjf8H7ZWuaL3F6Ah9vQHzgR"),
    },
];

fn add_stakes(
    genesis_config: &mut GenesisConfig,
    staker_infos: &[StakerInfo],
    unlock_info: &UnlockInfo,
) -> u64 {
    staker_infos
        .iter()
        .map(|staker_info| create_and_add_stakes(genesis_config, staker_info, unlock_info, None))
        .sum::<u64>()
}

pub fn add_genesis_accounts(genesis_config: &mut GenesisConfig, mut issued_carats: u64) {
    // add_stakes() and add_validators() award tokens for rent exemption and
    //  to cover an initial transfer-free period of the network

    issued_carats += add_stakes(
        genesis_config,
        CREATOR_STAKER_INFOS,
        &UNLOCKS_HALF_AT_9_MONTHS,
    ) + add_stakes(
        genesis_config,
        SERVICE_STAKER_INFOS,
        &UNLOCKS_ALL_AT_9_MONTHS,
    ) + add_stakes(
        genesis_config,
        FOUNDATION_STAKER_INFOS,
        &UNLOCKS_ALL_DAY_ZERO,
    ) + add_stakes(genesis_config, GRANTS_STAKER_INFOS, &UNLOCKS_ALL_DAY_ZERO)
        + add_stakes(
            genesis_config,
            COMMUNITY_STAKER_INFOS,
            &UNLOCKS_ALL_DAY_ZERO,
        );

    // "one thanks" (community pool) gets 500_000_000SOL (total) - above distributions
    create_and_add_stakes(
        genesis_config,
        &StakerInfo {
            name: "one thanks",
            staker: "7vEAL3nS9CWmy1q6njUUyHE7Cf5RmyQpND6CsoHjzPiR",
            carats: (500_000_000 * CARATS_PER_GEMA).saturating_sub(issued_carats),
            withdrawer: Some("3FFaheyqtyAXZSYxDzsr5CVKvJuvZD1WE1VEsBtDbRqB"),
        },
        &UNLOCKS_ALL_DAY_ZERO,
        None,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_genesis_accounts() {
        let mut genesis_config = GenesisConfig::default();

        add_genesis_accounts(&mut genesis_config, 0);

        let carats = genesis_config
            .accounts
            .iter()
            .map(|(_, account)| account.carats)
            .sum::<u64>();

        assert_eq!(500_000_000 * CARATS_PER_GEMA, carats);
    }
}
