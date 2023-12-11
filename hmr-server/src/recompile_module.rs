use axum::{
    routing::{get, get_service},
    Router, Server,
};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{collections::HashSet, fs};
use tower_http::services::ServeDir;

use crate::*;

#[cfg(not(feature = "debug-compilation-wasm-pack"))]
static WASM_PACK_COMPILATION_MODE: &str = "release";
#[cfg(feature = "debug-compilation-wasm-pack")]
static WASM_PACK_COMPILATION_MODE: &str = "debug";

pub async fn recompile_module(
    config: &Config,
    mod_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut library_paths = HashSet::<String>::new();
    let mut library_names = HashSet::<String>::new();
    let target_dir = &config.target_dir.to_owned().ok_or("Target dir not set")?;
    if let Ok(entries) = fs::read_dir(
        target_dir
            .join("wasm32-unknown-unknown")
            .join(WASM_PACK_COMPILATION_MODE)
            .join("deps"),
    ) {
        for entry in entries {
            let entry = entry?.path();
            if entry.extension().unwrap() == "d" {
                // read file
                let file = fs::read_to_string(entry)?;

                file.split(&[' ', '\n']).for_each(|line| {
                    if line.contains(".cargo/registry") {
                        let crate_path = PathBuf::from(line);
                        // strip `/src/...` from the path
                        let mut crate_path = crate_path.as_path();
                        loop {
                            if crate_path.ends_with("src") {
                                break;
                            }
                            crate_path = crate_path.parent().unwrap();
                        }
                        let crate_path = crate_path.parent().unwrap();
                        // strip version at the end of the name
                        let crate_name = crate_path
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .split('-')
                            .filter(|s| s.chars().next().unwrap().is_alphabetic())
                            .collect::<Vec<&str>>()
                            .join("_");
                        library_names.insert(crate_name.clone());
                        library_paths.insert(format!("{}/src", crate_path.to_str().unwrap()));
                        // library_paths.insert(format!(
                        //     "{}={}",
                        //     crate_name,
                        //     crate_path.to_str().unwrap().to_string(),
                        // ));
                    }
                });
            }
        }
    }
    let output_name = target_dir
        .join("web-assets")
        .join("wasm")
        .join(mod_path)
        .with_extension("wasm");
    let dep_wasm_path = target_dir
        .join("wasm32-unknown-unknown")
        .join(WASM_PACK_COMPILATION_MODE)
        .join("deps");
    let dep_release_path = target_dir.join(WASM_PACK_COMPILATION_MODE).join("deps");
    let input_name = config.project_dir.join("src").join(mod_path);
    let mut args = vec![
        "--edition",
        "2021",
        "--target",
        "wasm32-unknown-unknown",
        "-C",
        "opt-level=1",
        "--crate-type",
        "cdylib",

        // "--emit",
        // "obj",

        "-L",
        dep_wasm_path.to_str().ok_or(ERR_MSG_PATH_TO_STR)?,
        "-L",
        dep_release_path.to_str().ok_or(ERR_MSG_PATH_TO_STR)?,
        "-o",
        output_name.to_str().ok_or(ERR_MSG_PATH_TO_STR)?,
        input_name.to_str().ok_or(ERR_MSG_PATH_TO_STR)?,
    ];
    // library_paths.iter().for_each(|path| {
    //     args.push("-L");
    //     args.push(path);
    // });
    library_names.iter().for_each(|path| {
        args.push("--extern");
        args.push(path);
    });

    println!("args: {:?}", args.join(" "));

    // compile one module into wasm module using rustc
    Command::new("rustc").args(args).status()?;

    Ok(())
}
