use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use zip::ZipArchive;

use crate::data::PASS;

pub type NamedFile = (String, Vec<u8>);
pub fn read_rom(path: impl AsRef<Path>) -> Result<Vec<NamedFile>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut archive = ZipArchive::new(reader)?;

    (0..archive.len()).map(|index| {
        let mut file = archive.by_index_decrypt(index, PASS.as_bytes())??;
        let mut bytes = Vec::with_capacity(file.size() as usize);
        file.read_to_end(&mut bytes)?;
        Ok((file.name().to_owned(), bytes))
    }).collect()
}
