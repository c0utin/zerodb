use std::fs::File;
use std::io::Read;
use color_eyre::{Result, eyre::Context};
use crate::core::file::{read_and_validate_header, validate_header};

pub fn read_database() -> Result<()> {
    let mut file = File::open("data/data.zkdb")
        .context("Failed to open file")?;
    let header = read_and_validate_header(&mut file)
        .context("Failed to validate ZKDB file")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .context("Failed to read file contents")?;
    println!("File content: {}", contents);
    Ok(())
}

