use crate::cli::CliError;
use solana_client::{
    client_error::{ClientError, Result as ClientResult},
    rpc_client::RpcClient,
};
use solana_sdk::{
    commitment_config::CommitmentConfig, hash::Hash, message::Message,
    native_token::carats_to_gema, pubkey::Pubkey,
};

pub fn check_account_for_fee(
    rpc_client: &RpcClient,
    account_pubkey: &Pubkey,
    blockhash: &Hash,
    message: &Message,
) -> Result<(), CliError> {
    check_account_for_multiple_fees(rpc_client, account_pubkey, blockhash, &[message])
}

pub fn check_account_for_fee_with_commitment(
    rpc_client: &RpcClient,
    account_pubkey: &Pubkey,
    blockhash: &Hash,
    message: &Message,
    commitment: CommitmentConfig,
) -> Result<(), CliError> {
    check_account_for_multiple_fees_with_commitment(
        rpc_client,
        account_pubkey,
        blockhash,
        &[message],
        commitment,
    )
}

pub fn check_account_for_multiple_fees(
    rpc_client: &RpcClient,
    account_pubkey: &Pubkey,
    blockhash: &Hash,
    messages: &[&Message],
) -> Result<(), CliError> {
    check_account_for_multiple_fees_with_commitment(
        rpc_client,
        account_pubkey,
        blockhash,
        messages,
        CommitmentConfig::default(),
    )
}

pub fn check_account_for_multiple_fees_with_commitment(
    rpc_client: &RpcClient,
    account_pubkey: &Pubkey,
    blockhash: &Hash,
    messages: &[&Message],
    commitment: CommitmentConfig,
) -> Result<(), CliError> {
    check_account_for_spend_multiple_fees_with_commitment(
        rpc_client,
        account_pubkey,
        0,
        blockhash,
        messages,
        commitment,
    )
}

pub fn check_account_for_spend_multiple_fees_with_commitment(
    rpc_client: &RpcClient,
    account_pubkey: &Pubkey,
    balance: u64,
    blockhash: &Hash,
    messages: &[&Message],
    commitment: CommitmentConfig,
) -> Result<(), CliError> {
    let fee = get_fee_for_message(rpc_client, blockhash, messages)?;
    if !check_account_for_balance_with_commitment(
        rpc_client,
        account_pubkey,
        balance + fee,
        commitment,
    )
    .map_err(Into::<ClientError>::into)?
    {
        if balance > 0 {
            return Err(CliError::InsufficientFundsForSpendAndFee(
                carats_to_gema(balance),
                carats_to_gema(fee),
                *account_pubkey,
            ));
        } else {
            return Err(CliError::InsufficientFundsForFee(
                carats_to_gema(fee),
                *account_pubkey,
            ));
        }
    }
    Ok(())
}

pub fn get_fee_for_message(
    rpc_client: &RpcClient,
    blockhash: &Hash,
    messages: &[&Message],
) -> Result<u64, CliError> {
    Ok(messages
        .iter()
        .map(|message| rpc_client.get_fee_for_message(blockhash, message))
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .sum())
}

pub fn check_account_for_balance(
    rpc_client: &RpcClient,
    account_pubkey: &Pubkey,
    balance: u64,
) -> ClientResult<bool> {
    check_account_for_balance_with_commitment(
        rpc_client,
        account_pubkey,
        balance,
        CommitmentConfig::default(),
    )
}

pub fn check_account_for_balance_with_commitment(
    rpc_client: &RpcClient,
    account_pubkey: &Pubkey,
    balance: u64,
    commitment: CommitmentConfig,
) -> ClientResult<bool> {
    let carats = rpc_client
        .get_balance_with_commitment(account_pubkey, commitment)?
        .value;
    if carats != 0 && carats >= balance {
        return Ok(true);
    }
    Ok(false)
}

pub fn check_unique_pubkeys(
    pubkey0: (&Pubkey, String),
    pubkey1: (&Pubkey, String),
) -> Result<(), CliError> {
    if pubkey0.0 == pubkey1.0 {
        Err(CliError::BadParameter(format!(
            "Identical pubkeys found: `{}` and `{}` must be unique",
            pubkey0.1, pubkey1.1
        )))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use solana_client::{
        rpc_request::RpcRequest,
        rpc_response::{Response, RpcResponseContext},
    };
    use solana_sdk::system_instruction;
    use std::collections::HashMap;

    #[test]
    fn test_check_account_for_fees() {
        let account_balance = 1;
        let account_balance_response = json!(Response {
            context: RpcResponseContext { slot: 1 },
            value: json!(account_balance),
        });
        let pubkey = solana_sdk::pubkey::new_rand();

        let pubkey0 = Pubkey::new(&[0; 32]);
        let pubkey1 = Pubkey::new(&[1; 32]);
        let ix0 = system_instruction::transfer(&pubkey0, &pubkey1, 1);
        let message0 = Message::new(&[ix0], Some(&pubkey0));

        let ix0 = system_instruction::transfer(&pubkey0, &pubkey1, 1);
        let ix1 = system_instruction::transfer(&pubkey1, &pubkey0, 1);
        let message1 = Message::new(&[ix0, ix1], Some(&pubkey0));

        let mut mocks = HashMap::new();
        mocks.insert(RpcRequest::GetBalance, account_balance_response.clone());
        let rpc_client = RpcClient::new_mock_with_mocks("".to_string(), mocks);
        let blockhash = rpc_client.get_latest_blockhash().unwrap();
        check_account_for_fee(&rpc_client, &pubkey, &blockhash, &message0)
            .expect("unexpected result");

        let check_fee_response = json!(Response {
            context: RpcResponseContext { slot: 1 },
            value: json!(2),
        });
        let mut mocks = HashMap::new();
        mocks.insert(RpcRequest::GetFeeForMessage, check_fee_response);
        mocks.insert(RpcRequest::GetBalance, account_balance_response.clone());
        let rpc_client = RpcClient::new_mock_with_mocks("".to_string(), mocks);
        assert!(check_account_for_fee(&rpc_client, &pubkey, &blockhash, &message1).is_err());

        let check_fee_response = json!(Response {
            context: RpcResponseContext { slot: 1 },
            value: json!(2),
        });
        let mut mocks = HashMap::new();
        mocks.insert(RpcRequest::GetFeeForMessage, check_fee_response);
        mocks.insert(RpcRequest::GetBalance, account_balance_response);
        let rpc_client = RpcClient::new_mock_with_mocks("".to_string(), mocks);
        assert!(check_account_for_multiple_fees(
            &rpc_client,
            &pubkey,
            &blockhash,
            &[&message0, &message0]
        )
        .is_err());

        let account_balance = 2;
        let account_balance_response = json!(Response {
            context: RpcResponseContext { slot: 1 },
            value: json!(account_balance),
        });
        let check_fee_response = json!(Response {
            context: RpcResponseContext { slot: 1 },
            value: json!(1),
        });

        let mut mocks = HashMap::new();
        mocks.insert(RpcRequest::GetFeeForMessage, check_fee_response);
        mocks.insert(RpcRequest::GetBalance, account_balance_response);
        let rpc_client = RpcClient::new_mock_with_mocks("".to_string(), mocks);

        check_account_for_multiple_fees(&rpc_client, &pubkey, &blockhash, &[&message0, &message0])
            .expect("unexpected result");
    }

    #[test]
    fn test_check_account_for_balance() {
        let account_balance = 50;
        let account_balance_response = json!(Response {
            context: RpcResponseContext { slot: 1 },
            value: json!(account_balance),
        });
        let pubkey = solana_sdk::pubkey::new_rand();

        let mut mocks = HashMap::new();
        mocks.insert(RpcRequest::GetBalance, account_balance_response);
        let rpc_client = RpcClient::new_mock_with_mocks("".to_string(), mocks);

        assert!(check_account_for_balance(&rpc_client, &pubkey, 1).unwrap());
        assert!(check_account_for_balance(&rpc_client, &pubkey, account_balance).unwrap());
        assert!(!check_account_for_balance(&rpc_client, &pubkey, account_balance + 1).unwrap());
    }

    #[test]
    fn test_get_fee_for_message() {
        let check_fee_response = json!(Response {
            context: RpcResponseContext { slot: 1 },
            value: json!(1),
        });
        let mut mocks = HashMap::new();
        mocks.insert(RpcRequest::GetFeeForMessage, check_fee_response);
        let rpc_client = RpcClient::new_mock_with_mocks("".to_string(), mocks);
        let blockhash = rpc_client.get_latest_blockhash().unwrap();

        // No messages, no fee.
        assert_eq!(
            get_fee_for_message(&rpc_client, &blockhash, &[]).unwrap(),
            0
        );

        // One message w/ one signature, a fee.
        let pubkey0 = Pubkey::new(&[0; 32]);
        let pubkey1 = Pubkey::new(&[1; 32]);
        let ix0 = system_instruction::transfer(&pubkey0, &pubkey1, 1);
        let message0 = Message::new(&[ix0], Some(&pubkey0));
        assert_eq!(
            get_fee_for_message(&rpc_client, &blockhash, &[&message0]).unwrap(),
            1
        );

        // No signatures, no fee.
        let check_fee_response = json!(Response {
            context: RpcResponseContext { slot: 1 },
            value: json!(0),
        });
        let mut mocks = HashMap::new();
        mocks.insert(RpcRequest::GetFeeForMessage, check_fee_response);
        let rpc_client = RpcClient::new_mock_with_mocks("".to_string(), mocks);
        let message = Message::default();
        assert_eq!(
            get_fee_for_message(&rpc_client, &blockhash, &[&message, &message]).unwrap(),
            0
        );
    }

    #[test]
    fn test_check_unique_pubkeys() {
        let pubkey0 = solana_sdk::pubkey::new_rand();
        let pubkey_clone = pubkey0;
        let pubkey1 = solana_sdk::pubkey::new_rand();

        check_unique_pubkeys((&pubkey0, "foo".to_string()), (&pubkey1, "bar".to_string()))
            .expect("unexpected result");
        check_unique_pubkeys((&pubkey0, "foo".to_string()), (&pubkey1, "foo".to_string()))
            .expect("unexpected result");

        assert!(check_unique_pubkeys(
            (&pubkey0, "foo".to_string()),
            (&pubkey_clone, "bar".to_string())
        )
        .is_err());
    }
}
