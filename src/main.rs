mod core;

use crate::core::disk::read_database;
use color_eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;

    read_database()?;

    Ok(())
}
