use std::collections::HashMap;

use indexmap::IndexSet;
use rand_pcg::Pcg64Mcg;

use crate::{
    helpers::RemoveRandom,
    map::{Door, Gear, Map, Sprite, Things},
};

use super::{
    id::Id,
    logic::{Logic, Reach},
    pool::Pool,
    seed::Seed,
    transfers::generate_transfers,
};

pub struct Generator<'logic> {
    rng: Pcg64Mcg,
    reach: Reach<'logic>,
    pool: Pool,
    needs_placement: IndexSet<Id>,
    seed: Seed,
}

impl<'logic> Generator<'logic> {
    pub fn new(maps: &[Map], logic: &'logic Logic, mut rng: Pcg64Mcg) -> Self {
        let mut builder = GeneratorBuilder::new();

        for map in maps {
            builder.add_transfer_targets(map);
            builder.add_door_requirements(map);
        }

        let (logic_transfers, map_transfers) = generate_transfers(logic, &mut rng);
        let reach = Reach::new(logic, logic_transfers, builder.door_requirements);
        let pool = Pool::new(logic, maps);
        let needs_placement = reach.reach(&pool);

        let mut generator = Generator {
            rng,
            reach,
            pool,
            needs_placement,
            seed: Seed::new(map_transfers),
        };

        for item in logic.items() {
            if !generator.needs_placement.contains(item) {
                eprintln!("item {item} is unreachable");
                generator.place_unreachable(*item);
            }
        }

        generator
    }

    fn place_unreachable(&mut self, location: Id) {
        let sprite = self.pool.choose_remove(&mut self.rng);
        self.seed.placements.push((location, sprite));
    }

    pub fn finished(&self) -> bool {
        self.pool.is_empty()
    }

    pub fn place_random(&mut self) {
        let reach = self.reach.reach(&self.pool);
        let mut options = self
            .needs_placement
            .intersection(&reach)
            .copied()
            .collect::<Vec<_>>();

        loop {
            let sprite = self.pool.choose_remove(&mut self.rng);
            assert!(!options.is_empty(), "ran out of locations");
            let location = options.choose_remove(&mut self.rng);
            self.commit_placement(location, sprite);

            if self.finished() || matches!(sprite, Sprite::Gear(_)) {
                break;
            }
        }
    }

    pub fn fill_unreachable(&mut self) {
        for location in self.unreachable() {
            let sprite = self.pool.choose_remove_filler(&mut self.rng);
            self.commit_placement(location, sprite);
        }
    }

    pub fn finish(self) -> Seed {
        self.seed
    }

    fn unreachable(&self) -> Vec<Id> {
        let reach = self.reach.reach(&self.pool);
        self.needs_placement.difference(&reach).copied().collect()
    }

    fn commit_placement(&mut self, location: Id, sprite: Sprite) {
        self.needs_placement.swap_remove(&location);
        self.seed.placements.push((location, sprite));
    }
}

#[derive(Default)]
struct GeneratorBuilder {
    transfer_targets: HashMap<Id, Id>,
    door_requirements: HashMap<Id, Option<Sprite>>,
}

impl GeneratorBuilder {
    fn new() -> Self {
        Self::default()
    }

    // TODO finish implementing
    fn add_transfer_targets(&mut self, map: &Map) {
        let map_id = map.identifier;

        for (x, y, sprite) in map.sprites_with_positions() {
            if let Sprite::Things(Things::Transfer) = sprite.kind.into() {
                let transfer = Id { map: map_id, x, y };
                let target = Id {
                    map: sprite.extra_bytes[0],
                    x: sprite.extra_bytes[1] as usize,
                    y: sprite.extra_bytes[2] as usize,
                };
                self.transfer_targets.insert(transfer, target);
            }
        }
    }

    fn add_door_requirements(&mut self, map: &Map) {
        let map_id = map.identifier;

        for (x, y, sprite) in map.sprites_with_positions() {
            match sprite.kind.into() {
                Sprite::Door(Door::Boulder) | Sprite::Things(Things::NGPBoulder) => {
                    self.add_door_requirement(x, y, map_id, Some(Sprite::Gear(Gear::Hammer)));
                }
                Sprite::Collectible(_) | Sprite::Door(Door::Gold | Door::Silver) => {
                    self.add_door_requirement(x, y, map_id, None);
                }
                _ => {}
            }
        }
    }

    fn add_door_requirement(&mut self, x: usize, y: usize, map: u8, sprite: Option<Sprite>) {
        let id = Id { map, x, y };
        self.door_requirements.insert(id, sprite);
    }
}
