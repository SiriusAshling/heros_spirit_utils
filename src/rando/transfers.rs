use std::collections::HashMap;

use rand::Rng;
use rand_pcg::Pcg64Mcg;

use crate::{
    helpers::RemoveRandom,
    map::{Map, Sprite, SpriteData, Things},
};

use super::{id::Id, logic::Logic, Seed};

impl Seed {
    pub fn generate_transfers<'logic>(
        &mut self,
        maps: &[Map],
        logic: &'logic Logic,
        rng: &mut Pcg64Mcg,
    ) -> HashMap<Id, &'logic String> {
        let mut groups = logic.transfer_groups();
        let mut ends = vec![];

        groups.retain_mut(|group| {
            let is_end = group.len() == 1;

            if is_end {
                ends.push(group.pop().unwrap());
            }

            !is_end
        });

        let mut logic_transfers = HashMap::new();

        while let Some(source) = choose_source(&mut groups, &mut ends, rng) {
            let target = choose_target(&mut groups, &mut ends, rng)
                .expect("unable to match up all transfers");

            logic_transfers.insert(source.1, target.0);
            logic_transfers.insert(target.1, source.0);
            self.push((source.1, target.1.find_target_transfer(maps)));
            self.push((target.1, source.1.find_target_transfer(maps)));
        }

        logic_transfers
    }
}

fn choose_source<'a>(
    groups: &mut Vec<Vec<(&'a String, Id)>>,
    ends: &mut Vec<(&'a String, Id)>,
    rng: &mut Pcg64Mcg,
) -> Option<(&'a String, Id)> {
    if ends.is_empty() {
        choose_target(groups, ends, rng)
    } else {
        Some(ends.choose_remove(rng))
    }
}

fn choose_target<'a>(
    groups: &mut Vec<Vec<(&'a String, Id)>>,
    ends: &mut Vec<(&'a String, Id)>,
    rng: &mut Pcg64Mcg,
) -> Option<(&'a String, Id)> {
    if groups.is_empty() {
        if ends.len() == 1 {
            return ends.pop();
        }

        return None;
    }

    let index = rng.random_range(..groups.len());
    let group = &mut groups[index];
    let target = group.choose_remove(rng);

    if group.len() < 2 {
        let mut group = groups.swap_remove(index);

        if let Some(end) = group.pop() {
            ends.push(end);
        }
    }

    Some(target)
}

impl Id {
    fn find_target_transfer(self, maps: &[Map]) -> SpriteData {
        let sprite = self.expect_sprite(maps);

        debug_assert!(matches!(
            sprite.kind.into(),
            Sprite::Things(Things::Transfer)
        ));

        let x = sprite.extra_bytes[1] as usize;
        let y = sprite.extra_bytes[2] as usize;
        let target_id = Id::new(sprite.extra_bytes[0], x, y);
        let map = target_id.expect_map(maps);

        x.checked_sub(1)
            .and_then(|x| map.sprite(x, y))
            .or_else(|| x.checked_add(1).and_then(|x| map.sprite(x, y)))
            .or_else(|| y.checked_sub(1).and_then(|y| map.sprite(x, y)))
            .or_else(|| y.checked_add(1).and_then(|y| map.sprite(x, y)))
            .unwrap_or_else(|| {
                panic!("failed to find target transfer for {self} (targets {target_id})")
            })
            .clone()
    }
}
