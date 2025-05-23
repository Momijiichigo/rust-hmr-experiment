use axum::{
    routing::{get, get_service},
    Router,
};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{collections::HashSet, fs};
use tower_http::services::ServeDir;

pub mod modify_glue;
pub mod setup;
pub mod recompile_module;
pub mod parser_linking_section;
pub mod modify_wasm;

pub const ERR_MSG_PATH_TO_STR: &str = "Failed to convert path to string";
pub struct Config {
    pub project_dir: PathBuf,
    pub project_name: String,
    /// Optionally specify the target directory for web assets
    pub target_dir: Option<PathBuf>,
    pub src_files: Vec<PathBuf>,
}

