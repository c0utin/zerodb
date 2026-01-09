use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use color_eyre::{Result, eyre::Context};
use crate::core::file;

pub struct DbFile {
    file: File,
}

impl DbFile {
    pub fn new(name: &str) -> Result<Self> {
        file::create(name)?;
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(name)?;

       Ok(Self { file })
    }

    pub fn read_all(&mut self) -> Result<String> {
        self.file.seek(SeekFrom::Start(0))
            .context("Failed to seek to beginning of file")?;

        let mut content = String::new();
        self.file.read_to_string(&mut content)
            .context("Failed to read file content to string")?;
        Ok(content)
    }

    pub fn write_at_end(&mut self, content: &[u8]) -> Result<()> {
        self.file.seek(SeekFrom::End(0))
            .context("Failed to seek to end of file before writing")?;
        self.file.write_all(content)
            .context("Failed to write content to db_file")?;
        self.file.flush()
            .context("Failed to flush content to disk")?;
        Ok(())
    }
}

