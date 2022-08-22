use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::error::Error;

use crate::map::{self, Map};
use crate::zip::NamedFile;
use crate::{draw, graphics};

pub fn import_files(path: impl AsRef<Path>, files: &mut Vec<NamedFile>, extension: impl AsRef<OsStr>) -> Result<(), Box<dyn Error>> {
    files.extend(fs::read_dir(path)?.filter_map(|file| {
        let file = file.ok()?;
        if PathBuf::from(file.file_name()).extension().map_or(false, |ext| ext == extension.as_ref()) {
            let mut filename = PathBuf::from(file.file_name());
            filename.set_extension("");
            let bytes = fs::read(file.path()).ok()?;
            Some((filename.to_string_lossy().to_string(), bytes))
        } else { None }
    }));

    Ok(())
}

pub fn import_tilesets(path: impl AsRef<Path>, files: &mut Vec<NamedFile>) -> Result<(), Box<dyn Error>> {
    let mut path = path.as_ref().to_owned();
    path.push("tile8/all.bmp");
    let tile8_list = draw::undraw_tile8s(path)?;
    files.push(("graphics".to_string(), graphics::encode_graphics(tile8_list)));
    Ok(())
}

pub fn import_maps(path: impl AsRef<Path>, files: &mut Vec<NamedFile>) -> Result<(), Box<dyn Error>> {
    let mut maps = Vec::new();
    import_files(path, &mut maps, "txt")?;
    for (_, bytes) in maps {
        let map: Map = String::from_utf8(bytes)?.parse()?;
        files.push((format!("map{:02}", map.identifier as u8), map::encode_map(map)));
    }

    Ok(())
}
