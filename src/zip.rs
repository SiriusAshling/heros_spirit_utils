use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use zip::ZipArchive;

use crate::data::PASS;

pub fn read_rom(path: impl AsRef<Path>) -> Result<HashMap<String, Vec<u8>>, Box<dyn Error>> {
    let mut map = HashMap::new();

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut archive = ZipArchive::new(reader)?;
    for index in 0..archive.len() {
        let mut file = archive.by_index_decrypt(index, PASS.as_bytes())??;
        let mut bytes = Vec::with_capacity(file.size() as usize);
        file.read_to_end(&mut bytes)?;
        map.insert(file.name().to_owned(), bytes);
    }

    Ok(map)
}
