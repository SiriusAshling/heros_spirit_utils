use std::{path::Path, error::Error, fs};

use crate::{map::Map, data::TERRAIN_FLAGS};

fn decode_map<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
    let data = fs::read(&path)?;
    let map_id = data[0];
    let width = data[1] as usize;
    let height = data[2] as usize;

    let tiles_end = 3 + width * height * 7 / 8;
    let tile_bytes = &data[3..tiles_end];
    let objects = &data[tiles_end..];

    let tile_bytes_len = tile_bytes.len();
    let mut tile_bits = Vec::with_capacity(tile_bytes_len * 8);

    for (index, byte) in tile_bytes.into_iter().enumerate() {
        let mut bits = Vec::with_capacity(8);

        for bit_index in 0..8 {
            let bit = byte & (1 << bit_index) != 0;
            let position = (10963 * map_id as usize + index * 8) % (1 + bit_index);
            bits.insert(position, bit);
        }

        tile_bits.append(&mut bits);
    }

    let mut tile_chunks = tile_bits.chunks(7);

    let mut read_tile = || {
        tile_chunks.next().and_then(|tile_bits| {
            if tile_bits.len() < 7 {
                return None;
            }

            let mut tile = 0u8;
            for bit_index in 0..7 {
                let bit = tile_bits[bit_index];
                if bit {
                    tile |= 1 << bit_index;
                }
            }
            Some(tile)
        })
    };

    let mut tiles = Vec::with_capacity(height);
    for _ in 0..height {
        let mut row = Vec::with_capacity(width);

        for _ in 0..width {
            if let Some(tile) = read_tile() {
                row.push(tile);
            }
        }

        tiles.push(row);
    }

    let display = tiles.into_iter().map(|row| format!("{:?}", row)).collect::<Vec<_>>().join("\n");

    let mut path = path.as_ref()
        .parent().unwrap()
        .parent().unwrap()
        .to_owned();
    fs::create_dir("maps");
    path.push("maps");
    path.push(format!("{:?}", Map::from(map_id)));
    fs::write(&path, format!("{}", display))?;
    return Ok(());

// 
//     let mut terrain = vec![vec![0; width]; height];
//     let len = TERRAIN_FLAGS.len();
// 
//     'outer: for y in 0..height {
//         for x in 0..width {
//             if let Some(mut tile) = tiles.pop() {
//                 if map_id == 13 {
//                     tile += 3;
//                 }
//                 let tile = tile as usize;
//                 if len > tile {
//                     terrain[y][x] = TERRAIN_FLAGS[tile];
//                 }
//             } else { break 'outer }
//         }
//     }
// 
//     let binary = |byte: u8| -> String {
//         let mut bits = String::with_capacity(8);
//         for bit_index in 0..8 {
//             let bit = byte & (1 << (7 - bit_index)) != 0;
//             bits.push(if bit { '1' } else { '0' });
//         }
//         bits
//     };
//     let display = terrain.into_iter().map(|row| row.into_iter().map(binary).collect::<Vec<_>>().join(", ")).collect::<Vec<_>>().join("\n");
// 
//     let mut path = path.as_ref()
//         .parent().unwrap()
//         .parent().unwrap()
//         .to_owned();
//     fs::create_dir("maps");
//     path.push("maps");
//     path.push(format!("{:?}", Map::from(map_id)));
//     fs::write(&path, format!("{}", display))?;
// 
//     Ok(())
}

pub fn decode<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
    let rom = fs::read_dir(path)?;
    for file in rom {
        let file = file?;
        let filename = file.file_name();
        let filename = filename.to_string_lossy();
        if filename.starts_with("map") {
            decode_map(file.path())?;
        }
    }

    Ok(())
}
