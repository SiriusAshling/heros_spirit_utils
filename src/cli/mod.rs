mod export;
mod import;
mod randomize;

pub use export::{export_all, export_rom};
pub use import::{import_all, import_rom, import_saves};
pub use randomize::{draw_logic, randomize};

use std::fmt::{self, Display};
use std::fs::DirEntry;
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use inquire::Select;
use strum::{Display, EnumDiscriminants, VariantArray};

use crate::helpers::files_in_dir;
use crate::Result;

pub trait FromPrompt: Sized {
    fn from_prompt() -> Result<Self>;
}

#[derive(Parser)]
#[clap(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub action: Option<Action>,
}

#[derive(Subcommand, EnumDiscriminants)]
#[strum_discriminants(derive(VariantArray, Display), strum(serialize_all = "kebab-case"))]
pub enum Action {
    /// Randomize transfer destinations and item locations
    ///
    /// Roms are looked for in a "Roms/" subfolder.
    /// Additionally the file "rando/logic.json" is required.
    Randomize {
        #[command(flatten)]
        args: RandomizeArgs,
    },
    /// Visualize the contents of "rando/logic.json"
    ///
    /// Roms are looked for in a "Roms/" subfolder.
    DrawLogic {
        #[command(flatten)]
        args: RomArgs,
    },
    /// Exports the rom and save files into formats suitable for viewing and editing.
    ///
    /// If it can find both the rom and save files, it can compare the two to determine which items you're missing.
    ///
    /// Roms are looked for in a "Roms/" subfolder, Saves are looked for in the current directory.
    Export {
        #[command(flatten)]
        args: RomArgs,
    },
    /// Reimport all files previously exported
    Import {
        #[command(flatten)]
        args: RomArgs,
    },
    /// Exports save files into formats suitable for viewing and editing.
    ///
    /// Saves are looked for in the current directory.
    ExportSaves,
    /// Reimport the files previously exported with "Export Saves"
    ImportSaves,
    /// Exports the rom into formats suitable for viewing and editing.
    ///
    /// Roms are looked for in a "Roms/" subfolder.
    ExportRom {
        #[command(flatten)]
        args: RomArgs,
    },
    /// Reimport the files previously exported with "Export Rom"
    ImportRom {
        #[command(flatten)]
        args: RomArgs,
    },
}

impl FromPrompt for Action {
    fn from_prompt() -> Result<Self> {
        let action =
            Select::new("Select an action", ActionDiscriminants::VARIANTS.to_vec()).prompt()?;

        let action = match action {
            ActionDiscriminants::Randomize => Action::Randomize {
                args: RandomizeArgs::default(),
            },
            ActionDiscriminants::DrawLogic => Action::DrawLogic {
                args: RomArgs::default(),
            },
            ActionDiscriminants::Export => Action::Export {
                args: RomArgs::default(),
            },
            ActionDiscriminants::Import => Action::Import {
                args: RomArgs::default(),
            },
            ActionDiscriminants::ExportSaves => Action::ExportSaves,
            ActionDiscriminants::ImportSaves => Action::ImportSaves,
            ActionDiscriminants::ExportRom => Action::ExportRom {
                args: RomArgs::default(),
            },
            ActionDiscriminants::ImportRom => Action::ImportRom {
                args: RomArgs::default(),
            },
        };

        Ok(action)
    }
}

#[derive(Args, Default)]
pub struct RandomizeArgs {
    #[command(flatten)]
    pub rom_args: RomArgs,
    /// Using the same seed will result in the same output
    #[arg(long, short)]
    pub seed: Option<String>,
}

#[derive(Args, Default)]
pub struct RomArgs {
    /// Path to the rom to operate on
    #[arg(short, long)]
    pub rom: Option<PathBuf>,
}

impl FromPrompt for PathBuf {
    fn from_prompt() -> Result<Self> {
        let available_roms = files_in_dir("Roms")?.collect::<Vec<_>>();
        let rom = match available_roms.len() {
            0 => Err("no roms found")?,
            1 => available_roms[0].path(),
            _ => {
                let options = available_roms.into_iter().map(DirEntryDisplay).collect();
                let selection = Select::new("Select a rom", options).prompt()?;
                selection.0.path()
            }
        };

        Ok(rom)
    }
}

struct DirEntryDisplay(DirEntry);
impl Display for DirEntryDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.file_name().to_string_lossy().fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use crate::rom::{Rom, RomReader};

    use super::*;

    #[test]
    fn roundtrip() {
        let mut reader = RomReader::open("Roms/main.hsrom".into()).unwrap();
        let first = Rom::parse(&mut reader);
        first.export(&mut reader);

        import_rom("Roms/test.hsrom".into());

        let mut reader = RomReader::open("Roms/test.hsrom".into()).unwrap();
        let second = Rom::parse(&mut reader);

        assert_eq!(first, second);
    }
}
