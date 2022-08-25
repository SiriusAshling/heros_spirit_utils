use std::error::Error;
use std::io::{BufWriter, Cursor};
use std::str::FromStr;

use image::{ImageFormat, RgbaImage};

use crate::data::TERRAIN_FLAGS;
use crate::{draw, map};
use crate::error::SimpleError;
use crate::map::{Map};
use crate::graphics::TileData;
use crate::sprite::{SpriteData, Sprite};

const TILE_OFFSET: u16 = 1;
const SPRITE_OFFSET: u16 = TILE_OFFSET + u8::MAX as u16 + 1;

impl Map {
    pub fn to_tmx(&self, tile_data: &TileData) -> Result<String, Box<dyn Error>> {
        let width = self.tiles[0].len();
        let height = self.tiles.len();

        let tmx = format!(include_str!("tiled_map_template.xml"),
            width,
            height,
            self.tiles_tileset(tile_data)?,
            self.sprite_tileset(tile_data)?,
            self.tiles_tiledata(),
            self.sprites_tiledata()
        );

        Ok(tmx)
    }

    pub fn from_tmx(identifier: u8, tmx: &str) -> Result<Self, Box<dyn Error>> {
        let (width, height) = Self::size_from_tmx(tmx)?;
        let tiles = Self::tiles_from_tmx(tmx, width)?;
        let offset = Self::first_gids_from_tmx(tmx, "Sprites")?;
        let sprites = Self::sprites_from_tmx(tmx, width, height, offset)?;

        Ok(Map { identifier, tiles, sprites })
    }

    fn tiles_tileset(&self, tile_data: &TileData) -> Result<String, Box<dyn Error>> {
        let variants = if self.identifier == map::GLITCH { 1..TERRAIN_FLAGS.len() as u8 - 3 } else { 1..TERRAIN_FLAGS.len() as u8 };
        self.tileset(variants, draw::draw_tile, TILE_OFFSET, "Tiles", tile_data)
    }

    fn sprite_tileset(&self, tile_data: &TileData) -> Result<String, Box<dyn Error>> {
        self.tileset(u8::MIN..u8::MAX, draw::draw_sprite, SPRITE_OFFSET, "Sprites", tile_data)
    }

    fn tileset(&self, variants: impl IntoIterator<Item = u8>, draw_fn: impl Fn(u8, u8, &TileData) -> RgbaImage, offset: u16, name: &str, tile_data: &TileData) -> Result<String, Box<dyn Error>> {
        let tiles_tmx = variants.into_iter().map(|id| {
            let image = draw_fn(id, self.identifier, tile_data);
            let mut bytes = BufWriter::new(Cursor::new(Vec::new()));
            image.write_to(&mut bytes, ImageFormat::Bmp)?;
            let image_string = base64::encode(bytes.buffer());
            let name = format!("{:?}", Sprite::from(id));
            let tmx = format!(include_str!("tiled_tile_template.xml"), id, image_string, name);
            Ok(tmx)
        }).collect::<Result<String, Box<dyn Error>>>()?;

        let tmx = format!(include_str!("tiled_tileset_template.xml"), offset, name, tiles_tmx);
        Ok(tmx)
    }

    fn tiles_tiledata(&self) -> String {
        let mut tiles = self.tiles.iter().flatten().map(|&tile| (tile as u16 + TILE_OFFSET).to_string()).collect::<Vec<_>>();
        // Some maps are missing the last tile
        if tiles.len() != self.tiles.len() * self.tiles[0].len() {
            tiles.push("0".to_string());
        }
        tiles.join(",")
    }

    fn sprites_tiledata(&self) -> String {
        self.sprites.iter().enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().filter_map(move |(x, sprite)| sprite.as_ref().map(|s| (x, y, s))))
            .map(|(x, y, sprite)| {
                let byte_properties = sprite.extra_bytes.iter().enumerate().map(|(index, byte)| format!(include_str!("tiled_property_template.xml"), index + 1, byte)).collect::<String>();
                let (_, height) = Sprite::from(sprite.kind).tile_size();
                format!(include_str!("tiled_object_template.xml"), x * 16, (y + height as usize) * 16, sprite.kind as u16 + SPRITE_OFFSET, byte_properties)
            }).collect()
    }

    fn size_from_tmx(tmx: &str) -> Result<(usize, usize), Box<dyn Error>> {
        let tags_start = tmx.find("<map").ok_or(SimpleError("Failed to find map element"))?;
        let tags_end = tmx[tags_start..].find('>').ok_or(SimpleError("Failed to find map element"))? + tags_start;
        let tags = tmx[tags_start..tags_end].split(' ').collect::<Vec<_>>();
        let width = tags.iter().find_map(|tag| tag.strip_prefix("width=\"")).and_then(|s| s.strip_suffix('"'))
            .ok_or(SimpleError("Failed to read width from map"))?.parse()?;
        let height = tags.iter().find_map(|tag| tag.strip_prefix("height=\"")).and_then(|s| s.strip_suffix('"'))
            .ok_or(SimpleError("Failed to read height from map"))?.parse()?;

        Ok((width, height))
    }

    // Tiled tries to be smart and replaces our gids with different ones <.<
    fn first_gids_from_tmx(tmx: &str, tileset_name: &str) -> Result<u16, Box<dyn Error>> {
        let first_gid = tmx.match_indices("<tileset").find_map(|(tags_start, _)| {
            let tags_end = tmx[tags_start..].find('>')? + tags_start;
            let tags = tmx[tags_start..tags_end].split(' ').collect::<Vec<_>>();
            let name = tags.iter().find_map(|tag| tag.strip_prefix("name=\""))?.strip_suffix('"')?;
            if name != tileset_name { return None }
            let first_gid = tags.iter().find_map(|tag| tag.strip_prefix("firstgid=\""))?.strip_suffix('"')?;
            first_gid.parse().ok()
        }).ok_or(SimpleError("Failed to read Sprite tileset from map"))?;

        Ok(first_gid)
    }

    fn tiles_from_tmx(tmx: &str, map_width: usize) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
        let tiles = tmx.match_indices("<layer").find_map(|(tags_start, _)| {
            let tags_end = tmx[tags_start..].find('>')? + tags_start;
            let name = tmx[tags_start..tags_end].split(' ').find_map(|tag| tag.strip_prefix("name=\""))?.strip_suffix('"')?;
            if name != "Tiles" { return None }
            let content_start = tags_end + 1;
            let content_end = tmx[content_start..].find("</layer>")? + content_start;
            let content = tmx[content_start..content_end].trim().strip_prefix("<data encoding=\"csv\">")?.strip_suffix("</data>")?.trim();
            let ids = content.split(',').map(str::trim).map(u16::from_str).collect::<Result<Vec<_>, _>>().ok()?;
            Some(ids)
        }).ok_or(SimpleError("Failed to read Tiles layer"))?.into_iter()
            .filter(|tile| *tile != 0)  // filter out filler tiles that were added because of missing tiles in the map
            .map(|tile| (tile - TILE_OFFSET) as u8).collect::<Vec<_>>()
            .chunks(map_width).map(<[_]>::to_vec).collect();
        Ok(tiles)
    }

    fn sprites_from_tmx(tmx: &str, width: usize, height: usize, offset: u16) -> Result<Vec<Vec<Option<SpriteData>>>, Box<dyn Error>> {
        let mut sprites = vec![vec![None; width]; height];

        let sprite_data = tmx.match_indices("<objectgroup").find_map(|(tags_start, _)| {
            let tags_end = tmx[tags_start..].find('>')? + tags_start;
            let (tags, has_properties) = tmx[tags_start..tags_end].strip_suffix('/').map_or((&tmx[tags_start..tags_end], true), |s| (s, false));
            let name = tags.split(' ').find_map(|tag| tag.strip_prefix("name=\""))?.strip_suffix('"')?;
            if name != "Sprites" { return None }
            let sprites = if has_properties {
                let content_start = tags_end + 1;
                let content_end = tmx[content_start..].find("</objectgroup>")? + content_start;
                tmx[content_start..content_end].trim().split("<object").filter_map(|object| {
                    let tags_end = object.find('>')?;
                    let (tags, has_properties) = object[..tags_end].strip_suffix('/').map_or((&object[..tags_end], true), |s| (s, false));
                    let tags = tags.split(' ').collect::<Vec<_>>();
                    let gid = tags.iter().find_map(|tag| tag.strip_prefix("gid=\""))?.strip_suffix('"')?.parse::<u16>().ok()?;
                    let kind = (gid - offset) as u8;
                    let (_, height) = Sprite::from(kind).tile_size();
                    let x = tags.iter().find_map(|tag| tag.strip_prefix("x=\""))?.strip_suffix('"')?.parse::<f32>().ok()?;
                    let x = (x / 16.0).round() as usize;
                    let y = tags.iter().find_map(|tag| tag.strip_prefix("y=\""))?.strip_suffix('"')?.parse::<f32>().ok()?;
                    let y = (y / 16.0).round() as usize - height as usize;
                    let extra_bytes = if has_properties {
                        let content_start = tags_end + 1;
                        let content_end = object[content_start..].find("</object>")? + content_start;
                        let content = object[content_start..content_end].trim().strip_prefix("<properties>")?.strip_suffix("</properties>")?.trim();
                        let mut extra_bytes = content.split("<property").filter_map(|byte| {
                            let tags_end = byte.find("/>")?;
                            let tags = byte[..tags_end].split(' ').collect::<Vec<_>>();
                            let name = tags.iter().find_map(|tag| tag.strip_prefix("name=\""))?.strip_suffix('"')?;
                            let index = name.strip_prefix("byte_")?.parse::<u8>().ok()?;
                            let value = tags.iter().find_map(|tag| tag.strip_prefix("value=\""))?.strip_suffix('"')?.parse().ok()?;
                            Some((index, value))
                        }).collect::<Vec<_>>();
                        extra_bytes.sort_unstable_by_key(|(index, _)| *index);
                        extra_bytes.into_iter().map(|(_, value)| value).collect()
                    } else { Vec::new() };

                    Some((x, y, kind, extra_bytes))
                }).collect()
            } else { Vec::new() };

            Some(sprites)
        }).ok_or(SimpleError("Failed to read Sprites layer"))?;

        for (x, y, kind, extra_bytes) in sprite_data {
            sprites[y][x] = Some(SpriteData { kind, extra_bytes })
        }

        Ok(sprites)
    }
}
