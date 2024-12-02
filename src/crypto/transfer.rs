use solana_sdk::signature::Signer;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;


pub fn solana_token_transfer(private_key: &str, recipient: &str, amount: u64) -> String {
    let keypair = Keypair::from_base58_string(private_key); 
    let recipient_pubkey = Pubkey::from_str(recipient).unwrap();

    return "".to_string()
}
