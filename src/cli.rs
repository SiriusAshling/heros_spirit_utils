use std::fmt::{self, Display};
use std::fs::DirEntry;
use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use inquire::Select;
use strum::{Display, VariantArray};

use crate::rom::available_roms;
use crate::util::const_concat;
use crate::Result;

#[derive(Parser)]
#[clap(version, about)]
struct Cli {
    action: Option<Action>,
    #[arg(short, long)]
    rom: Option<PathBuf>,
}

pub struct CliOutput {
    pub action: Action,
    /// Name of the rom to operate on
    ///
    /// It will be searched at "Roms/<rom>.hsrom"
    pub rom: Option<PathBuf>,
}

#[derive(Clone, ValueEnum, VariantArray, Display)]
pub enum Action {
    /// Exports the rom and save files into formats suitable for viewing and editing
    ///
    /// Saves are looked for in the current directory. Roms are looked for in a "Roms/" subfolder.
    Export,
    /// Reimport any files exported earlier
    Import,
}

pub fn cli() -> Result<CliOutput> {
    let cli = Cli::parse();

    let action = match cli.action {
        None => Select::new("Select an action", Action::VARIANTS.to_vec()).prompt()?,
        Some(action) => action,
    };

    let rom = match cli.rom {
        None => {
            let available_roms = available_roms();
            match available_roms.len() {
                0 => None,
                1 => Some(available_roms[0].path()),
                _ => {
                    let options = available_roms.into_iter().map(DirEntryDisplay).collect();
                    let selection = Select::new("Select a rom", options)
                        .with_help_message(SKIPPABLE_HELP_MESSAGE)
                        .prompt_skippable()?;
                    selection.map(|dir_entry| dir_entry.0.path())
                }
            }
        }
        Some(rom) => {
            let mut path = PathBuf::from("Roms");
            path.push(rom);
            path.set_extension("hsrom");
            Some(path)
        }
    };

    Ok(CliOutput { action, rom })
}

struct DirEntryDisplay(DirEntry);
impl Display for DirEntryDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.file_name().to_string_lossy().fmt(f)
    }
}

// it had to be done
const SKIPPABLE_HELP_MESSAGE: &str = {
    const DEFAULT: &str = {
        match Select::<u8>::DEFAULT_HELP_MESSAGE {
            None => panic!("no default help message"),
            Some(default) => default,
        }
    };
    const_concat!(DEFAULT, ", esc to skip")
};
