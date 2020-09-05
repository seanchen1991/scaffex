use anyhow::Result;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use serde::Deserialize;

const SCAFFEX_FILE: &str = ".scaffex.toml";

#[derive(Debug, Deserialize)]
pub struct Config<'a> {
    /// `START` delimiter
    start: &'a str,
    /// `END` delimiter
    end: &'a str,
    /// source path
    src: &'a Path,
    /// destination path
    dest: &'a Path,
    /// excluded paths
    exclude: Vec<&'a Path>,
    /// replacement text
    replace: Option<&'a str>,
}

fn main() -> Result<()> {
    let mut buffer = String::new();
    let config: String = fs::read_to_string(SCAFFEX_FILE)?;
    let config: Config = toml::from_str(&config)?;
    let src_file = File::open(config.src)?;
    let lines = BufReader::new(src_file).lines();

    for line in lines {
        let line = line.unwrap();

        if !line.contains("//Start") {
            buffer.push_str(format!("{}\n", line).as_str());
        }
    }

    let mut dest_file = File::create(config.dest)?;
    dest_file.write_all(buffer.as_bytes())?;
    
    Ok(())
}
