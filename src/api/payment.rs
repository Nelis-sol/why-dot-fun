use axum::response::IntoResponse;
use axum::Json;
use axum::Extension;
use crate::StatusCode;
use serde::{Serialize, Deserialize};
use crate::secrets::Secrets;
use crate::solana::generate_payment::generate_payment;


#[derive(Serialize, Deserialize)]
pub struct PaymentArgs {
    pub sender: String,
    pub amount: u64,
}

#[axum::debug_handler]
pub async fn payment(
    secrets: Extension<Secrets>,
    Json(payment_args): Json<PaymentArgs>
) -> impl IntoResponse {

    let sender = payment_args.sender;
    let amount = payment_args.amount;

    let transaction = generate_payment(
        &secrets,
        sender, 
        amount
    ).await
    .unwrap();

    let response = (
        StatusCode::OK, 
        Json(transaction)
    ).into_response();

    response

}