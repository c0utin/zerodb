mod core;

use crate::core::disk;

use color_eyre::{Result, eyre::Context};

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut db_file = disk::DbFile::new("data/hello.zkdb")?;

    let content_to_write = "poggers1.\n"; // Added newline for clarity
    db_file.write_at_end(content_to_write.as_bytes())
        .context("Failed to write content to db_file")?;

    let read_content = db_file.read_all()?;
    println!("Content of the file:\n{}", read_content);

    Ok(())
}
