use crate::{
    map::{Map, Sprite, SpriteData, Things},
    saves::Direction,
};

use super::id::Id;

pub struct Seed {
    pub placements: Vec<(Id, Sprite)>,
    pub map_transfers: Vec<(Id, Id)>,
}

impl Seed {
    pub fn new(map_transfers: Vec<(Id, Id)>) -> Self {
        Self {
            placements: vec![],
            map_transfers,
        }
    }

    pub fn apply(self, maps: &mut [Map]) {
        for (location, sprite) in self.placements {
            *location.sprite_mut(maps) = Some(SpriteData {
                kind: sprite.into(),
                extra_bytes: vec![],
            });
        }

        for (source, target) in self.map_transfers {
            *source.sprite_mut(maps) = Some(SpriteData {
                kind: Sprite::Things(Things::Transfer).into(),
                extra_bytes: vec![
                    target.map,
                    target.x as u8,
                    target.y as u8,
                    Direction::Down as u8,
                ],
            });
        }
    }
}
