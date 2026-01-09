/*
================================================================================
 ZKDB FILE HEADER FORMAT (v0.1)
 Size: 4096 bytes (1 page)
================================================================================

 Byte Offset
 ┌────────┬────────┬──────────────────────────┬──────────────────────────────┐
 │ Offset │ Size   │ Field                    │ Description                  │
 ├────────┼────────┼──────────────────────────┼──────────────────────────────┤
 │ 0x0000 │ 8      │ magic                    │ "ZKDBv01"                    │
 │ 0x0008 │ 4      │ page_size                │ Page size (4096)             │
 │ 0x000C │ 4      │ schema_offset            │ Offset of schema section     │
 │ 0x0010 │ 4      │ schema_size              │ Size of schema section       │
 │ 0x0014 │ 8      │ segment_dir_offset       │ Offset of segment directory  │
 │ 0x001C │ 4      │ segment_count            │ Number of segments           │
 │ 0x0020 │ 4064   │ reserved                 │ Must be zero-filled          │
 └────────┴────────┴──────────────────────────┴──────────────────────────────┘

 Invariants:
 - Header size is fixed at 4096 bytes
 - Magic must match exactly or file is invalid
 - All numeric fields are little-endian
 - Reserved bytes must be zero
 - Schema and segment directory are append-only
 - No in-place updates are allowed

================================================================================
*/
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use color_eyre::{Result, eyre::{eyre, Context}};
use std::path::Path;

pub const HEADER_SIZE: usize = 4096;
pub const OFFSET_MAGIC: usize = 0;
pub const OFFSET_PAGE_SIZE: usize = 8;
pub const OFFSET_SCHEMA_OFFSET: usize = 12;
pub const OFFSET_SCHEMA_SIZE: usize = 16;
pub const OFFSET_SEGMENT_DIR_OFFSET: usize = 20;
pub const OFFSET_SEGMENT_COUNT: usize = 28;
pub const MAGIC: &[u8; 8] = b"ZKDBv01\0";
pub const PAGE_SIZE: u32 = 4096;

pub fn create<P: AsRef<Path>>(path: P) -> Result<()> {
    let mut header = [0u8; HEADER_SIZE];

    // magic
    header[OFFSET_MAGIC..OFFSET_MAGIC + 8].copy_from_slice(MAGIC);

    // page_size
    header[OFFSET_PAGE_SIZE..OFFSET_PAGE_SIZE + 4]
        .copy_from_slice(&PAGE_SIZE.to_le_bytes());

    // schema_offset
    header[OFFSET_SCHEMA_OFFSET..OFFSET_SCHEMA_OFFSET + 4]
        .copy_from_slice(&0u32.to_le_bytes());

    // schema_size
    header[OFFSET_SCHEMA_SIZE..OFFSET_SCHEMA_SIZE + 4]
        .copy_from_slice(&0u32.to_le_bytes());

    // segment_dir_offset
    header[OFFSET_SEGMENT_DIR_OFFSET..OFFSET_SEGMENT_DIR_OFFSET + 8]
        .copy_from_slice(&0u64.to_le_bytes());

    // segment_count
    header[OFFSET_SEGMENT_COUNT..OFFSET_SEGMENT_COUNT + 4]
        .copy_from_slice(&0u32.to_le_bytes());

    let mut file = File::create(&path)
        .context("Failed to create ZKDB file")?;

    file.write_all(&header)
        .context("Failed to write ZKDB header")?;

    file.sync_all()
        .context("Failed to fsync ZKDB header")?;

    Ok(())
}

pub fn validate_magic(header: &[u8]) -> Result<()> {
    if header.len() < 8 {
        return Err(eyre!("Header too small to contain magic bytes"));
    }

    let magic_bytes = &header[OFFSET_MAGIC..OFFSET_MAGIC + 8];
    if magic_bytes != MAGIC {
        return Err(eyre!(
            "Invalid magic bytes. Expected {:?}, got {:?}",
            MAGIC,
            magic_bytes
        ));
    }

    Ok(())
}

pub fn validate_page_size(header: &[u8]) -> Result<()> {
    if header.len() < OFFSET_PAGE_SIZE + 4 {
        return Err(eyre!("Header too small to contain page_size"));
    }

    let page_size_bytes = &header[OFFSET_PAGE_SIZE..OFFSET_PAGE_SIZE + 4];
    let page_size = u32::from_le_bytes([
        page_size_bytes[0],
        page_size_bytes[1],
        page_size_bytes[2],
        page_size_bytes[3],
    ]);

    if page_size != 4096 {
        return Err(eyre!("Invalid page size: {}. Expected 4096", page_size));
    }

    Ok(())
}

pub fn validate_header(header: &[u8]) -> Result<()> {
    if header.len() < HEADER_SIZE {
        return Err(eyre!(
            "File too small. Expected at least {} bytes, got {}",
            HEADER_SIZE,
            header.len()
        ));
    }

    validate_magic(header)
        .context("Failed to validate magic bytes")?;

    validate_page_size(header)
        .context("Failed to validate page size")?;

    Ok(())
}

pub fn read_and_validate_header(file: &mut File) -> Result<[u8; HEADER_SIZE]> {
    let mut header = [0u8; HEADER_SIZE];

    file.seek(SeekFrom::Start(0))
        .context("Failed to seek to beginning of file")?;

    file.read_exact(&mut header)
        .context("Failed to read header from file")?;

    validate_header(&header)
        .context("Header validation failed")?;

    Ok(header)
}


