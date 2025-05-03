mod generator;
mod id;
mod logic;
mod pool;
mod seed;
mod spoiler;
mod transfers;
mod visualize;

pub use logic::Logic;
pub use seed::Seed;
pub use spoiler::Spoiler;
pub use visualize::Visualizer;

use generator::Generator;

use crate::map::Map;

pub fn generate(maps: &[Map], logic: &Logic, seed: Option<String>) -> (Seed, Spoiler) {
    let mut generator = Generator::new(maps, logic, seed);

    while !generator.finished() {
        generator.place_random();
        generator.fill_unreachable();
    }

    generator.finish()
}
