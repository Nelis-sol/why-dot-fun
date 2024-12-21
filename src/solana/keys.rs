use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signature::Signer;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::transaction::Transaction;
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use crate::secrets::Secrets;
use solana_client::rpc_config::RpcSendTransactionConfig;
use solana_sdk::instruction::Instruction;
use solana_sdk::compute_budget::ComputeBudgetInstruction;

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

pub async fn get_or_create_ata(
    payer: &Keypair,
    wallet_address: &Pubkey,
    token_mint_address: &Pubkey,
    token_program_id: &Pubkey,
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
    let create_ata_ix = spl_associated_token_account::instruction::create_associated_token_account(
        &payer.pubkey(),
        &wallet_address,
        &token_mint_address,
        &token_program_id,
    );

    let modify_compute_units = ComputeBudgetInstruction::set_compute_unit_limit(30000);
    let set_priority_fee = ComputeBudgetInstruction::set_compute_unit_price(1000);

    let latest_blockhash = rpc_client.get_latest_blockhash()?;

    let mut transaction = Transaction::new_signed_with_payer(
        &[create_ata_ix, modify_compute_units, set_priority_fee], 
        Some(&payer.pubkey()),
        &[payer],
        latest_blockhash
    );





    let signature = rpc_client.send_and_confirm_transaction_with_spinner(&transaction)?;

    println!("signature: {}", signature.to_string());

    // Return the ATA address after confirming the transaction
    Ok(ata_address)
}



// funding_address,
// wallet_address,
// token_mint_address,
// token_program_id,
