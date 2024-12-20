use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
    pubkey::Pubkey
};
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use spl_token::instruction::transfer;
use std::str::FromStr;
use crate::solana::keys::get_or_create_ata;


pub fn transfer_solana_token(
    rpc_url: String,
    sender_private_key: String, 
    receiver_pubkey: Pubkey, 
    token_mint: String,
    amount: u64
) -> Result<(), Box<dyn std::error::Error>> {
    log::debug!("Transfer Solana token");

    // Initialize the RPC client
    let commitment_config = CommitmentConfig::confirmed();
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), commitment_config);

    // Initialize accounts needed for the transfer
    println!("sender_private_key: {}", sender_private_key);
    log::info!("sender_private_key: {}", sender_private_key);
    let sender_keypair: Keypair = Keypair::from_base58_string(&sender_private_key);
    println!("sender_keypair: {:?}", sender_keypair.secret());
    log::info!("sender_keypair: {:?}", sender_keypair.secret());

    let token_mint: Pubkey = Pubkey::from_str(&token_mint).expect("Invalid token mint address");

    let account_info = client.get_account(&token_mint).expect("Failed to fetch account info for token mint");
    let token_program_id = account_info.owner;

    let sender_token_account = get_or_create_ata(
        &sender_keypair.pubkey(), 
        &sender_keypair.pubkey(),
        &token_mint,
        &token_program_id,
        &sender_keypair,
        rpc_url.clone()
    ).expect("Failed to get or create sender token account");

    let receiver_token_account = get_or_create_ata(
        &sender_keypair.pubkey(), 
        &receiver_pubkey,
        &token_mint,
        &token_program_id,
        &sender_keypair,
        rpc_url.clone()
    ).expect("Failed to get or create receiver token account");


    let amount_to_transfer: u64 = amount;

    // Create the transfer instruction
    let transfer_ix = transfer(
        &spl_token::id(),
        &sender_token_account,
        &receiver_token_account,
        &sender_keypair.pubkey(),
        &[&sender_keypair.pubkey()],
        amount_to_transfer,
    )
    .expect("Failed to create transfer instruction");

    // Create the transaction
    let mut tx = Transaction::new_with_payer(&[transfer_ix], Some(&sender_keypair.pubkey()));

    // Sign the transaction
    let latest_blockhash = client.get_latest_blockhash().expect("Failed to get blockhash");
    tx.sign(&[&sender_keypair], latest_blockhash);

    // Send the transaction
    let _signature = client
        .send_and_confirm_transaction(&tx)
        .expect("Failed to send transaction");

    Ok(())
}
