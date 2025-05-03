use std::ffi::OsStr;
use std::fmt::Debug;
use std::path::{Path, PathBuf};

use crate::helpers::ResultExtension;
use crate::map::Map;
use crate::rom::RomWriter;
use crate::{graphics, helpers, saves, Result};

pub fn import_all(rom: PathBuf) {
    import_saves();
    import_rom(rom);
}

pub fn import_saves() {
    saves::encode("savedata").feedback("Import savedata");
    saves::encode("savedatb").feedback("Import savedatb");
    saves::encode("savedatc").feedback("Import savedatc");
    saves::encode("bunny").feedback("Import bunny");
}

pub fn import_rom(rom: PathBuf) {
    if let Some(mut rom) = RomWriter::create(rom).ok_feedback("Create rom") {
        rom.import_tilesets().feedback("Import graphics");
        rom.import_maps().feedback("Import maps");
        rom.import_files("Maps/Metadata", "Maps/Metadata/")
            .feedback("Import map meta");
        rom.import_files("Textures", "").feedback("Import images");
        rom.import_files("Audio", "Audio/")
            .feedback("Import sounds");
        rom.import_files("Shaders", "").feedback("Import shaders");
        rom.import_files("Other", "").feedback("Import other files");
    }
}

impl RomWriter {
    fn import_tilesets(&mut self) -> Result<()> {
        let tile8_list = graphics::undraw_tile8s("rom_files/Graphics/tile8.bmp")?;
        self.write("graphics.bin", &graphics::encode_graphics(tile8_list))?;

        Ok(())
    }

    fn import_maps(&mut self) -> Result<()> {
        for file in helpers::read_dir("rom_files/Maps")? {
            let file = file?;
            let path = file.path();
            if path.extension() != Some(OsStr::new("tmx")) {
                continue;
            }
            let name = path.file_stem().unwrap();
            let name = name
                .to_str()
                .ok_or_else(|| format!("invalid filename \"{}\"", path.display()))?;

            let identifier = name
                .strip_prefix("map")
                .and_then(|id| id.parse().ok())
                .ok_or_else(|| format!("invalid map identifier \"{name}\""))?;
            let map = Map::from_tmx(identifier, &helpers::read_to_string(&path)?)?;
            self.write(&format!("Maps/{name}"), &map.encode())?;
        }

        Ok(())
    }

    fn import_files(&mut self, folder: &str, prefix: &str) -> Result<()> {
        for file in helpers::read_dir(format!("rom_files/{folder}"))? {
            let file = file?;
            let file_name = file.file_name();
            let file_name = file_name
                .to_str()
                .ok_or_else(|| format!("invalid filename \"{}\"", file_name.to_string_lossy()))?;

            if file.file_type()?.is_dir() {
                self.import_files(
                    &format!("{folder}/{file_name}"),
                    &format!("{prefix}{file_name}/"),
                )?;
            } else {
                self.import_file(file.path(), &format!("{prefix}{file_name}"))?;
            }
        }

        Ok(())
    }

    fn import_file<P: AsRef<Path> + Debug>(&mut self, path: P, name: &str) -> Result<()> {
        self.write(name, &helpers::read(path)?)
    }
}
