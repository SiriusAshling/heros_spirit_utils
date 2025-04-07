use std::path::{Path, PathBuf};
use std::{fs, io};

use image::ImageFormat;
use image::RgbaImage;

use crate::graphics::TileData;
use crate::map::{self, Map};
use crate::rom::{ArchiveReader, RomReader};
use crate::{draw, global, missing, savedata, stats, util, Result};

#[allow(clippy::similar_names)]
pub fn export(rom: Option<PathBuf>) {
    let savedata = util::feedback("Export savedata", savedata::decode("savedata", true));
    let savedatb = util::feedback("Export savedatb", savedata::decode("savedatb", true));
    let savedatc = util::feedback("Export savedatc", savedata::decode("savedatc", true));
    let bunny = util::feedback("Export bunny", savedata::decode("bunny", false));
    util::feedback("Export global save", global::decode());

    let Some(rom) = rom else { return };
    let Some(mut rom) = util::feedback("Read rom", RomReader::open(rom)) else {
        return;
    };

    let tile_data = util::feedback("Parse graphics", TileData::parse(&mut rom));
    let maps = util::feedback("Parse maps", map::parse_maps(&mut rom));

    if let Some(tile_data) = &tile_data {
        util::feedback("Export graphics", export_tilesets(tile_data));
    }

    if let Some(maps) = maps {
        util::feedback("Gather stats", stats::map_stats(&maps));

        util::feedback("Check missing items from saves", {
            let savedata = missing::check("savedata", savedata, &maps);
            let savedatb = missing::check("savedatb", savedatb, &maps);
            let savedatc = missing::check("savedatc", savedatc, &maps);
            let bunny = missing::check("bunny", bunny, &maps);
            savedata.and(savedatb).and(savedatc).and(bunny)
        });

        if let Some(tile_data) = tile_data {
            util::feedback(
                "Export maps",
                export_maps("rom_files/Maps", &maps, &tile_data),
            );

            let maps = maps
                .into_iter()
                .map(|map| {
                    let identifier = map.identifier;
                    let map = draw::draw_map(map, &tile_data);
                    util::feedback(
                        format!("Draw map {}", map::map_name(identifier)),
                        export_map_image("rom_files/Maps/images", identifier, &map),
                    );
                    (identifier, map)
                })
                .collect();
            util::feedback(
                "Draw world map",
                export_full_map_image(&draw::merge_maps(maps)),
            );
        }
    }

    util::feedback(
        "Export images",
        export_files("rom_files/Textures", &mut rom.archive, &rom.index.images),
    );

    util::feedback(
        "Export sounds",
        export_files("rom_files/SFX", &mut rom.archive, &rom.index.sounds),
    );

    util::feedback(
        "Export music",
        export_files("rom_files/Music", &mut rom.archive, &rom.index.music),
    );

    util::feedback(
        "Export shaders",
        export_files("rom_files/Shaders", &mut rom.archive, &rom.index.shaders),
    );

    util::feedback(
        "Export other files",
        export_files("rom_files/Other", &mut rom.archive, &rom.index.other),
    );
}

fn export_tilesets(tile_data: &TileData) -> Result<()> {
    fs::create_dir_all("rom_files/Graphics/tile16")?;
    draw::draw_tile8s("rom_files/Graphics/tile8.bmp", &tile_data.tile8_list)?;
    draw::draw_tile16s(tile_data)
}

fn export_files(path: &str, archive: &mut ArchiveReader, indices: &[usize]) -> Result<()> {
    for index in indices.iter().copied() {
        let mut reader = archive.by_index(index)?;
        let name = reader
            .enclosed_name()
            .ok_or_else(|| format!("Failed to sanitize filename \"{}\"", reader.name()))?;
        let mut file_path = PathBuf::from(path);
        file_path.push(name);
        let mut writer = util::file_create(file_path)?;
        io::copy(&mut reader, &mut writer)?;
    }

    Ok(())
}

fn export_maps(path: impl AsRef<Path>, maps: &[Map], tile_data: &TileData) -> Result<()> {
    for map in maps {
        let mut path = path.as_ref().to_owned();
        path.push(format!("map{:02}.tmx", map.identifier));

        util::write(path, map.to_tmx(tile_data)?)?;
    }

    Ok(())
}

fn export_map_image(path: impl AsRef<Path>, identifier: u8, map: &RgbaImage) -> Result<()> {
    let mut path = path.as_ref().to_owned();
    fs::create_dir_all(&path)?;
    let map_name = map::map_name(identifier);
    path.push(format!("{identifier}_{map_name}.png"));

    map.save_with_format(path, ImageFormat::Png)?;

    Ok(())
}

fn export_full_map_image(map: &RgbaImage) -> Result<()> {
    let mut path = PathBuf::from("rom_files/Maps/images");
    fs::create_dir_all(&path)?;
    path.push("FullMap.png");

    map.save_with_format(path, ImageFormat::Png)?;

    Ok(())
}
