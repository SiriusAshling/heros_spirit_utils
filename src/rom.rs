use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::PathBuf;

use zip::write::SimpleFileOptions;
use zip::{ZipArchive, ZipWriter};

use crate::graphics::TileData;
use crate::helpers::{self, ResultExtension};
use crate::map::{Map, MapColors, MapMeta};
use crate::Result;

pub type ArchiveReader = ZipArchive<BufReader<File>>;
pub struct RomReader {
    pub archive: ArchiveReader,
    pub index: Index,
}

impl RomReader {
    pub fn open(rom: PathBuf) -> Option<Self> {
        let archive = helpers::file_open(rom)
            .and_then(|file| Ok(ZipArchive::new(BufReader::new(file))?))
            .ok_feedback("Read rom")?;
        let index = Index::new(&archive);
        Some(Self { archive, index })
    }
}

pub fn read_by_index(archive: &mut ArchiveReader, index: usize) -> Result<Vec<u8>> {
    let mut file = archive.by_index(index)?;
    let mut bytes = Vec::with_capacity(file.size() as usize);
    file.read_to_end(&mut bytes)?;
    Ok(bytes)
}

#[derive(Default)]
pub struct Index {
    pub graphics: Option<usize>,
    pub maps: Vec<usize>,
    pub map_colors: Option<usize>,
    pub map_meta: Vec<usize>,
    pub images: Vec<usize>,
    pub audio: Vec<usize>,
    pub shaders: Vec<usize>,
    pub other: Vec<usize>,
}

impl Index {
    fn new(archive: &ArchiveReader) -> Self {
        let mut indices = Self::default();
        for index in 0..archive.len() {
            let name = archive.name_for_index(index).unwrap();
            indices.insert(name, index);
        }
        indices
    }

    fn insert(&mut self, name: &str, index: usize) {
        #[allow(clippy::case_sensitive_file_extension_comparisons)]
        match name {
            "graphics.bin" => self.graphics = Some(index),
            name if name.starts_with("Maps/map") => self.maps.push(index),
            "Maps/Metadata/MapColors.json" => self.map_colors = Some(index),
            name if name.starts_with("Maps/Metadata/") => self.map_meta.push(index),
            "10" | "bunnyover" | "fallenone" | "haphy" | "haphyover" | "hh" | "hhp" | "meow"
            | "ninjabunny" | "rain1" | "rain2" | "rainsplash" | "rawr" | "warrior" | "winter" => {
                self.images.push(index);
            }
            name if name.ends_with(".png") => self.images.push(index),
            name if name.starts_with("Audio/") => self.audio.push(index),
            "retrofx" => self.shaders.push(index),
            name if name.ends_with(".glsl") => self.shaders.push(index),
            _ => self.other.push(index),
        }
    }
}

pub struct RomWriter {
    pub archive: ZipWriter<File>,
}

impl RomWriter {
    pub fn create(rom: PathBuf) -> Result<Self> {
        let archive = ZipWriter::new(helpers::file_create(rom)?);
        Ok(Self { archive })
    }

    pub fn write(&mut self, name: &str, bytes: &[u8]) -> Result<()> {
        self.archive
            .start_file(name, SimpleFileOptions::default())?;
        self.archive.write_all(bytes)?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rom {
    pub tile_data: Option<TileData>,
    pub maps: Option<Vec<Map>>,
    pub map_colors: Option<MapColors>,
    pub map_meta: Option<HashMap<usize, MapMeta>>,
}

impl Rom {
    pub fn parse(rom: &mut RomReader) -> Self {
        let tile_data = TileData::parse(rom).ok_feedback("Parse graphics");
        let maps = Map::parse_all(rom).ok_feedback("Parse maps");
        let map_colors = MapColors::parse(rom).ok_feedback("Parse map colors");
        let map_meta = MapMeta::parse_all(rom).ok_feedback("Parse map meta");

        Self {
            tile_data,
            maps,
            map_colors,
            map_meta,
        }
    }
}
