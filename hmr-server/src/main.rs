use axum::{
    routing::{get, get_service},
    Router, Server,
};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{collections::HashSet, fs};
use tower_http::services::ServeDir;

use hmr_server::{Config, ERR_MSG_PATH_TO_STR, setup::*, modify_glue::*, recompile_module::*};


async fn serve(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let target_dir = &config.target_dir.to_owned().ok_or("Target dir not set")?;

    let target_web_assets_dir = target_dir.join("web-assets");
    // Set up router

    // Start server
    Server::bind(&"127.0.0.1:3000".parse()?)
        .serve(tower::make::Shared::new(ServeDir::new(
            &target_web_assets_dir,
        )))
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let mut config = Config {
        project_dir: PathBuf::from("../wasm-project"),
        target_dir: Some(PathBuf::from("../target")),
        src_files: vec![PathBuf::from("mod1.rs")],
    };
    println!("=== Setup ===");
    setup(&mut config).await.unwrap();
    println!("=== Recompile module ===");
    recompile_module(&config, &config.src_files[0])
        .await
        .unwrap();
    println!("=== Serve ===");
    serve(&config).await.unwrap();
}

