use axum::{Router, routing::post, http::StatusCode};
use std::env;

mod routes;
mod response;

use routes::keypair::generate_keypair;
use routes::message::{sign_message, verify_message};
use routes::send::{send_sol, send_token};
use routes::token::{create_token, mint_token};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/keypair", post(generate_keypair))
        .route("/token/create", post(create_token))
        .route("/token/mint", post(mint_token))
        .route("/message/sign", post(sign_message))
        .route("/message/verify", post(verify_message))
        .route("/send/sol", post(send_sol))
        .route("/send/token", post(send_token));

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("Server running at http://{}/", addr);
    axum::serve(listener, app).await.unwrap();
}