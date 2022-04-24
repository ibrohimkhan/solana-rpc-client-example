use solana_client::rpc_client::RpcClient;
use rust_client::*;
use solana_sdk::signer::Signer;

const URL: &str = "https://api.devnet.solana.com";
fn main() {
    let rpc_client = RpcClient::new(URL);

    let sender = create_keypair();
    let receiver = create_keypair();

    println!("Sender: {:?}", sender.pubkey());
    println!("Receiver: {:?}", receiver.pubkey());

    if let Ok(airdrop_signature) = request_airdrop(&rpc_client, &sender.pubkey(), 2.0) {
        println!("Airdrop finished! Signature: {:?}", airdrop_signature);

        if let Ok(balance) = check_balance(&rpc_client, &sender.pubkey()) {
            println!("Sender balance: {}", balance);
        }

        let transfer_amount = 0.5;

        match transfer_funds(&rpc_client, &sender, &receiver.pubkey(), transfer_amount) {
            Ok(sig) => {
                println!("Transfer of {} finished. Signature: {:?}", transfer_amount, sig);
                if let Ok(balance) = check_balance(&rpc_client, &sender.pubkey()) {
                    println!("Sender balance: {}", balance);
                }

                if let Ok(balance) = check_balance(&rpc_client, &receiver.pubkey()) {
                    println!("Receiver balance: {}", balance);
                }
            },
            Err(err) => {
                println!("Error: {:?}", err);
            }
        }
    }
}
