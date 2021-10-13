use {
    solana_sdk::{
        hash::Hash, pubkey::Pubkey, signature::Keypair, system_transaction,
        transaction::Transaction,
    },
    std::{
        io::{Error, ErrorKind},
        net::SocketAddr,
    },
};

pub fn request_airdrop_transaction(
    _faucet_addr: &SocketAddr,
    _id: &Pubkey,
    carats: u64,
    _blockhash: Hash,
) -> Result<Transaction, Error> {
    if carats == 0 {
        Err(Error::new(ErrorKind::Other, "Airdrop failed"))
    } else {
        let key = Keypair::new();
        let to = solana_sdk::pubkey::new_rand();
        let blockhash = Hash::default();
        let tx = system_transaction::transfer(&key, &to, carats, blockhash);
        Ok(tx)
    }
}
