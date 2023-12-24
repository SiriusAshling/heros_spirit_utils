use std::{fs::File, path::Path};

use crate::Result;

pub fn open<P: AsRef<Path>>(path: P) -> Result<File> {
    let file = File::open(&path)
        .map_err(|err| format!("Error opening \"{}\": {err}", path.as_ref().display()))?;
    Ok(file)
}

pub fn create<P: AsRef<Path>>(path: P) -> Result<File> {
    let file = File::create(&path)
        .map_err(|err| format!("Error creating \"{}\": {err}", path.as_ref().display()))?;
    Ok(file)
}
