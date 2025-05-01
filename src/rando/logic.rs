use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    result::Result as StdResult,
    str::{FromStr, Split},
};

use indexmap::IndexSet;
use itertools::Itertools;
use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer,
};
use strum::{EnumDiscriminants, EnumString, VariantArray};

use crate::{
    helpers::file_open,
    map::{Collectible, Door, Gear, Map, Sprite, Things},
    Result,
};

use super::{id::Id, pool::Pool};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Logic {
    pub areas: HashMap<String, Area>,
}

impl Logic {
    pub fn parse() -> Result<Self> {
        let file = file_open("rando/logic.json")?;
        let logic = serde_json::from_reader(file)?;
        Ok(logic)
    }

    pub fn purge_doors(&mut self, maps: &[Map]) {
        for area in self.areas.values_mut() {
            area.items.retain(|item, _| {
                let sprite = item.expect_sprite(maps);
                match sprite.kind.into() {
                    Sprite::Collectible(_) | Sprite::Gear(_) => true,
                    Sprite::Door(Door::Gold | Door::Silver | Door::Boulder)
                    | Sprite::Things(Things::NGPBoulder) => false,
                    other => panic!("unexpected {other:?} as item {item}"),
                }
            });
        }
    }

    pub fn items(&self) -> impl Iterator<Item = &Id> {
        self.areas.values().flat_map(|area| area.items.keys())
    }

    pub fn transfer_groups(&self) -> Vec<Vec<(&String, Id)>> {
        let mut remaining = self.areas.keys().collect::<HashSet<_>>();
        let mut groups = vec![];

        while let Some(&key) = remaining.iter().next() {
            let mut paths = vec![key];
            let mut group = vec![];

            while let Some(path) = paths.pop() {
                if !remaining.contains(path) {
                    continue;
                }
                remaining.remove(path);

                let area = self.get_area(path);
                paths.extend(area.paths.keys());
                group.extend(area.transfers.iter().map(|transfer| (path, *transfer)));
            }

            if !group.is_empty() {
                groups.push(group);
            }
        }

        groups
    }

    fn get_area(&self, path: &str) -> &Area {
        self.areas
            .get(path)
            .unwrap_or_else(|| panic!("path target \"{path}\" doesn't exist"))
    }
}

pub struct Reach<'logic> {
    logic: &'logic Logic,
    transfers: HashMap<Id, &'logic String>,
    door_requirements: HashMap<Id, Option<Sprite>>,
}

impl<'logic> Reach<'logic> {
    pub fn new(
        logic: &'logic Logic,
        transfers: HashMap<Id, &'logic String>,
        door_requirements: HashMap<Id, Option<Sprite>>,
    ) -> Self {
        Self {
            logic,
            transfers,
            door_requirements,
        }
    }

    pub fn reach(&self, pool: &Pool) -> IndexSet<Id> {
        let mut paths = vec!["Spawn"];
        let mut visited = HashSet::new();
        let mut reached = IndexSet::new();

        while let Some(path) = paths.pop() {
            if visited.contains(path) {
                continue;
            }
            visited.insert(path);

            let area = self.logic.get_area(path);

            for (item, requirements) in &area.items {
                if self.is_met(requirements, pool) {
                    reached.insert(*item);
                }
            }

            for (to, requirements) in &area.paths {
                if self.is_met(requirements, pool) {
                    paths.push(to);
                }
            }

            for transfer in &area.transfers {
                paths.push(self.transfers[transfer]);
            }
        }

        reached
    }

    fn is_met(&self, requirements: &Requirements, pool: &Pool) -> bool {
        requirements.iter().any(|requirements| {
            requirements.iter().all(|requirement| {
                let items = self.required_items(requirement);
                pool.contains_all(&items)
            })
        })
    }

    fn required_items(&self, requirement: &Requirement) -> Vec<Sprite> {
        match requirement {
            Requirement::Ring => vec![Sprite::Gear(Gear::WindRing)],
            Requirement::Charm => vec![Sprite::Gear(Gear::LavaCharm)],
            Requirement::Swords(amount) => {
                vec![Sprite::Collectible(Collectible::Sword); *amount as usize]
            }
            Requirement::Gems(amount) => {
                vec![Sprite::Collectible(Collectible::Gem); *amount as usize]
            }
            Requirement::Id(id) => self
                .door_requirements
                .get(id)
                .unwrap_or_else(|| panic!("Unknown requirement {id}"))
                .into_iter()
                .copied()
                .collect(),
        }
    }
}

#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Area {
    pub items: HashMap<Id, Requirements>,
    pub paths: HashMap<String, Requirements>,
    pub transfers: Vec<Id>,
}

type Requirements = Vec<Vec<Requirement>>;

#[derive(EnumDiscriminants)]
#[strum_discriminants(derive(VariantArray, EnumString))]
pub enum Requirement {
    Ring,
    Charm,
    Swords(u8),
    Gems(u8),
    Id(Id),
}

impl RequirementDiscriminants {
    fn describe_format(&self) -> &str {
        match self {
            Self::Ring => "Ring",
            Self::Charm => "Charm",
            Self::Swords => "Swords.<amount>",
            Self::Gems => "Gems.<amount>",
            Self::Id => "<map>.<x>.<y>",
        }
    }
}

impl<'de> Deserialize<'de> for Requirement {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        fn invalid_variant<'de, D>(unexp: &str) -> D::Error
        where
            D: Deserializer<'de>,
        {
            Error::invalid_value(
                Unexpected::Str(unexp),
                &RequirementDiscriminants::VARIANTS
                    .iter()
                    .map(RequirementDiscriminants::describe_format)
                    .format_with(" or ", |variant, f| f(&format_args!("\"{variant}\"")))
                    .to_string()
                    .as_str(),
            )
        }

        fn extra_part<'de, T, D>(parts: &mut Split<char>, unexp: &str) -> StdResult<T, D::Error>
        where
            T: FromStr,
            T::Err: Display,
            D: Deserializer<'de>,
        {
            let part = parts.next().ok_or_else(|| invalid_variant::<D>(unexp))?;
            part.parse()
                .map_err(|err| Error::custom(format!("invalid part \"{part}\": {err}")))
        }

        let str = String::deserialize(deserializer)?;

        let mut parts = str.split('.');
        let variant = parts.next().unwrap();

        let requirement = match variant.parse() {
            Ok(RequirementDiscriminants::Ring) => Requirement::Ring,
            Ok(RequirementDiscriminants::Charm) => Requirement::Charm,
            Ok(RequirementDiscriminants::Swords) => {
                let amount = extra_part::<_, D>(&mut parts, &str)?;
                Requirement::Swords(amount)
            }
            Ok(RequirementDiscriminants::Gems) => {
                let amount = extra_part::<_, D>(&mut parts, &str)?;
                Requirement::Gems(amount)
            }
            Ok(RequirementDiscriminants::Id) | Err(_) => {
                let id = str.parse().map_err(|_| invalid_variant::<D>(&str))?;
                Requirement::Id(id)
            }
        };

        Ok(requirement)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_logic() {
        use crate::rom::{Rom, RomReader};

        let logic = Logic::parse().unwrap();

        let mut valid = true;

        let mut reader = RomReader::open("Roms/main.hsrom".into()).unwrap();
        let rom = Rom::parse(&mut reader);

        for (name, area) in &logic.areas {
            if area.items.is_empty() && area.transfers.is_empty() {
                eprintln!("{name} has no items or transfers");
                valid = false;
            }

            for path in area.paths.keys() {
                if !logic.get_area(path).paths.contains_key(name) {
                    eprintln!("{name} -> {path} exists but {path} -> {name} doesn't");
                    valid = false;
                }
            }
        }

        for map in rom.maps.unwrap() {
            if !(42..=45).contains(&map.identifier) {
                continue;
            }

            for (x, y, sprite) in map.sprites_with_positions() {
                let id = Id {
                    map: map.identifier,
                    x,
                    y,
                };

                if should_not_map(id) {
                    continue;
                }

                let mapped = match sprite.kind.into() {
                    Sprite::Collectible(_) | Sprite::Gear(_) | Sprite::Door(_) => logic
                        .areas
                        .values()
                        .flat_map(|area| area.items.keys())
                        .contains(&id),
                    Sprite::Things(Things::Transfer) => logic
                        .areas
                        .values()
                        .flat_map(|area| &area.transfers)
                        .contains(&id),
                    _ => true,
                };

                if !mapped {
                    eprintln!("{id} not mapped in logic");
                    valid = false;
                }
            }
        }

        assert!(valid);
    }

    fn should_not_map(id: Id) -> bool {
        matches!(
            id,
            Id {
                map: 43,
                x: 31,
                y: 1
            } | Id {
                map: 43,
                x: 32,
                y: 1
            }
        )
    }
}
