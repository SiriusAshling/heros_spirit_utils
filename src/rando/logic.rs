use std::{
    collections::{HashMap, HashSet},
    ops::{Deref, DerefMut},
    result::Result as StdResult,
};

use indexmap::{IndexMap, IndexSet};
use itertools::Itertools;
use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer,
};
use strum::{EnumDiscriminants, EnumString, VariantArray};

use crate::{
    helpers::file_open,
    map::{Collectible, Door, Gear, Map, Sprite},
    Result,
};

use super::{id::Id, pool::Pool};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Logic {
    pub areas: IndexMap<String, Area>,
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
                    Sprite::Door(Door::Gold | Door::Silver | Door::Boulder) => false,
                    other => panic!("unexpected {other:?} as item {item}"),
                }
            });
        }
    }

    pub fn items(&self) -> impl Iterator<Item = Id> + use<'_> {
        self.areas
            .values()
            .flat_map(|area| area.items.keys())
            .copied()
    }

    pub fn transfer_groups(&self) -> Vec<Vec<(&String, Id)>> {
        let mut remaining = self.areas.keys().collect::<IndexSet<_>>();
        let mut groups = vec![];

        while let Some(&key) = remaining.iter().next() {
            let mut paths = vec![key];
            let mut group = vec![];

            while let Some(path) = paths.pop() {
                if !remaining.contains(path) {
                    continue;
                }
                remaining.swap_remove(path);

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
    requirement_map: RequirementMap,
}

impl<'logic> Reach<'logic> {
    pub fn new(
        logic: &'logic Logic,
        transfers: HashMap<Id, &'logic String>,
        requirement_map: RequirementMap,
    ) -> Self {
        Self {
            logic,
            transfers,
            requirement_map,
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
                let items = self.requirement_map.required_items(requirement);
                items.iter().any(|items| pool.contains_all(items))
            })
        })
    }
}

pub struct RequirementMap {
    ring_requirement: Vec<Vec<Sprite>>,
    charm_requirement: Vec<Vec<Sprite>>,
    shield_requirement: Vec<Vec<Sprite>>,
    id_requirements: HashMap<Id, Vec<Vec<Sprite>>>,
}

impl RequirementMap {
    pub fn new(logic: &Logic) -> Self {
        Self {
            ring_requirement: vec![vec![Sprite::Gear(Gear::WindRing)]],
            charm_requirement: vec![vec![Sprite::Gear(Gear::LavaCharm)]],
            shield_requirement: vec![vec![Sprite::Collectible(Collectible::Shield)]],
            id_requirements: logic.items().map(|id| (id, vec![vec![]])).collect(),
        }
    }

    fn required_items(&self, requirement: &Requirement) -> &Vec<Vec<Sprite>> {
        match requirement {
            Requirement::Ring => &self.ring_requirement,
            Requirement::Charm => &self.charm_requirement,
            Requirement::Shield => &self.shield_requirement,
            Requirement::Id(id) => self
                .id_requirements
                .get(id)
                .unwrap_or_else(|| panic!("Unknown requirement {id}")),
        }
    }
}

impl Deref for RequirementMap {
    type Target = HashMap<Id, Vec<Vec<Sprite>>>;

    fn deref(&self) -> &Self::Target {
        &self.id_requirements
    }
}

impl DerefMut for RequirementMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.id_requirements
    }
}

#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Area {
    pub items: IndexMap<Id, Requirements>,
    pub paths: IndexMap<String, Requirements>,
    pub transfers: Vec<Id>,
}

type Requirements = Vec<Vec<Requirement>>;

#[derive(EnumDiscriminants)]
#[strum_discriminants(derive(VariantArray, EnumString))]
pub enum Requirement {
    Ring,
    Charm,
    Shield,
    Id(Id),
}

impl RequirementDiscriminants {
    fn describe_format(&self) -> &str {
        match self {
            Self::Ring => "Ring",
            Self::Charm => "Charm",
            Self::Shield => "Shield",
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

        let str = String::deserialize(deserializer)?;

        let mut parts = str.split('.');
        let variant = parts.next().unwrap();

        let requirement = match variant.parse() {
            Ok(RequirementDiscriminants::Ring) => Requirement::Ring,
            Ok(RequirementDiscriminants::Charm) => Requirement::Charm,
            Ok(RequirementDiscriminants::Shield) => Requirement::Shield,
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
    use rand_pcg::Pcg64Mcg;

    use crate::{map::Things, rando::generator::Generator};

    use super::*;

    #[test]
    fn validate_logic() {
        use crate::rom::{Rom, RomReader};

        let mut logic = Logic::parse().unwrap();

        let mut valid = true;

        let mut reader = RomReader::open("Roms/main.hsrom".into()).unwrap();
        let rom = Rom::parse(&mut reader);

        for (name, area) in &logic.areas {
            let mut ids = area.items.keys().chain(&area.transfers);
            match ids.next() {
                None => {
                    eprintln!("{name} has no items or transfers");
                    valid = false;
                }
                Some(first) => {
                    for id in ids {
                        if id.map != first.map {
                            eprintln!("{name} contains items or transfers in multiple maps: {first} and {id}");
                            valid = false;
                        }
                    }
                }
            }

            for path in area.paths.keys() {
                if !logic.get_area(path).paths.contains_key(name) {
                    eprintln!("{name} -> {path} exists but {path} -> {name} doesn't");
                    valid = false;
                }
            }
        }

        let maps = rom.maps.unwrap();

        for map in &maps {
            if !(42..=45).contains(&map.identifier) {
                continue;
            }

            for (x, y, sprite) in map.sprites_with_positions() {
                let id = Id::new(map.identifier, x, y);

                if id.is_excluded() {
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

        logic.purge_doors(&maps);

        Generator::new(&maps, &logic, &mut Pcg64Mcg::new(0)).unwrap();

        assert!(valid);
    }
}
