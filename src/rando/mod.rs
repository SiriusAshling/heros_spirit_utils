mod generator;
mod id;
mod logic;
mod pool;
mod seed;
mod transfers;
mod visualize;

pub use logic::Logic;
pub use seed::Seed;
pub use visualize::Visualizer;

use generator::Generator;
use rand_pcg::Pcg64Mcg;

use crate::map::Map;

pub fn generate(maps: &[Map], logic: &Logic, rng: Pcg64Mcg) -> Seed {
    let mut generator = Generator::new(maps, logic, rng);

    while !generator.finished() {
        generator.place_random();
        generator.fill_unreachable();
    }

    generator.finish()
}
