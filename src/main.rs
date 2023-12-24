mod feedback;
mod files;
mod saves;

use std::{error::Error, path::PathBuf};

use clap::{Parser, Subcommand};
use strum::{Display, EnumIter, IntoEnumIterator};

fn main() {
    let args = Cli::parse();
    args.command.execute();
}

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}
#[derive(Subcommand)]
enum Command {
    /// Export data into viewable formats ("help export" for more information)
    ///
    /// If [Command] is not specified, everything is exported
    Export {
        #[command(subcommand)]
        export_command: Option<ExportCommand>,
    },
    /// Import data from the exported formats ("help import" for more information)
    ///
    /// If [Command] is not specified, everything is imported
    Import {
        #[command(subcommand)]
        import_command: Option<ImportCommand>,
    },
}
#[derive(Subcommand, EnumIter, Display)]
enum ExportCommand {
    /// Export saves ("help export saves" for more information)
    Saves {
        /// Saves Folder.
        /// Defaults to the current working directory.
        /// Vanilla saves will be searched in <FOLDER> and exported saves will be stored in <FOLDER>/export
        #[arg(short, long)]
        folder: Option<PathBuf>,
        /// Names of saves to export, e.g. "game0", "global".
        /// If no names are specified, all saves are exported
        names: Vec<String>,
    },
}
#[derive(Subcommand, EnumIter, Display)]
enum ImportCommand {
    /// Import saves ("help import saves" for more information)
    Saves {
        /// Saves Folder.
        /// Defaults to the current working directory.
        /// Saves to import will be searched in <FOLDER>/export and created vanilla saves will be stored in <FOLDER>
        #[arg(short, long)]
        folder: Option<PathBuf>,
        /// Names of saves to import, e.g. "game0", "global"
        /// If no names are specified, all saves are imported
        names: Vec<String>,
    },
}

trait Execute {
    fn execute(self);
}
impl<T> Execute for Option<T>
where
    T: Execute + IntoEnumIterator,
{
    fn execute(self) {
        match self {
            None => {
                execute_iter(T::iter());
            }
            Some(t) => t.execute(),
        }
    }
}
// TODO unused
impl<T> Execute for Vec<T>
where
    T: Execute + IntoEnumIterator,
{
    fn execute(self) {
        if self.is_empty() {
            execute_iter(T::iter());
        } else {
            execute_iter(self);
        }
    }
}
fn execute_iter<I>(i: I)
where
    I: IntoIterator,
    I::Item: Execute,
{
    for t in i {
        t.execute();
    }
}
impl Execute for Command {
    fn execute(self) {
        match self {
            Command::Export { export_command } => export_command.execute(),
            Command::Import { import_command } => import_command.execute(),
        }
    }
}
impl Execute for ExportCommand {
    fn execute(self) {
        match self {
            ExportCommand::Saves { folder, names } => {
                saves::export_saves(folder.unwrap_or_default(), names)
            }
        }
    }
}
impl Execute for ImportCommand {
    fn execute(self) {
        match self {
            ImportCommand::Saves { folder, names } => {
                saves::import_saves(folder.unwrap_or_default(), names)
            }
        }
    }
}
