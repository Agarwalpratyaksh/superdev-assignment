use axum::Json;
use serde::Serialize;
use solana_sdk::signature::{Keypair, Signer};
use bs58;

use crate::response::{ApiResponse, success, error}; // ✅ Import helpers

#[derive(Serialize)]
pub struct KeypairData {
    pub pubkey: String,
    pub secret: String,
}

pub async fn generate_keypair() -> Json<ApiResponse<KeypairData>> {
    let keypair = Keypair::new();
    let pubkey = keypair.pubkey().to_string();
    let secret = bs58::encode(keypair.to_bytes()).into_string();

    Json(success(KeypairData { pubkey, secret }))
}
