use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signature::Signer;
use spl_associated_token_account::instruction::create_associated_token_account_idempotent;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::transaction::Transaction;
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use crate::secrets::Secrets;

pub fn generate_private_key() -> Keypair {
    log::debug!("Generate new Solana keypair");

    let keypair = Keypair::new();
    return keypair
}

pub fn generate_private_key_base58() -> String {
    log::debug!("Generate new Solana keypair");

    let keypair = Keypair::new();
    return keypair.to_base58_string()
}

pub fn derive_public_key_from_private_key(private_key: &str) -> String {
    log::debug!("Derive Solana public key from private key");

    let keypair = Keypair::from_base58_string(private_key);
    return keypair.pubkey().to_string()
}

pub fn get_or_create_ata(
    funding_address: &Pubkey, 
    wallet_address: &Pubkey,
    token_mint_address: &Pubkey,
    token_program_id: &Pubkey,
    payer: &Keypair,
    rpc_url: String,
) -> Result<Pubkey, Box<dyn std::error::Error>> {
    
    let rpc_client = RpcClient::new(rpc_url);

    // Check if the associated token account already exists
    let ata_address = spl_associated_token_account::get_associated_token_address(
        &wallet_address,
        &token_mint_address,
    );

    if rpc_client.get_account(&ata_address).is_ok() {
        log::debug!("ATA already exists: {}", ata_address);
        return Ok(ata_address);
    }

    // Create the associated token account if it doesn't exist
    let ix = spl_associated_token_account::instruction::create_associated_token_account_idempotent(
        &funding_address,
        &wallet_address,
        &token_mint_address,
        &token_program_id,
    );

    let mut transaction = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));

    let latest_blockhash = rpc_client.get_latest_blockhash()?;

    transaction.sign(&[payer], latest_blockhash);

    rpc_client.send_and_confirm_transaction(&transaction)?;

    // Return the ATA address after confirming the transaction
    Ok(ata_address)
}



// funding_address,
// wallet_address,
// token_mint_address,
// token_program_id,
