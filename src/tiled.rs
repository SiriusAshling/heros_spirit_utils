use std::fmt::Display;
use std::io::Cursor;
use std::str::FromStr;

use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use image::ImageFormat;
use itertools::Itertools;

use crate::data::TERRAIN_FLAGS;
use crate::graphics::TileData;
use crate::map::Map;
use crate::sprite::{Sprite, SpriteData};
use crate::Result;
use crate::{draw, map};

const TILE_OFFSET: u16 = 1;
const SPRITE_OFFSET: u16 = TILE_OFFSET + u8::MAX as u16 + 1;

impl Map {
    pub fn to_tmx(&self, tile_data: &TileData) -> Result<String> {
        let width = self.tiles[0].len();
        let height = self.tiles.len();

        let tmx = format!(
            include_str!("tiled_map_template.xml"),
            width,
            height,
            self.tiles_tileset(tile_data)?,
            self.sprite_tileset(tile_data)?,
            self.tiles_tiledata(),
            self.sprites_tiledata()
        );

        Ok(tmx)
    }

    pub fn from_tmx(identifier: u8, tmx: &str) -> Result<Self> {
        let (width, height) = Self::size_from_tmx(tmx)?;
        let tiles = Self::tiles_from_tmx(tmx, width)?;
        let offset = Self::first_gids_from_tmx(tmx, "Sprites")?;
        let sprites = Self::sprites_from_tmx(tmx, width, height, offset)?;

        Ok(Map {
            identifier,
            tiles,
            sprites,
        })
    }

    fn tiles_tileset(&self, tile_data: &TileData) -> Result<String> {
        let mut max = TERRAIN_FLAGS.len() as u8;
        if self.identifier == map::GLITCH {
            max -= 3;
        }
        let tiles_tmx = (1..max)
            .map(|id| {
                let image = draw::draw_tile(id, self.identifier, tile_data);
                let mut bytes = Cursor::new(Vec::new());
                image.write_to(&mut bytes, ImageFormat::Bmp)?;
                let image_string = BASE64_STANDARD.encode(bytes.get_ref());
                let tmx = format!(include_str!("tiled_tile_template.xml"), id, image_string);
                Ok(tmx)
            })
            .collect::<Result<String>>()?;

        Ok(format!(
            include_str!("tiled_tileset_template.xml"),
            TILE_OFFSET, "Tiles", tiles_tmx
        ))
    }

    fn sprite_tileset(&self, tile_data: &TileData) -> Result<String> {
        let tiles_tmx = (u8::MIN..u8::MAX)
            .map(|id| {
                let image = draw::draw_sprite(id, self.identifier, tile_data);
                let mut bytes = Cursor::new(Vec::new());
                image.write_to(&mut bytes, ImageFormat::Bmp)?;
                let image_string = BASE64_STANDARD.encode(bytes.get_ref());
                let name = format!("{:?}", Sprite::from(id));
                let tmx = format!(
                    include_str!("tiled_sprite_template.xml"),
                    id, image_string, name
                );
                Ok(tmx)
            })
            .collect::<Result<String>>()?;

        Ok(format!(
            include_str!("tiled_tileset_template.xml"),
            SPRITE_OFFSET, "Sprites", tiles_tmx
        ))
    }

    fn tiles_tiledata(&self) -> String {
        let mut csv = self
            .tiles
            .iter()
            .map(|col| {
                col.iter()
                    .map(|tile| u16::from(*tile) + TILE_OFFSET)
                    .format_with(", ", |tile, f| f(&format_args!("{tile:02}")))
            })
            .join(",\n");

        let map_is_missing_last_tile =
            self.tiles.first().unwrap().len() != self.tiles.last().unwrap().len();
        if map_is_missing_last_tile {
            csv.push_str(", 00");
        }

        csv
    }

    fn sprites_tiledata(&self) -> impl Display + use<'_> {
        self.sprites
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(x, sprite)| sprite.as_ref().map(|s| (x, y, s)))
            })
            .format_with("", |(x, y, sprite), f| {
                let byte_properties =
                    sprite
                        .extra_bytes
                        .iter()
                        .enumerate()
                        .format_with("", |(index, byte), f| {
                            f(&format_args!(
                                include_str!("tiled_property_template.xml"),
                                index + 1,
                                byte
                            ))
                        });
                let (_, height) = Sprite::from(sprite.kind).tile_size();
                f(&format_args!(
                    include_str!("tiled_object_template.xml"),
                    x * 16,
                    (y + height as usize) * 16,
                    u16::from(sprite.kind) + SPRITE_OFFSET,
                    byte_properties
                ))
            })
    }

    fn size_from_tmx(tmx: &str) -> Result<(usize, usize)> {
        let tags_start = tmx.find("<map").ok_or("Failed to find map element")?;
        let tags_end = tmx[tags_start..]
            .find('>')
            .ok_or("Failed to find map element")?
            + tags_start;
        let tags = tmx[tags_start..tags_end].split(' ').collect::<Vec<_>>();
        let width = tags
            .iter()
            .find_map(|tag| tag.strip_prefix("width=\""))
            .and_then(|s| s.strip_suffix('"'))
            .ok_or("Failed to read width from map")?
            .parse()?;
        let height = tags
            .iter()
            .find_map(|tag| tag.strip_prefix("height=\""))
            .and_then(|s| s.strip_suffix('"'))
            .ok_or("Failed to read height from map")?
            .parse()?;

        Ok((width, height))
    }

    // Tiled tries to be smart and replaces our gids with different ones <.<
    fn first_gids_from_tmx(tmx: &str, tileset_name: &str) -> Result<u16> {
        let first_gid = tmx
            .match_indices("<tileset")
            .find_map(|(tags_start, _)| {
                let tags_end = tmx[tags_start..].find('>')? + tags_start;
                let tags = tmx[tags_start..tags_end].split(' ').collect::<Vec<_>>();
                let name = tags
                    .iter()
                    .find_map(|tag| tag.strip_prefix("name=\""))?
                    .strip_suffix('"')?;
                if name != tileset_name {
                    return None;
                }
                let first_gid = tags
                    .iter()
                    .find_map(|tag| tag.strip_prefix("firstgid=\""))?
                    .strip_suffix('"')?;
                first_gid.parse().ok()
            })
            .ok_or("Failed to read Sprite tileset from map")?;

        Ok(first_gid)
    }

    fn tiles_from_tmx(tmx: &str, map_width: usize) -> Result<Vec<Vec<u8>>> {
        let tiles = tmx
            .match_indices("<layer")
            .find_map(|(tags_start, _)| {
                let tags_end = tmx[tags_start..].find('>')? + tags_start;
                let name = tmx[tags_start..tags_end]
                    .split(' ')
                    .find_map(|tag| tag.strip_prefix("name=\""))?
                    .strip_suffix('"')?;
                if name != "Tiles" {
                    return None;
                }
                let content_start = tags_end + 1;
                let content_end = tmx[content_start..].find("</layer>")? + content_start;
                let content = tmx[content_start..content_end]
                    .trim()
                    .strip_prefix("<data encoding=\"csv\">")?
                    .strip_suffix("</data>")?
                    .trim();
                let ids = content
                    .split(',')
                    .map(str::trim)
                    .map(u16::from_str)
                    .collect::<std::result::Result<Vec<_>, _>>()
                    .ok()?;
                Some(ids)
            })
            .ok_or("Failed to read Tiles layer")?
            .into_iter()
            .filter(|tile| *tile != 0) // filter out filler tiles that were added because of missing tiles in the map
            .map(|tile| (tile - TILE_OFFSET) as u8)
            .collect::<Vec<_>>()
            .chunks(map_width)
            .map(<[_]>::to_vec)
            .collect();
        Ok(tiles)
    }

    fn sprites_from_tmx(
        tmx: &str,
        width: usize,
        height: usize,
        offset: u16,
    ) -> Result<Vec<Vec<Option<SpriteData>>>> {
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
                    if gid < offset {
                        eprintln!("Map contains invalid sprite with gid {gid}. Did you place a tile on the sprite layer by accident?");
                        return None;
                    }
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
        }).ok_or("Failed to read Sprites layer")?;

        for (x, y, kind, extra_bytes) in sprite_data {
            sprites[y][x] = Some(SpriteData { kind, extra_bytes });
        }

        Ok(sprites)
    }
}
