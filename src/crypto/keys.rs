use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signature::Signer;

pub fn generate_solana_private_key() -> String {
    log::debug!("Generate new Solana keypair");

    let keypair = Keypair::new();
    return keypair.to_base58_string()
}

pub fn derive_solana_public_key(private_key: &str) -> String {
    log::debug!("Derive Solana public key from private key");

    let keypair = Keypair::from_base58_string(private_key);
    return keypair.pubkey().to_string()
}
