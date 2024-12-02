use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
    pubkey::Pubkey
};
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use spl_token::instruction::transfer;
use std::str::FromStr;
use spl_associated_token_account::get_associated_token_address;


pub fn transfer_solana_token(
    sender_keypair: String, 
    receiver_pubkey: String, 
    token_mint: String, 
    amount: u64
) {
    log::debug!("Transfer Solana token");

    // Initialize the RPC client
    let rpc_url = "https://mainnet.helius-rpc.com/?api-key=2d6c544c-8fc7-4bac-9352-a60a7bb2a391";
    let commitment_config = CommitmentConfig::confirmed();
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), commitment_config);

    // Initialize accounts needed for the transfer
    let sender_keypair: Keypair = Keypair::from_base58_string(&sender_keypair);
    let receiver_pubkey: Pubkey = Pubkey::from_str(&receiver_pubkey).expect("Invalid receiver address");
    let token_mint: Pubkey = Pubkey::from_str(&token_mint).expect("Invalid token mint address");

    let sender_token_account = get_associated_token_address(
        &sender_keypair.pubkey(),
        &token_mint,
    );

    let receiver_token_account = get_associated_token_address(
        &receiver_pubkey,
        &token_mint,
    );

    let amount_to_transfer: u64 = amount;

    // Create the transfer instruction
    let transfer_ix = transfer(
        &spl_token::id(),
        &sender_token_account,
        &receiver_token_account,
        &sender_keypair.pubkey(),
        &[],
        amount_to_transfer,
    )
    .expect("Failed to create transfer instruction");

    // Create the transaction
    let mut tx = Transaction::new_with_payer(&[transfer_ix], Some(&sender_keypair.pubkey()));

    // Sign the transaction
    let latest_blockhash = client.get_latest_blockhash().expect("Failed to get blockhash");
    tx.sign(&[&sender_keypair], latest_blockhash);

    // Send the transaction
    let signature = client
        .send_and_confirm_transaction(&tx)
        .expect("Failed to send transaction");

    println!("Transaction successful with signature: {}", signature);
}
