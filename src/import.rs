use std::ffi::OsStr;
use std::path::PathBuf;

use crate::map::{self, Map};
use crate::rom::RomWriter;
use crate::{draw, graphics, savedata, util, Result};

pub fn import(rom: Option<PathBuf>) {
    util::feedback("Import savedata", savedata::encode("savedata"));
    util::feedback("Import savedatb", savedata::encode("savedatb"));
    util::feedback("Import savedatc", savedata::encode("savedatc"));
    util::feedback("Import bunny", savedata::encode("bunny"));

    let Some(rom) = rom else { return };
    util::feedback("Import rom", import_rom(rom));
}

fn import_rom(rom: PathBuf) -> Result<()> {
    let mut rom = RomWriter::create(rom)?;

    import_tilesets(&mut rom)?;
    import_maps(&mut rom)?;
    import_files("Textures", &mut rom)?;
    import_files("SFX", &mut rom)?;
    import_files("Music", &mut rom)?;
    import_files("Shaders", &mut rom)?;
    import_files("Other", &mut rom)?;

    Ok(())
}

fn import_tilesets(rom: &mut RomWriter) -> Result<()> {
    let tile8_list = draw::undraw_tile8s("rom_files/Graphics/tile8.bmp")?;
    rom.write("graphics.bin", &graphics::encode_graphics(tile8_list))?;

    Ok(())
}

fn import_maps(rom: &mut RomWriter) -> Result<()> {
    for file in util::read_dir("rom_files/Maps")? {
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
        let map = Map::from_tmx(identifier, &util::read_to_string(&path)?)?;
        rom.write(&format!("Maps/{name}"), &map::encode_map(map))?;
    }

    Ok(())
}

fn import_files(folder: &str, rom: &mut RomWriter) -> Result<()> {
    for file in util::read_dir(&format!("rom_files/{folder}"))? {
        let file = file?;
        let file_name = file.file_name();
        let file_name = file_name
            .to_str()
            .ok_or_else(|| format!("invalid filename \"{}\"", file_name.to_string_lossy()))?;
        rom.write(file_name, &util::read(file.path())?)?;
    }

    Ok(())
}
