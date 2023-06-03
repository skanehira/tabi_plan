mod config;

use axum::{routing::post, Router};
use clap::Parser as _;
use std::sync::Arc;
use tokio::signal;
mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = config::AppConfig::try_parse()?;

    let app = Router::new().route("/routes", post(routes::handler));

    let app = app.with_state(Arc::new(config));

    eprintln!("server start");
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}
