mod generator;
mod id;
mod logic;
mod pool;
mod seed;
mod spoiler;
mod transfers;
mod visualize;

use constcat::concat_slices;
pub use logic::Logic;
use rand_pcg::Pcg64Mcg;
use rand_seeder::Seeder;
pub use seed::Seed;
use spoiler::ItemSpoiler;
pub use spoiler::Spoiler;
use strum::VariantNames;
pub use visualize::Visualizer;

use generator::Generator;

use crate::{
    map::{Collectible, Door, Enemy, Gear, Map},
    Result,
};

pub fn generate(maps: &[Map], logic: &Logic, seed: Option<String>) -> Result<(Seed, Spoiler)> {
    let rng_seed = seed.unwrap_or_else(random_seed);
    let mut rng = Seeder::from(&rng_seed).into_rng();

    for attempt in 0_u8..10 {
        match try_generate(maps, logic, &mut rng) {
            Ok((seed, spoiler)) => {
                let spoiler = Spoiler {
                    seed: rng_seed,
                    items: spoiler,
                };
                return Ok((seed, spoiler));
            }
            Err(err) => {
                eprintln!("seed attempt {attempt} failed - {err}");
            }
        }
    }

    Err("all attempts failed")?
}

fn random_seed() -> String {
    use adjective_adjective_animal::{Generator, ADJECTIVES};

    const NOUNS: &[&str] = concat_slices!([&str]: Collectible::VARIANTS, Gear::VARIANTS, Door::VARIANTS, Enemy::VARIANTS);

    Generator::new(ADJECTIVES, NOUNS).next().unwrap()
}

fn try_generate(maps: &[Map], logic: &Logic, rng: &mut Pcg64Mcg) -> Result<(Seed, ItemSpoiler)> {
    let mut generator = Generator::new(maps, logic, rng)?;

    while !generator.finished() {
        generator.place_item()?;
    }

    #[cfg(debug_assertions)]
    generator.validate_seed(maps, logic)?;

    Ok(generator.finish())
}
