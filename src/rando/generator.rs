use indexmap::IndexSet;
use rand::seq::SliceRandom;
use rand_pcg::Pcg64Mcg;

use crate::{
    helpers::RemoveRandom,
    map::{Collectible, Door, Enemy, Map, Sprite},
};

use super::{
    id::Id,
    logic::{Logic, Reach, RequirementMap},
    pool::Pool,
    seed::Seed,
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
        let mut seed = Seed::default();
        let mut requirement_map = RequirementMap::new(logic);

        let logic_transfers = seed.generate_transfers(maps, logic, &mut rng);
        seed.blind_shuffle::<Door>(maps, &mut requirement_map, &mut rng);
        seed.blind_shuffle::<Enemy>(maps, &mut requirement_map, &mut rng);

        let reach = Reach::new(logic, logic_transfers, requirement_map);
        let pool = Pool::new(logic, maps);
        let needs_placement = reach.reach(&pool);

        let mut generator = Generator {
            rng,
            reach,
            pool,
            needs_placement,
            seed,
        };

        for item in logic.items() {
            if !generator.needs_placement.contains(&item) {
                eprintln!("item {item} is unreachable");
                generator.place_unreachable(item);
            }
        }

        generator
    }

    fn place_unreachable(&mut self, location: Id) {
        let sprite = self.pool.choose_remove(&mut self.rng);
        self.seed.push((location, sprite.into()));
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
        self.seed.push((location, sprite.into()));
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

        requirement_map.extend(
            ids.iter()
                .zip(&id_kinds)
                .map(|(id, kind)| (*id, kind.requirement())),
        );

        id_kinds.shuffle(rng);

        self.extend(
            id_kinds
                .into_iter()
                .zip(ids)
                .filter(|(_, id)| !id.is_excluded())
                .map(|(kind, id)| (id, kind.sprite().into())),
        );
    }
}
