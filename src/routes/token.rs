use std::str::FromStr;

use axum::Json;
use serde::{Deserialize, Serialize};
use solana_sdk::{instruction::Instruction, pubkey::Pubkey};
use spl_token::instruction::{initialize_mint, mint_to};
use spl_token::ID as TOKEN_PROGRAM_ID;

use base64::{engine::general_purpose, Engine as _};

use crate::response::{ApiResponse, success, error};

#[derive(Debug, Deserialize)]
pub struct CreateTokenRequest {
    mintAuthority: String,
    mint: String,
    decimals: u8,
}

#[derive(Serialize)]
pub struct AccountMetaJson {
    pubkey: String,
    is_signer: bool,
    is_writable: bool,
}

#[derive(Serialize)]
pub struct TokenInstructionData {
    program_id: String,
    accounts: Vec<AccountMetaJson>,
    instruction_data: String,
}

pub async fn create_token(Json(payload): Json<CreateTokenRequest>) -> Json<ApiResponse<TokenInstructionData>> {
    let mint_pubkey = match Pubkey::from_str(&payload.mint) {
        Ok(pk) => pk,
        Err(_) => return Json(error("Invalid mint pubkey")),
    };

    let authority_pubkey = match Pubkey::from_str(&payload.mintAuthority) {
        Ok(pk) => pk,
        Err(_) => return Json(error("Invalid mint authority pubkey")),
    };

    let ix = match initialize_mint(
        &TOKEN_PROGRAM_ID,
        &mint_pubkey,
        &authority_pubkey,
        None,
        payload.decimals,
    ) {
        Ok(i) => i,
        Err(_) => return Json(error("Failed to create mint instruction")),
    };

    let instruction_data = general_purpose::STANDARD.encode(ix.data);

    let accounts = ix.accounts.iter().map(|a| AccountMetaJson {
        pubkey: a.pubkey.to_string(),
        is_signer: a.is_signer,
        is_writable: a.is_writable,
    }).collect();

    Json(success(TokenInstructionData {
        program_id: ix.program_id.to_string(),
        accounts,
        instruction_data,
    }))
}

#[derive(Debug, Deserialize)]
pub struct MintTokenRequest {
    mint: String,
    destination: String,
    authority: String,
    amount: u64,
}

#[derive(Serialize)]
pub struct MintTokenData {
    program_id: String,
    accounts: Vec<AccountMetaJson>,
    instruction_data: String,
}

pub async fn mint_token(Json(payload): Json<MintTokenRequest>) -> Json<ApiResponse<MintTokenData>> {
    let mint = match Pubkey::from_str(&payload.mint) {
        Ok(pk) => pk,
        Err(_) => return Json(error("Invalid mint address")),
    };

    let destination = match Pubkey::from_str(&payload.destination) {
        Ok(pk) => pk,
        Err(_) => return Json(error("Invalid destination address")),
    };

    let authority = match Pubkey::from_str(&payload.authority) {
        Ok(pk) => pk,
        Err(_) => return Json(error("Invalid authority address")),
    };

    let ix = match mint_to(
        &TOKEN_PROGRAM_ID,
        &mint,
        &destination,
        &authority,
        &[],
        payload.amount,
    ) {
        Ok(ix) => ix,
        Err(_) => return Json(error("Failed to build mint instruction")),
    };

    let encoded_data = general_purpose::STANDARD.encode(ix.data);

    let accounts: Vec<AccountMetaJson> = ix.accounts.into_iter().map(|a| AccountMetaJson {
        pubkey: a.pubkey.to_string(),
        is_signer: a.is_signer,
        is_writable: a.is_writable,
    }).collect();

    Json(success(MintTokenData {
        program_id: ix.program_id.to_string(),
        accounts,
        instruction_data: encoded_data,
    }))
}
