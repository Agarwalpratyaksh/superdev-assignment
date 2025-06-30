use crate::response::{ApiResponse, error, success};
use axum::{Json, http::StatusCode};
use base64::{Engine as _, engine::general_purpose};
use serde::{Deserialize, Serialize};
use solana_sdk::instruction::AccountMeta;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey, system_instruction};
use spl_token::ID as TOKEN_PROGRAM_ID;
use spl_token::instruction::transfer;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub struct SendSolRequest {
    from: String,
    to: String,
    lamports: u64,
}

#[derive(Serialize)]
pub struct SendSolData {
    program_id: String,
    accounts: Vec<String>,
    instruction_data: String,
}

pub async fn send_sol(
    Json(payload): Json<SendSolRequest>,
) -> (StatusCode, Json<ApiResponse<SendSolData>>) {
    let from_pubkey = match Pubkey::from_str(&payload.from) {
        Ok(pk) => pk,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(error::<SendSolData>("Invalid sender address")),
            );
        }
    };

    let to_pubkey = match Pubkey::from_str(&payload.to) {
        Ok(pk) => pk,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(error::<SendSolData>("Invalid recipient address")),
            );
        }
    };

    if payload.lamports == 0 {
        return (
            StatusCode::BAD_REQUEST,
            Json(error::<SendSolData>("Lamports must be greater than 0")),
        );
    }

    let ix: Instruction = system_instruction::transfer(&from_pubkey, &to_pubkey, payload.lamports);
    let encoded_data = general_purpose::STANDARD.encode(&ix.data);

    (
        StatusCode::OK,
        Json(success(SendSolData {
            program_id: ix.program_id.to_string(),
            accounts: ix.accounts.iter().map(|a| a.pubkey.to_string()).collect(),
            instruction_data: encoded_data,
        })),
    )
}

#[derive(Debug, Deserialize)]
pub struct SendTokenRequest {
    destination: String,
    mint: String,
    owner: String,
    amount: u64,
}

#[derive(Serialize)]
pub struct SendTokenData {
    program_id: String,
    accounts: Vec<SendTokenAccountMeta>,
    instruction_data: String,
}

#[derive(Serialize)]
pub struct SendTokenAccountMeta {
    pubkey: String,
    isSigner: bool,
}

pub async fn send_token(
    Json(payload): Json<SendTokenRequest>,
) -> (StatusCode, Json<ApiResponse<SendTokenData>>) {
    let destination = match Pubkey::from_str(&payload.destination) {
        Ok(pk) => pk,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(error::<SendTokenData>("Invalid destination address")),
            );
        }
    };

    let mint = match Pubkey::from_str(&payload.mint) {
        Ok(pk) => pk,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(error::<SendTokenData>("Invalid mint address")),
            );
        }
    };

    let owner = match Pubkey::from_str(&payload.owner) {
        Ok(pk) => pk,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(error::<SendTokenData>("Invalid owner address")),
            );
        }
    };

    if payload.amount == 0 {
        return (
            StatusCode::BAD_REQUEST,
            Json(error::<SendTokenData>("Amount must be greater than 0")),
        );
    }

    let source_ata = spl_associated_token_account::get_associated_token_address(&owner, &mint);
    let destination_ata =
        spl_associated_token_account::get_associated_token_address(&destination, &mint);

    let ix = match transfer(
        &TOKEN_PROGRAM_ID,
        &source_ata,
        &destination_ata,
        &owner,
        &[],
        payload.amount,
    ) {
        Ok(i) => i,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(error::<SendTokenData>(
                    "Failed to create transfer instruction",
                )),
            );
        }
    };

    let encoded_data = general_purpose::STANDARD.encode(&ix.data);

    let accounts: Vec<SendTokenAccountMeta> = ix
        .accounts
        .into_iter()
        .map(|meta| SendTokenAccountMeta {
            pubkey: meta.pubkey.to_string(),
            isSigner: meta.is_signer,
        })
        .collect();

    (
        StatusCode::OK,
        Json(success(SendTokenData {
            program_id: ix.program_id.to_string(),
            accounts,
            instruction_data: encoded_data,
        })),
    )
}
