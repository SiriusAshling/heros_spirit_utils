#![recursion_limit = "256"]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::struct_field_names)]
#![allow(clippy::match_same_arms)]

mod cli;
mod data;
mod graphics;
mod helpers;
mod map;
mod rom;
mod saves;

use std::error::Error;

use clap::Parser;
use cli::{export_all, export_rom, import_all, import_rom, import_saves, Action, Cli};
use helpers::OptionExtension;
use saves::Saves;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let action = cli.action.unwrap_or_prompt()?;

    match action {
        Action::Export { args } => export_all(args.rom.unwrap_or_prompt()?),
        Action::Import { args } => import_all(args.rom.unwrap_or_prompt()?),
        Action::ExportSaves => {
            Saves::decode();
        }
        Action::ImportSaves => import_saves(),
        Action::ExportRom { args } => {
            export_rom(args.rom.unwrap_or_prompt()?);
        }
        Action::ImportRom { args } => import_rom(args.rom.unwrap_or_prompt()?),
    }

    Ok(())
}
