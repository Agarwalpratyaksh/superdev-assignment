use axum::{Json, http::StatusCode};
use serde::Serialize;
use solana_sdk::signature::{Keypair, Signer};
use bs58;

use crate::response::{ApiResponse, success};

#[derive(Serialize)]
pub struct KeypairData {
    pub pubkey: String,
    pub secret: String,
}

pub async fn generate_keypair() -> (StatusCode, Json<ApiResponse<KeypairData>>) {
    let keypair = Keypair::new();
    let pubkey = keypair.pubkey().to_string();
    let secret = bs58::encode(keypair.to_bytes()).into_string();

    (
        StatusCode::OK,
        Json(success(KeypairData { pubkey, secret })),
    )
}