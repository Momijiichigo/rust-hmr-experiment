use std::fs::{self, File};
use std::io::{self, Write, BufReader, BufRead};
use crate::Config;

pub fn modify_glue_js(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let project_name = config.project_dir
        .file_name()
        .ok_or("Project dir not set")?
        .to_str()
        .ok_or("Failed to convert project dir to string")?
        .replace('-', "_");
    let js_glue_file_path = config.target_dir
        .to_owned()
        .ok_or("Target dir not set")?
        .join("web-assets")
        .join("pkg")
        .join(format!("{project_name}.js")); // Path to the generated JS glue file
    let temp_file_path = config.target_dir
        .to_owned()
        .ok_or("Target dir not set")?
        .join("web-assets")
        .join("pkg")
        .join(format!("{project_name}.js.temp")); // Path to the temporary file

    println!("js_glue_file_path: {}", js_glue_file_path.to_str().unwrap());
    // Open the file for reading
    let file = File::open(&js_glue_file_path)?;
    let reader = BufReader::new(file);

    // Open a temporary file for writing
    let mut temp_file = File::create(&temp_file_path)?;

    for line in reader.lines() {
        let line = line?;
        if line.trim() == "function __wbg_get_imports() {" {
            // Write the modified line
            writeln!(temp_file, "window.__wbg_get_imports = function() {{")?;
        } else {
            // Write the line as is
            writeln!(temp_file, "{}", line)?;
        }
    }

    // Replace the original file with the modified file
    fs::rename(temp_file_path, js_glue_file_path)?;

    Ok(())
}
