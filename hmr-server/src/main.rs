use anyhow::Context;
use axum::{
    routing::{get, get_service},
    Router, Server,
};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{collections::HashSet, fs};
use tower_http::services::ServeDir;

use hmr_server::{modify_glue::*, recompile_module::*, setup::*, Config, ERR_MSG_PATH_TO_STR};

async fn serve(config: &Config) -> anyhow::Result<()> {
    let target_dir = &config.target_dir.to_owned().context("Target dir not set")?;

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
async fn main() -> anyhow::Result<()> {
    let mut config = Config {
        project_dir: PathBuf::from("../wasm-project"),
        project_name: "wasm_project".to_string(),
        target_dir: Some(PathBuf::from("../target")),
        src_files: vec![PathBuf::from("mod1.rs")],
    };
    println!("=== Setup ===");
    setup(&mut config).await.context("setup failed")?;
    println!("=== Recompile module ===");
    recompile_module(&config, &config.src_files[0])
        .await
        .context("recompile module failed")?;
    println!("=== Serve ===");
    serve(&config).await.context("serve failed")?;
    Ok(())
}
