use std::collections::{HashMap, HashSet};
use std::io;
use std::path::{Path, PathBuf};

use image::ImageFormat;
use image::RgbaImage;
use itertools::Itertools;

use crate::graphics::{merge_maps, DrawData};
use crate::helpers::ResultExtension;
use crate::map::{self, Collectible, Enemy, Map};
use crate::rom::{ArchiveReader, Rom, RomReader};
use crate::saves::Saves;
use crate::{helpers, Result};

#[allow(clippy::similar_names)]
pub fn export_all(rom: PathBuf) {
    let saves = Saves::decode();

    if let Some(rom) = export_rom(rom) {
        rom.export_missing_items(&saves);
    }
}

pub fn export_rom(rom: PathBuf) -> Option<Rom> {
    let mut reader = RomReader::open(rom)?;
    let rom = Rom::parse(&mut reader);

    rom.export(&mut reader);
    rom.export_extras();

    Some(rom)
}

impl Rom {
    pub fn export(&self, reader: &mut RomReader) {
        if let (Some(tile_data), Some(map_colors), Some(map_meta)) =
            (&self.tile_data, &self.map_colors, &self.map_meta)
        {
            let data = DrawData {
                tile_data,
                map_colors,
                map_meta,
            };

            export_tilesets(&data).feedback("Export graphics");

            if let Some(maps) = &self.maps {
                export_maps("rom_files/Maps", &maps, &data).feedback("Export maps");
            }
        }

        if let Some(index) = reader.index.map_colors {
            export_file("rom_files/Maps/Metadata", &mut reader.archive, index)
                .feedback("Export map colors");
        }

        export_files(
            "rom_files/Maps/Metadata",
            &mut reader.archive,
            &reader.index.map_meta,
        )
        .feedback("Export map meta");

        export_files(
            "rom_files/Textures",
            &mut reader.archive,
            &reader.index.images,
        )
        .feedback("Export images");

        export_files("rom_files", &mut reader.archive, &reader.index.audio)
            .feedback("Export sounds");

        export_files(
            "rom_files/Shaders",
            &mut reader.archive,
            &reader.index.shaders,
        )
        .feedback("Export shaders");

        export_files("rom_files/Other", &mut reader.archive, &reader.index.other)
            .feedback("Export other files");
    }

    pub fn export_extras(&self) {
        if let Some(maps) = &self.maps {
            export_stats(&maps).feedback("Gather stats");

            if let (Some(tile_data), Some(map_colors), Some(map_meta)) =
                (&self.tile_data, &self.map_colors, &self.map_meta)
            {
                let data = DrawData {
                    tile_data,
                    map_colors,
                    map_meta,
                };

                export_maps("rom_files/Maps", &maps, &data).feedback("Export maps");

                let maps = maps
                    .into_iter()
                    .map(|map| {
                        let identifier = map.identifier;
                        let map = data.draw_map(&map);

                        export_map_image(identifier, &map)
                            .feedback(format!("Draw map {}", map::map_name(identifier)));

                        (identifier, map)
                    })
                    .collect();

                for (name, map) in merge_maps(maps) {
                    export_image(name, &map).feedback(format!("Draw {name}"));
                }
            }
        }
    }

    pub fn export_missing_items(&self, saves: &Saves) {
        if let (Some(maps), Some(savedata), Some(savedatb), Some(savedatc), Some(bunny)) = (
            &self.maps,
            &saves.savedata,
            &saves.savedatb,
            &saves.savedatc,
            &saves.bunny,
        ) {
            savedata
                .check("savedata", maps)
                .and(savedatb.check("savedatb", maps))
                .and(savedatc.check("savedatc", maps))
                .and(bunny.check("bunny", maps))
                .feedback("Check missing items from saves");
        }
    }
}

fn export_tilesets(data: &DrawData) -> Result<()> {
    helpers::create_dir_all("rom_files/Graphics/tile16")?;
    data.draw_tile8s("rom_files/Graphics/tile8.bmp")?;
    data.draw_tile16s()
}

fn export_files(path: &str, archive: &mut ArchiveReader, indices: &[usize]) -> Result<()> {
    for index in indices {
        export_file(path, archive, *index)?;
    }

    Ok(())
}

fn export_file(path: &str, archive: &mut ArchiveReader, index: usize) -> Result<()> {
    let mut reader = archive.by_index(index)?;
    let name = reader
        .enclosed_name()
        .ok_or_else(|| format!("Failed to sanitize filename \"{}\"", reader.name()))?;
    let mut file_path = PathBuf::from(path);
    file_path.push(name);
    let mut writer = helpers::file_create(file_path)?;
    io::copy(&mut reader, &mut writer)?;

    Ok(())
}

fn export_maps(path: impl AsRef<Path>, maps: &[Map], data: &DrawData) -> Result<()> {
    for map in maps {
        let mut path = path.as_ref().to_owned();
        path.push(format!("map{:02}.tmx", map.identifier));

        helpers::write(path, map.to_tmx(data)?)?;
    }

    Ok(())
}

fn export_map_image(identifier: u8, map: &RgbaImage) -> Result<()> {
    let map_name = map::map_name(identifier);

    export_image(format!("{identifier}_{map_name}.png"), map)
}

fn export_image<P: AsRef<Path>>(name: P, map: &RgbaImage) -> Result<()> {
    fn export_image(name: &Path, map: &RgbaImage) -> Result<()> {
        let mut path = PathBuf::from("rom_files/Maps/images");
        helpers::create_dir_all(&path)?;
        path.push(name);

        map.save_with_format(path, ImageFormat::Png)?;

        Ok(())
    }

    export_image(name.as_ref(), map)
}

fn export_stats(maps: &[Map]) -> Result<()> {
    let sprite_stats = maps
        .iter()
        .map(|map| (map.identifier, map.stats()))
        .collect::<HashMap<_, _>>();

    let mut all_collectibles = HashSet::<Collectible>::new();
    let mut all_enemies = HashSet::<Enemy>::new();
    for stats in sprite_stats.values() {
        all_collectibles.extend(stats.collectibles.keys());
        all_enemies.extend(stats.enemies.keys());
    }
    let mut all_collectibles = Vec::from_iter(all_collectibles);
    all_collectibles.sort_unstable();
    let mut all_enemies = Vec::from_iter(all_enemies);
    all_enemies.sort_unstable();

    let mut sprite_stats = Vec::from_iter(sprite_stats);
    sprite_stats.sort_unstable_by_key(|(map, _)| *map);

    let collectibles_header = format!(
        ", {}",
        all_collectibles
            .iter()
            .format_with(", ", |collectible, f| f(&format_args!("{collectible:?}")))
    );
    let collectibles_stats = sprite_stats.iter().format_with("\n", |(map, stats), f| {
        f(&format_args!(
            "{}, {}",
            map::map_name(*map),
            all_collectibles
                .iter()
                .map(|collectible| stats.collectibles.get(collectible).copied().unwrap_or(0))
                .format(", ")
        ))
    });
    let enemies_header = format!(
        ", {}",
        all_enemies
            .iter()
            .format_with(", ", |enemy, f| f(&format_args!("{enemy:?}")))
    );
    let enemies_stats = sprite_stats.iter().format_with("\n", |(map, stats), f| {
        f(&format_args!(
            "{}, {}",
            map::map_name(*map),
            all_enemies
                .iter()
                .map(|enemy| stats.enemies.get(enemy).copied().unwrap_or(0))
                .format(", ")
        ))
    });

    let collectibles_stats = format!("{collectibles_header}\n{collectibles_stats}");
    let enemies_stats = format!("{enemies_header}\n{enemies_stats}");

    helpers::write("rom_files/Maps/stats/collectibles.csv", collectibles_stats)?;
    helpers::write("rom_files/Maps/stats/enemies.csv", enemies_stats)?;

    Ok(())
}
