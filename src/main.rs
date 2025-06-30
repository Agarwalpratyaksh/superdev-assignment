use axum::{Router, routing::post};

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

    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    println!("Server running at http://{}/", address);
    axum::serve(listener, app).await.unwrap();
}
