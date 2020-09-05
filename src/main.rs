use anyhow::Result;
use std::fs;
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
    dest: Option<&'a Path>,
    /// excluded paths
    exclude: Vec<&'a Path>,
    /// replacement text
    replace: Option<&'a str>,
}

fn main() -> Result<()> {
    let contents: String = fs::read_to_string(SCAFFEX_FILE)?;
    let config: Config = toml::from_str(&contents)?;
    
    println!("config: {:?}", config);
    
    Ok(())
}
