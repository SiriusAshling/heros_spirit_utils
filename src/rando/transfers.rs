use std::collections::HashMap;

use rand::Rng;
use rand_pcg::Pcg64Mcg;

use crate::helpers::RemoveRandom;

use super::{id::Id, logic::Logic};

pub fn generate_transfers<'a>(
    logic: &'a Logic,
    rng: &mut Pcg64Mcg,
) -> (HashMap<Id, &'a String>, Vec<(Id, Id)>) {
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
    let mut map_transfers = vec![];

    while let Some(source) = choose_source(&mut groups, &mut ends, rng) {
        let target = choose_target(&mut groups, rng).expect("unable to match up all transfers");

        logic_transfers.insert(source.1, target.0);
        logic_transfers.insert(target.1, source.0);
        map_transfers.push((source.1, target.1));
        map_transfers.push((target.1, source.1));
    }

    (logic_transfers, map_transfers)
}

fn choose_source<'a>(
    groups: &mut Vec<Vec<(&'a String, Id)>>,
    ends: &mut Vec<(&'a String, Id)>,
    rng: &mut Pcg64Mcg,
) -> Option<(&'a String, Id)> {
    if ends.is_empty() {
        choose_target(groups, rng)
    } else {
        Some(ends.choose_remove(rng))
    }
}

fn choose_target<'a>(
    groups: &mut Vec<Vec<(&'a String, Id)>>,
    rng: &mut Pcg64Mcg,
) -> Option<(&'a String, Id)> {
    if groups.is_empty() {
        return None;
    }

    let index = rng.random_range(..groups.len());
    let group = &mut groups[index];
    let target = group.choose_remove(rng);

    if group.is_empty() {
        groups.swap_remove(index);
    }

    Some(target)
}
