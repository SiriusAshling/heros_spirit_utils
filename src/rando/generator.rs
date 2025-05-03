// TODO investigate seed: LeatherDigressiveWitch

use indexmap::IndexSet;
use rand::{
    seq::{IteratorRandom, SliceRandom},
    SeedableRng,
};
use rand_pcg::Pcg64Mcg;

use crate::{
    helpers::RemoveRandom,
    map::{Collectible, Door, Enemy, Map, Sprite},
    Result,
};

use super::{
    id::Id,
    logic::{Logic, Reach, RequirementMap},
    pool::Pool,
    seed::Seed,
    spoiler::ItemSpoiler,
};

pub struct Generator<'logic> {
    rng: Pcg64Mcg,
    reach: Reach<'logic>,
    reach_cache: IndexSet<Id>,
    pool: Pool,
    needs_placement: IndexSet<Id>,
    seed: Seed,
    spoiler: ItemSpoiler,
}

impl<'logic> Generator<'logic> {
    pub fn new(maps: &[Map], logic: &'logic Logic, rng: &mut Pcg64Mcg) -> Result<Self> {
        let mut rng = Pcg64Mcg::from_rng(rng);

        let mut seed = Seed::default();
        let mut requirement_map = RequirementMap::new(logic);

        let logic_transfers = seed.generate_transfers(maps, logic, &mut rng);
        seed.blind_shuffle::<Door>(maps, &mut requirement_map, &mut rng);
        seed.blind_shuffle::<Enemy>(maps, &mut requirement_map, &mut rng);

        let reach = Reach::new(logic, logic_transfers, requirement_map);
        let pool = Pool::new(logic, maps);
        let reach_cache = reach.reach(&pool);
        let needs_placement = reach_cache.clone();

        let spoiler = ItemSpoiler::default();

        let mut generator = Generator {
            rng,
            reach,
            reach_cache,
            pool,
            needs_placement,
            seed,
            spoiler,
        };

        for item in logic.items() {
            if !generator.needs_placement.contains(&item) {
                eprintln!("item {item} is unreachable");
                generator.place_unreachable(item)?;
            }
        }

        Ok(generator)
    }

    fn place_unreachable(&mut self, location: Id) -> Result<()> {
        let sprite = self.pool.choose_remove_filler(&mut self.rng)?;
        self.seed.push((location, sprite.into()));

        Ok(())
    }

    pub fn finished(&self) -> bool {
        self.pool.is_empty()
    }

    pub fn place_item(&mut self) -> Result<()> {
        let sprite = self.random_item()?;

        let location = *self
            .needs_placement
            .intersection(&self.reach_cache)
            .choose(&mut self.rng)
            .unwrap();
        self.needs_placement.swap_remove(&location);

        self.commit_placement(location, sprite);

        Ok(())
    }

    pub fn finish(self) -> (Seed, ItemSpoiler) {
        (self.seed, self.spoiler)
    }

    fn random_item(&mut self) -> Result<Sprite> {
        let sprite = self.pool.choose_remove(&mut self.rng);

        if matches!(sprite, Sprite::Gear(_)) {
            self.update_reach();

            if self.reach_cache.is_empty() {
                Err(format!("failed to safely place {sprite:?}"))?;
            }

            self.fill_unreachable()?;
        }

        Ok(sprite)
    }

    fn update_reach(&mut self) {
        self.reach_cache = self.reach.reach(&self.pool)
    }

    fn fill_unreachable(&mut self) -> Result<()> {
        for location in self.unreachable().collect::<Vec<_>>() {
            let sprite = self.pool.choose_remove_filler(&mut self.rng)?;
            self.commit_placement(location, sprite);
        }

        Ok(())
    }

    fn unreachable(&self) -> impl Iterator<Item = Id> + use<'_> {
        self.needs_placement.difference(&self.reach_cache).copied()
    }

    fn commit_placement(&mut self, location: Id, sprite: Sprite) {
        self.needs_placement.swap_remove(&location);
        self.seed.push((location, sprite.into()));

        if let Sprite::Gear(gear) = sprite {
            self.spoiler.gear.insert(gear, location);
        }
    }

    #[cfg(debug_assertions)]
    pub fn validate_seed(&self, maps: &[Map], logic: &Logic) -> Result<()> {
        fn pool_without(sprite: Sprite, maps: &[Map], logic: &Logic) -> Pool {
            let mut pool = Pool::new(logic, maps);
            let index = pool.iter().position(|item| *item == sprite).unwrap();
            pool.swap_remove(index);
            pool
        }

        for (&gear, id) in &self.spoiler.gear {
            let pool = pool_without(Sprite::Gear(gear), maps, logic);
            let reach = self.reach.reach(&pool);
            assert!(reach.contains(id));
        }

        Ok(())
    }
}

trait BlindShuffle: Sized {
    const EXPECTED: &str;

    fn match_sprite(sprite: Sprite) -> Option<Self>;

    fn iter(maps: &[Map]) -> impl Iterator<Item = Id> {
        maps.iter().flat_map(|map| {
            map.sprites_with_positions()
                .filter(|(_, _, sprite)| Self::match_sprite(sprite.kind.into()).is_some())
                .map(|(x, y, _)| Id::new(map.identifier, x, y))
        })
    }

    fn map(id: Id, maps: &[Map]) -> Self {
        let sprite = id.expect_sprite(maps).kind.into();
        Self::match_sprite(sprite)
            .unwrap_or_else(|| panic!("unexpected {sprite:?} as {} {id}", Self::EXPECTED))
    }

    fn requirement(&self) -> Vec<Vec<Sprite>>;

    fn sprite(self) -> Sprite;
}

impl BlindShuffle for Door {
    const EXPECTED: &str = "door";

    fn match_sprite(sprite: Sprite) -> Option<Self> {
        match sprite {
            Sprite::Door(door @ (Door::Gold | Door::Silver | Door::Boulder)) => Some(door),
            _ => None,
        }
    }

    fn requirement(&self) -> Vec<Vec<Sprite>> {
        vec![vec![self.key()]]
    }

    fn sprite(self) -> Sprite {
        Sprite::Door(self)
    }
}

impl BlindShuffle for Enemy {
    const EXPECTED: &str = "enemy";

    fn match_sprite(sprite: Sprite) -> Option<Self> {
        match sprite {
            Sprite::Enemy(enemy) => Some(enemy),
            _ => None,
        }
    }

    fn requirement(&self) -> Vec<Vec<Sprite>> {
        match self {
            Self::FairyG | Self::FairyB => vec![vec![]],
            _ => vec![
                vec![Sprite::Collectible(Collectible::Sword); self.strength() as usize],
                vec![Sprite::Collectible(Collectible::Gem); 99],
            ],
        }
    }

    fn sprite(self) -> Sprite {
        Sprite::Enemy(self)
    }
}

impl Seed {
    fn blind_shuffle<T>(
        &mut self,
        maps: &[Map],
        requirement_map: &mut RequirementMap,
        rng: &mut Pcg64Mcg,
    ) where
        T: BlindShuffle,
    {
        let ids = T::iter(maps).collect::<Vec<_>>();
        let mut id_kinds = ids.iter().map(|id| T::map(*id, maps)).collect::<Vec<_>>();
        id_kinds.shuffle(rng);

        requirement_map.reserve(id_kinds.len());
        self.reserve(id_kinds.len());

        for (kind, id) in id_kinds.into_iter().zip(ids) {
            requirement_map.insert(id, kind.requirement());

            if !id.is_excluded() {
                self.push((id, kind.sprite().into()));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use itertools::Itertools;

    use crate::{rando::generate, rom::RomReader};

    use super::*;

    #[test]
    fn determinism() {
        let (maps, logic) = test_logic();

        let first = generate(&maps, &logic, Some("seed".to_string())).unwrap();
        let second = generate(&maps, &logic, Some("seed".to_string())).unwrap();

        assert_eq!(first, second);
    }

    #[test]
    fn completability() {
        let (maps, logic) = test_logic();

        for seed in 0..100 {
            if let Err(err) = generate(&maps, &logic, Some(seed.to_string())) {
                panic!("{seed} failed: {err}");
            }
        }
    }

    #[test]
    fn enough_items() {
        let (maps, logic) = test_logic();

        let (seed, _) = generate(&maps, &logic, Some("seed".to_string())).unwrap();

        let mut item_counts = HashMap::<Collectible, u16>::new();
        let mut door_counts = HashMap::<Door, u16>::new();

        for (_, sprite) in seed.placements {
            match sprite.kind.into() {
                Sprite::Collectible(collectible) => {
                    *item_counts.entry(collectible).or_default() += 1
                }
                Sprite::Door(door) => *door_counts.entry(door).or_default() += 1,
                _ => {}
            }
        }

        assert_eq!(item_counts[&Collectible::Sword], 99);
        assert_eq!(item_counts[&Collectible::GoldKey], 50);
        assert_eq!(item_counts[&Collectible::SilverKey], 50);
        assert_eq!(item_counts[&Collectible::Gem], 99);

        assert_eq!(door_counts[&Door::Gold], 50);
        assert_eq!(door_counts[&Door::Silver], 50);
    }

    #[test]
    fn no_duplicate_placements() {
        let (maps, logic) = test_logic();

        let (seed, _) = generate(&maps, &logic, Some("seed".to_string())).unwrap();

        let duplicates = seed
            .placements
            .into_iter()
            .map(|(id, _)| id)
            .duplicates()
            .collect::<Vec<_>>();

        assert!(
            duplicates.is_empty(),
            "duplicate placements: {duplicates:?}"
        );
    }

    fn test_logic() -> (Vec<Map>, Logic) {
        let mut rom = RomReader::open("Roms/main.hsrom".into()).unwrap();
        let mut maps = Map::parse_all(&mut rom).unwrap();
        maps.retain(Map::is_hardcore);
        let mut logic = Logic::parse().unwrap();
        logic.purge_doors(&maps);

        (maps, logic)
    }
}
