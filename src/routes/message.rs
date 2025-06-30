use axum::{Json, http::StatusCode};
use base64::{engine::general_purpose, Engine as _};
use bs58;
use serde::{Deserialize, Serialize};
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signature, Signer},
};
use std::{convert::TryInto, str::FromStr};

use crate::response::{ApiResponse, success, error};

#[derive(Debug, Deserialize)]
pub struct SignMessageRequest {
    message: String,
    secret: String,
}

#[derive(Serialize)]
pub struct SignMessageData {
    signature: String,
    public_key: String,
    message: String,
}

pub async fn sign_message(
    Json(payload): Json<SignMessageRequest>,
) -> (StatusCode, Json<ApiResponse<SignMessageData>>) {
    if payload.message.is_empty() || payload.secret.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(error("Missing required fields")),
        );
    }

    let decoded = match bs58::decode(&payload.secret).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(error("Invalid base58 secret key")),
            );
        }
    };

    let secret: [u8; 64] = match decoded.as_slice().try_into() {
        Ok(arr) => arr,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(error("Invalid key length")),
            );
        }
    };

    let keypair = match Keypair::from_bytes(&secret) {
        Ok(kp) => kp,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(error("Failed to parse keypair")),
            );
        }
    };

    let signature = keypair.sign_message(payload.message.as_bytes());
    let signature_b64 = general_purpose::STANDARD.encode(signature.as_ref());

    (
        StatusCode::OK,
        Json(success(SignMessageData {
            signature: signature_b64,
            public_key: keypair.pubkey().to_string(),
            message: payload.message,
        })),
    )
}

#[derive(Debug, Deserialize)]
pub struct VerifyMessageRequest {
    message: String,
    signature: String,
    pubkey: String,
}

#[derive(Serialize)]
pub struct VerifyMessageData {
    valid: bool,
    message: String,
    pubkey: String,
}

pub async fn verify_message(
    Json(payload): Json<VerifyMessageRequest>,
) -> (StatusCode, Json<ApiResponse<VerifyMessageData>>) {
    if payload.message.is_empty() || payload.signature.is_empty() || payload.pubkey.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(error("Missing required fields")),
        );
    }

    let pubkey = match Pubkey::from_str(&payload.pubkey) {
        Ok(pk) => pk,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(error("Invalid base58 public key")),
            );
        }
    };

    let signature_bytes = match general_purpose::STANDARD.decode(&payload.signature) {
        Ok(bytes) => bytes,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(error("Invalid base64 signature")),
            );
        }
    };

    let signature = match Signature::try_from(signature_bytes.as_slice()) {
        Ok(sig) => sig,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(error("Invalid signature format")),
            );
        }
    };

    let valid = signature.verify(pubkey.as_ref(), payload.message.as_bytes());

    (
        StatusCode::OK,
        Json(success(VerifyMessageData {
            valid,
            message: payload.message,
            pubkey: payload.pubkey,
        })),
    )
}