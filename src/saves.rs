use std::{
    fs,
    io::{self, Read, Write},
    path::{Path, PathBuf},
};

use itertools::Itertools;

use crate::{feedback::try_or_feedback, files, Result};

pub fn export_saves(folder: PathBuf, names: Vec<String>) {
    try_or_feedback("Export Saves", || {
        for path in names_to_paths(folder, names, "hson")? {
            export_save(path);
        }
        Ok(())
    });
}
pub fn import_saves(mut folder: PathBuf, names: Vec<String>) {
    try_or_feedback("Import Saves", || {
        folder.push("export");
        for path in names_to_paths(folder, names, "json")? {
            import_save(path);
        }
        Ok(())
    });
}

fn names_to_paths(folder: PathBuf, names: Vec<String>, extension: &str) -> Result<Vec<PathBuf>> {
    if names.is_empty() {
        find_saves(folder, extension)
    } else {
        Ok(names.into_iter().map(PathBuf::from).collect())
    }
}
fn find_saves(folder: PathBuf, extension: &str) -> Result<Vec<PathBuf>> {
    let saves = fs::read_dir(folder)?
        .filter_map_ok(|entry| {
            let path = entry.path();
            if path.extension().map_or(false, |e| e == extension) {
                Some(path)
            } else {
                None
            }
        })
        .collect::<io::Result<_>>()?;
    Ok(saves)
}

fn export_save(mut path: PathBuf) {
    let name = path.file_stem().unwrap().to_string_lossy();
    try_or_feedback(format!("Export Save {name}"), || {
        let hson = files::open(&path)?;
        let json = flate2::read::GzDecoder::new(hson);
        push_folder(&mut path, "export")?;
        path.set_extension("json");
        let out = files::create(path)?;
        jsonformat::format_reader_writer(json, out, Default::default())?;

        Ok(())
    });
}
fn import_save(mut path: PathBuf) {
    let name = path.file_stem().unwrap().to_string_lossy();
    try_or_feedback(format!("Import Save {name}"), || {
        let json = files::open(&path)?;
        pop_folder(&mut path);
        path.set_extension("hson");
        let out = files::create(path)?;
        let hson = flate2::write::GzEncoder::new(out, Default::default());
        unformat(json, hson)?;

        Ok(())
    });
}

fn push_folder<P: AsRef<Path>>(path: &mut PathBuf, folder: P) -> Result<()> {
    let file = path.file_name().unwrap().to_owned();
    path.pop();
    path.push(folder);
    fs::create_dir_all(&path)?;
    path.push(file);
    Ok(())
}
fn pop_folder(path: &mut PathBuf) {
    let file = path.file_name().unwrap().to_owned();
    path.pop();
    path.pop();
    path.push(file);
}

fn unformat<R, W>(reader: R, mut writer: W) -> Result<()>
where
    R: Read,
    W: Write,
{
    let mut in_string = false;
    let mut escaped = false;

    for byte in reader.bytes() {
        let byte = byte?;

        if in_string {
            if escaped {
                escaped = false;
            } else {
                match byte {
                    b'"' => in_string = false,
                    b'\\' => escaped = true,
                    _ => {}
                }
            }
        } else {
            if byte.is_ascii_whitespace() {
                continue;
            }
            if byte == b'"' {
                in_string = true
            }
        }
        writer.write_all(&[byte])?;
    }

    Ok(())
}
