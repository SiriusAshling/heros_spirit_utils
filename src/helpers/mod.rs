mod fs;
mod option;
mod rand;
mod result;

pub use fs::{
    create_dir_all, file_create, file_open, files_in_dir, read, read_dir, read_to_string, write,
};
pub use option::OptionExtension;
pub use rand::RemoveRandom;
pub use result::ResultExtension;
