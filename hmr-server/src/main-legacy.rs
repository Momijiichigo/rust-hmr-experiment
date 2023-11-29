use axum::ServiceExt;
use axum::{
    routing::get,
    Router,
};
use tower_http::services::{ServeDir, ServeFile};
use std::path::PathBuf;
use std::process::{Command, Stdio};

use anyhow::Result;
#[tokio::main]
async fn main() {
    let config = Config::new(
        "../example/src".into(),
        "../target/web_asset".into(),
    ).unwrap();
    // for path in &config.src_files {
    //     compile_module(config.clone(), path).unwrap();
    //     println!("compile: {:?}", path);
    // }
    start_server(&config).await;
}
pub async fn start_server(config: &Config) {
    println!("serving files from {}", config.target_path.display());
    let serve_dir = ServeDir::new(config.target_path.clone());
    let router = Router::new()
        .nest_service("/", serve_dir);
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}

#[derive(Clone)]
pub struct Config {
    target_path: PathBuf,
    target_wasm_path: PathBuf,
    src_files: Vec<PathBuf>,
}
impl Config {
    pub fn new(src_path: PathBuf, target_path: PathBuf) -> Result<Self> {
        let target_wasm_path = target_path.join("wasm");
        std::fs::create_dir_all(&target_wasm_path)?;
        let mut config = Config {
            target_path,
            target_wasm_path,
            src_files: Vec::new(),
        };
        get_src_files(&mut config, src_path)?;
        Ok(config)
    }
}
fn get_src_files(config: &mut Config, path: PathBuf) -> Result<()> {
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            get_src_files(config, path)?;
        } else if path.extension().map(|e| e == "rs").unwrap_or(false) {
            config.src_files.push(path);
        }
    }
    Ok(())
}

pub fn compile_module(config: Config, path: &PathBuf) -> Result<()> {
    
    let mut child = Command::new("rustc")
        .arg(path)
        .arg("--target=wasm32-unknown-unknown")
        .arg(format!("--out-dir={}", config.target_wasm_path.display()))
        .arg("-C")
        .arg("opt-level=1")
        .stderr(Stdio::piped())
        .spawn()?;
    // printout command used
    println!("{:?}", child);

    if child.wait()?.success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Failed to compile"))
    }
}
