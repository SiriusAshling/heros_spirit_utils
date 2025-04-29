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
    pub fn sprite(self, maps: &[Map]) -> &Option<SpriteData> {
        let map = maps.iter().find(|m| m.identifier == self.map).unwrap();
        &map.sprites[self.y][self.x]
    }

    pub fn expect_sprite(self, maps: &[Map]) -> &SpriteData {
        self.sprite(maps)
            .as_ref()
            .unwrap_or_else(|| panic!("failed to match item {self} to map"))
    }

    pub fn sprite_mut(self, maps: &mut [Map]) -> &mut Option<SpriteData> {
        let map = maps.iter_mut().find(|m| m.identifier == self.map).unwrap();
        &mut map.sprites[self.y][self.x]
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
