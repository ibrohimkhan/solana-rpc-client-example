use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::{
    signature::{Keypair, Signature},
    system_transaction,
};

use std::error::Error;

const LAMPORTS_PER_SOL: f64 = 1_000_000_000_f64;

pub fn create_keypair() -> Keypair {
    Keypair::new()
}

pub fn request_airdrop(
    rpc_client: &RpcClient,
    pub_key: &Pubkey,
    amount_sol: f64,
) -> Result<Signature, Box<dyn Error>> {
    let sig = rpc_client.request_airdrop(&pub_key, (amount_sol * LAMPORTS_PER_SOL) as u64)?;
    loop {
        let confirmed = rpc_client.confirm_transaction(&sig)?;
        if confirmed {
            break;
        }
    }

    Ok(sig)
}

pub fn check_balance(rpc_client: &RpcClient, pub_key: &Pubkey) -> Result<f64, Box<dyn Error>> {
    Ok(rpc_client.get_balance(&pub_key)? as f64 / LAMPORTS_PER_SOL)
}

pub fn transfer_funds(
    rpc_client: &RpcClient,
    sender_keypair: &Keypair,
    receiver_pub_key: &Pubkey,
    amount_sol: f64,
) -> Result<Signature, Box<dyn Error>> {
    let amount_lamports = (amount_sol * LAMPORTS_PER_SOL) as u64;

    let tx = system_transaction::transfer(
        &sender_keypair,
        &receiver_pub_key,
        amount_lamports,
        rpc_client.get_latest_blockhash()?,
    );

    let sig = rpc_client.send_and_confirm_transaction(&tx)?;
    Ok(sig)
}
