use crate::{modify_wasm::demangle_imports, parser_linking_section::SymbolInfo};
use anyhow::{Context, Error};
use axum::{
    routing::{get, get_service},
    Router, Server,
};
use nom::AsBytes;
use std::process::Command;
use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};
use std::{collections::HashSet, fs};
use tower_http::services::ServeDir;
use walrus::{IdsToIndices, Module};

use crate::{
    modify_wasm::{demangle_funcs, module_from_bytes},
    parser_linking_section::take_linking_section,
    *,
};

#[cfg(not(feature = "debug-compilation-wasm-pack"))]
static WASM_PACK_COMPILATION_MODE: &str = "release";
#[cfg(feature = "debug-compilation-wasm-pack")]
static WASM_PACK_COMPILATION_MODE: &str = "debug";

pub async fn recompile_module(config: &Config, mod_path: &Path) -> anyhow::Result<()> {
    let mut library_paths = HashSet::<String>::new();
    let mut library_names = HashSet::<String>::new();
    let target_dir = &config.target_dir.to_owned().context("Target dir not set")?;
    if let Ok(entries) = fs::read_dir(
        target_dir
            .join("wasm32-unknown-unknown")
            .join(WASM_PACK_COMPILATION_MODE)
            .join("deps"),
    ) {
        for entry in entries {
            let entry = entry?.path();
            if entry.extension().context("no file extension found")? == "d" {
                // read file
                let file = fs::read_to_string(entry)?;

                for line in file.split(&[' ', '\n']) {
                    if line.contains(".cargo/registry") {
                        let crate_path = PathBuf::from(line);
                        // strip `/src/...` from the path
                        let mut crate_path = crate_path.as_path();
                        loop {
                            if crate_path.ends_with("src") {
                                break;
                            }
                            crate_path =
                                crate_path.parent().context("could not get parent path")?;
                        }
                        let crate_path =
                            crate_path.parent().context("could not get parent path")?;
                        // strip version at the end of the name
                        let crate_name = crate_path
                            .file_name()
                            .context("could not get file name")?
                            .to_str()
                            .context("could not convert to str")?
                            .split('-')
                            .filter(|s| s.chars().next().unwrap().is_alphabetic())
                            .collect::<Vec<&str>>()
                            .join("_");
                        library_names.insert(crate_name.clone());
                        library_paths.insert(format!(
                            "{}/src",
                            crate_path.to_str().context("could not convert to str")?
                        ));
                    }
                }
            }
        }
    }
    let output_name = target_dir
        .join("web-assets")
        .join("wasm")
        .join(mod_path)
        .with_extension("obj.wasm");
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
        "--emit",
        "obj",
        //--- experimenting --
        "--cfg",
        "feature=\"leptos?/csr\"",
        // ------
        "-L",
        dep_wasm_path.to_str().context(ERR_MSG_PATH_TO_STR)?,
        "-L",
        dep_release_path.to_str().context(ERR_MSG_PATH_TO_STR)?,
        "-o",
        output_name.to_str().context(ERR_MSG_PATH_TO_STR)?,
        input_name.to_str().context(ERR_MSG_PATH_TO_STR)?,
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

    let mut module =
        module_from_bytes(&fs::read(output_name)?).context("failed to parse bytes as wasm")?;
    restore_export(&mut module)?;
    demangle_funcs(&mut module);
    demangle_imports(&mut module);

    let output_name = target_dir
        .join("web-assets")
        .join("wasm")
        .join(mod_path)
        .with_extension("wasm");
    module.emit_wasm_file(output_name)?;

    Ok(())
}

/// modifies and adds the function exports into the wasm object file,
/// where the exports are omitted in the object file.
///
/// It uses the linking custom section to obtain the function names.
fn restore_export(module: &mut Module) -> anyhow::Result<()> {
    let custom = module
        .customs
        .iter()
        .find(|(_, custom)| custom.name() == "linking")
        .context("no linking custom section found in wasm")?
        .1;
    let data = custom.data(&walrus::IdsToIndices::default());

    let (remain_bytes, info_list) = take_linking_section(&data)
        .map_err(|e| e.to_owned())
        .context("parsing linking custom section failed")?;
    println!("remain_bytes: {:?}", remain_bytes);
    println!("info_list: {:?}", info_list);

    let funcs: Vec<_> = info_list
        .into_iter()
        .filter(|syminfo| matches!(syminfo, SymbolInfo::Function(_, Some(_), false)))
        .collect();

    module.funcs.iter_mut().for_each(|func| {
        for syminfo in funcs.iter() {
            if let SymbolInfo::Function(idx, Some(name), false) = syminfo {
                if *idx as usize == func.id().index() {
                    module.exports.add(&name, func.id());
                    break;
                }
            }
        }
    });

    Ok(())
}
