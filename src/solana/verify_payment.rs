use solana_sdk::transaction::Transaction;
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signature::Signature;
use crate::Secrets;


pub async fn transfer_solana_token(
    secrets: &Secrets,
    transaction: Transaction,
) -> Result<Signature, Box<dyn std::error::Error>> {

    // Initialize the RPC client
    let commitment_config = CommitmentConfig::confirmed();
    let rpc_client = RpcClient::new_with_commitment(&secrets.rpc_url, commitment_config);

    let signature = rpc_client.send_and_confirm_transaction_with_spinner(&transaction)?;

    return Ok(signature);

}
