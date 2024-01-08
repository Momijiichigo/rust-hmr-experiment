use std::fs::{self, File};
use std::io::{self, Write, BufReader, BufRead};
use anyhow::Context;

use crate::Config;

pub fn modify_glue_js(config: &Config) -> anyhow::Result<()> {
    let project_name = config.project_dir
        .file_name()
        .context("Project dir not set")?
        .to_str()
        .context("Failed to convert project dir to string")?
        .replace('-', "_");
    let js_glue_file_path = config.target_dir
        .to_owned()
        .context("Target dir not set")?
        .join("web-assets")
        .join("pkg")
        .join(format!("{project_name}.js")); // Path to the generated JS glue file
    let temp_file_path = config.target_dir
        .to_owned()
        .context("Target dir not set")?
        .join("web-assets")
        .join("pkg")
        .join(format!("{project_name}.js.temp")); // Path to the temporary file

    println!("js_glue_file_path: {}", js_glue_file_path.to_str().unwrap());
    // Open the file for reading
    let file = File::open(&js_glue_file_path)?;
    let reader = BufReader::new(file);

    // Open a temporary file for writing
    let mut temp_file = File::create(&temp_file_path)?;

    let mod_funcs = [
        globalize_get_imports,
        globalize_stack_pointer
    ];
    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        let mut result_line = ModLine::NoChange;

        for func in mod_funcs.iter() {
            result_line = func(line);
            if let ModLine::Change(_) = result_line {
                break;
            }
        }
        match result_line {
            ModLine::Change(line) => {
                writeln!(temp_file, "{}", line)?;
            }
            ModLine::NoChange => {
                writeln!(temp_file, "{}", line)?;
            }
        }
    }

    // Replace the original file with the modified file
    fs::rename(temp_file_path, js_glue_file_path)?;

    Ok(())
}
enum ModLine {
    Change(String),
    NoChange,
}
fn globalize_get_imports(line: &str) -> ModLine {
    if line == "function __wbg_get_imports() {" {
        ModLine::Change("window.__wbg_get_imports = function() {".to_string())
    } else {
        ModLine::NoChange
    }
}
fn globalize_stack_pointer(line: &str) -> ModLine {
    if line == "let heap_next = heap.length;" {
        ModLine::Change("window.heap_next = heap.length;".to_string())
    } else {
        ModLine::NoChange
    }
}
