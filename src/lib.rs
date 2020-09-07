#![feature(split_inclusive)]

use anyhow::Result;
use ropey::{self, Rope};
use serde::Deserialize;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter};
use std::path::Path;

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
    let src = File::open(config.src)?;
    let mut dest_file = ropey::Rope::from_reader(
        File::open(config.src)?
    )?;
    
    let delimiters = find_delimiters(src, &config);
    replace_text(&mut dest_file, delimiters, &config);
    
    dest_file.write_to(
        BufWriter::new(File::create(config.dest)?)
    )?;
    
    Ok(())
}

fn replace_text(dest: &mut Rope, delimiters: Vec<(usize, usize)>, config: &Config) {
    for (start, end) in delimiters.into_iter() {
        dest.remove(start..end);

        if let Some(replace_with) = config.replace {
            dest.insert(start, replace_with);
        } else {
            dest.insert(start, "\n");
        }
    } 
}

fn find_delimiters(src: File, config: &Config) -> Vec<(usize, usize)>{
    // Find all `START` and `END` delimiters, storing their indices
    let mut start_positions: Vec<usize> = vec![];
    let mut end_positions: Vec<usize> = vec![];
    let mut total_chars: usize = 0;

    for line in BufReader::new(src).lines() {
        let line = line.unwrap();
        let mut token_pos: usize = 0;

        for token in line.split_inclusive(' ') {
            if token.starts_with(config.start) {
               start_positions.push(total_chars + token_pos);
            } else if token.starts_with(config.end) {
                println!("Config End Length: {}", config.end.len());
                end_positions.push(total_chars + token_pos + config.end.len());
            }

            token_pos += token.len();
        }

        total_chars += line.len();
    }

    start_positions.iter().zip(end_positions.iter())
        .map(|(s, e)| (*s, *e))
        .collect()
}
