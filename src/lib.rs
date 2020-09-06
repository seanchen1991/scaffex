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

pub fn scaffold() -> Result<()> {
    let config: String = fs::read_to_string(SCAFFEX_FILE)?;
    let config: Config = toml::from_str(&config)?;
    let src_file = File::open(config.src)?;

    let generated = generate(src_file, &config);

    let mut dest_file = File::create(config.dest)?;
    dest_file.write_all(generated.as_bytes())?;
    
    Ok(())
}

fn generate(src: File, config: &Config) -> String {
    let mut discard = false;
    let mut buffer = String::new();
    let mut offset: usize = 0; 

    for line in BufReader::new(src).lines() {
        let line = line.unwrap(); 
        
        if line.contains(config.start) {
            offset = line.chars().take_while(|c| c.is_whitespace()).count(); 
            discard = true;
            continue;
        } else if line.contains(config.end) {
            if let Some(replace) = config.replace {
               buffer.push_str(&" ".repeat(offset).as_str()); 
               buffer.push_str(format!("{}\n", replace).as_str()); 
            } else {
                buffer.push('\n');
            }
            
            discard = false;
            continue;
        }

        if !discard {
            buffer.push_str(format!("{}\n", line).as_str());
        }
    }

    buffer
}
