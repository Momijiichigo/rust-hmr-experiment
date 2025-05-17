use anyhow::Context;
use axum::{
    routing::{get, get_service},
    Router,
};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{collections::HashSet, fs};
use tower_http::services::{ServeDir, ServeFile};

use hmr_server::{modify_glue::*, recompile_module::*, setup::*, Config, ERR_MSG_PATH_TO_STR};

async fn not_found_handler() -> axum::response::Response {
    // Handle 404
    axum::response::Response::new(axum::body::Body::from("404 Not Found"))
}

async fn serve(config: &Config) -> anyhow::Result<()> {
    let target_dir = &config.target_dir.to_owned().context("Target dir not set")?;


    let target_web_assets_dir = target_dir.join("web-assets");
    // Set up router

    // Start server
    axum::serve(
        tokio::net::TcpListener::bind("127.0.0.1:3000").await?,
        Router::new()
            .route("/", get_service(ServeFile::new(
                target_web_assets_dir.join("index.html"),
            )))
            .fallback_service(
                ServeDir::new(&target_web_assets_dir)
            )
            // nest_service("/", get_service(ServeDir::new(&target_web_assets_dir))),
    )
    .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut config = Config {
        project_dir: PathBuf::from("..").join("wasm-project"),
        project_name: "wasm_project".to_string(),
        target_dir: Some(PathBuf::from("..").join("target")),
        src_files: vec![PathBuf::from("mod1.rs")],
    };
    println!("=== Setup ===");
    setup(&mut config).await.context("setup failed")?;
    println!("=== Recompile module ===");
    recompile_module(&config, &config.src_files[0])
        .await
        .context("recompile module failed")?;
    println!("=== Serve to localhost:3000 ===");
    serve(&config).await.context("serve failed")?;
    Ok(())
}
