use axum::response::IntoResponse;
use axum::Json;
use axum::Extension;
use crate::StatusCode;
use serde::{Serialize, Deserialize};
use crate::secrets::Secrets;
use crate::solana::generate_payment::generate_payment;
use base64::{engine::general_purpose, Engine as _};
use bincode;


#[derive(Serialize, Deserialize)]
pub struct DepositArgs {
    pub sender: String,
    pub public_key: String,
    pub amount: u64,
}

#[axum::debug_handler]
pub async fn deposit(
    secrets: Extension<Secrets>,
    Json(payment_args): Json<DepositArgs>
) -> impl IntoResponse {

    let sender = payment_args.sender;
    let public_key = payment_args.public_key;
    let amount = payment_args.amount;

    let transaction = generate_payment(
        &secrets,
        sender, 
        amount
    ).await
    .unwrap();

    let serialized_transaction = bincode::serialize(&transaction).expect("Failed to serialize transaction");
    let encoded_transaction = general_purpose::STANDARD.encode(serialized_transaction);

    let response = (
        StatusCode::OK, 
        Json(encoded_transaction)
    ).into_response();

    response

}