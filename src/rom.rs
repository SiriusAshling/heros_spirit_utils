use std::fs::{self, DirEntry, File};
use std::io::{BufReader, Read, Write};
use std::path::PathBuf;

use zip::write::SimpleFileOptions;
use zip::{ZipArchive, ZipWriter};

use crate::util;
use crate::Result;

pub fn available_roms() -> Vec<DirEntry> {
    fs::read_dir("Roms")
        .into_iter()
        .flatten()
        .flatten()
        .collect()
}

pub type ArchiveReader = ZipArchive<BufReader<File>>;
pub struct RomReader {
    pub archive: ArchiveReader,
    pub index: Index,
}

impl RomReader {
    pub fn open(rom: PathBuf) -> Result<Self> {
        let file = util::file_open(rom)?;
        let archive = ZipArchive::new(BufReader::new(file))?;
        let index = Index::new(&archive);
        Ok(Self { archive, index })
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
        let archive = ZipWriter::new(util::file_create(rom)?);
        Ok(Self { archive })
    }

    pub fn write(&mut self, name: &str, bytes: &[u8]) -> Result<()> {
        self.archive
            .start_file(name, SimpleFileOptions::default())?;
        self.archive.write_all(bytes)?;
        Ok(())
    }
}
