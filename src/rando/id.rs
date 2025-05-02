use std::{
    error::Error,
    fmt::{self, Display},
    str::{FromStr, SplitN},
};

use serde::Deserialize;

use crate::{
    map::{Map, SpriteData},
    Result,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(try_from = "String")]
pub struct Id {
    pub map: u8,
    pub x: usize,
    pub y: usize,
}

impl Id {
    pub const fn new(map: u8, x: usize, y: usize) -> Self {
        Self { map, x, y }
    }

    pub const fn is_excluded(self) -> bool {
        const INNER_CASTLE_GUARD_LEFT: Id = Id::new(43, 45, 56);
        const INNER_CASTLE_GUARD_RIGHT: Id = Id::new(43, 46, 56);
        const FINAL_TRANSFER_LEFT: Id = Id::new(43, 31, 1);
        const FINAL_TRANSFER_RIGHT: Id = Id::new(43, 32, 1);
        const FINAL_GUARD_LEFT: Id = Id::new(43, 31, 3);
        const FINAL_GUARD_RIGHT: Id = Id::new(43, 32, 3);

        matches!(
            self,
            INNER_CASTLE_GUARD_LEFT
                | INNER_CASTLE_GUARD_RIGHT
                | FINAL_TRANSFER_LEFT
                | FINAL_TRANSFER_RIGHT
                | FINAL_GUARD_LEFT
                | FINAL_GUARD_RIGHT
        )
    }

    pub fn expect_map(self, maps: &[Map]) -> &Map {
        maps.iter()
            .find(|m| m.identifier == self.map)
            .unwrap_or_else(|| self.unknown_map_identifier())
    }

    pub fn expect_map_mut(self, maps: &mut [Map]) -> &mut Map {
        maps.iter_mut()
            .find(|m| m.identifier == self.map)
            .unwrap_or_else(|| self.unknown_map_identifier())
    }

    pub fn sprite(self, maps: &[Map]) -> Option<&SpriteData> {
        self.expect_map(maps).sprite(self.x, self.y)
    }

    pub fn expect_sprite(self, maps: &[Map]) -> &SpriteData {
        self.sprite(maps).unwrap_or_else(|| self.failed_to_match())
    }

    pub fn sprite_mut(self, maps: &mut [Map]) -> Option<&mut SpriteData> {
        self.expect_map_mut(maps).sprite_mut(self.x, self.y)
    }

    pub fn expect_sprite_mut(self, maps: &mut [Map]) -> &mut SpriteData {
        self.sprite_mut(maps)
            .unwrap_or_else(|| self.failed_to_match())
    }

    fn unknown_map_identifier(self) -> ! {
        panic!("unknown map identifier {}", self.map)
    }

    fn failed_to_match(self) -> ! {
        panic!("failed to match {self} to map")
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.map, self.x, self.y)
    }
}

impl FromStr for Id {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        fn parse_part<T>(parts: &mut SplitN<char>, s: &str) -> Result<T>
        where
            T: FromStr,
            T::Err: Display,
        {
            let part = parts
                .next()
                .ok_or_else(|| "expected string with format <map>.<x>.<y>".to_string())?;
            let part = part
                .parse()
                .map_err(|err| format!("invalid part \"{part}\" in \"{s}\": {err}"))?;
            Ok(part)
        }

        let mut parts = s.splitn(4, '.');
        let map = parse_part(&mut parts, s)?;
        let x = parse_part(&mut parts, s)?;
        let y = parse_part(&mut parts, s)?;

        if let Some(remaining) = parts.next() {
            Err(format!("trailing symbols \"{remaining}\" in \"{s}\""))?;
        }

        Ok(Id { map, x, y })
    }
}

impl TryFrom<&str> for Id {
    type Error = <Id as FromStr>::Err;

    fn try_from(value: &str) -> Result<Self> {
        value.parse()
    }
}

impl TryFrom<String> for Id {
    type Error = <Id as FromStr>::Err;

    fn try_from(value: String) -> Result<Self> {
        value.parse()
    }
}
