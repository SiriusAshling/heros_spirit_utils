use std::collections::{HashMap, HashSet};

use indexmap::IndexSet;
use serde::Deserialize;

use crate::{
    helpers::file_open,
    map::{Door, Gear, Map, Sprite, Things},
    Result,
};

use super::{id::Id, pool::Pool};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Logic {
    areas: HashMap<String, Area>,
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
                    Sprite::Collectible(_) => true,
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
            remaining.remove(key);

            let mut paths = vec![key];
            let mut visited = HashSet::new();
            let mut group = vec![];

            while let Some(path) = paths.pop() {
                if visited.contains(path) {
                    continue;
                }
                visited.insert(path);

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
                self.requirement(requirement)
                    .is_none_or(|sprite| pool.contains(&sprite))
            })
        })
    }

    fn requirement(&self, requirement: &Requirement) -> Option<Sprite> {
        match requirement {
            Requirement::Ring => Some(Sprite::Gear(Gear::WindRing)),
            Requirement::Id(id) => *self
                .door_requirements
                .get(id)
                .unwrap_or_else(|| panic!("Unknown requirement {id}")),
        }
    }
}

#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct Area {
    items: HashMap<Id, Requirements>,
    paths: HashMap<String, Requirements>,
    transfers: Vec<Id>,
}

type Requirements = Vec<Vec<Requirement>>;

#[derive(Deserialize)]
enum Requirement {
    Ring,
    #[serde(untagged)]
    Id(Id),
}
