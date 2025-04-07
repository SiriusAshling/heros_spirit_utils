#![recursion_limit = "256"]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::struct_field_names)]
#![allow(clippy::match_same_arms)]

mod cli;
mod data;
mod draw;
mod export;
mod global;
mod graphics;
mod import;
mod inventory;
mod map;
mod missing;
mod palette;
mod rom;
mod savedata;
mod sprite;
mod stats;
mod tiled;
mod util;

use std::error::Error;

use cli::{cli, Action, CliOutput};
use export::export;
use import::import;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let CliOutput { action, rom } = cli()?;
    match action {
        Action::Export => export(rom),
        Action::Import => import(rom),
    }

    Ok(())
}
