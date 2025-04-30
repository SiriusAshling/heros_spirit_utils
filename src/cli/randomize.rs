use std::path::PathBuf;

use rand::{rng, SeedableRng};
use rand_pcg::Pcg64Mcg;

use crate::{
    cli::import_rom,
    helpers::ResultExtension,
    rando::{generate, Logic},
    rom::{Rom, RomReader},
};

pub fn randomize(rom: PathBuf) {
    let logic = Logic::parse().ok_feedback("Parse logic");

    let reader = RomReader::open(rom);
    if let Some(mut reader) = reader {
        let mut rom = Rom::parse(&mut reader);

        if let (Some(mut logic), Some(maps)) = (logic, rom.maps.take()) {
            let (mut maps, mut other) = maps
                .into_iter()
                .partition::<Vec<_>, _>(|map| (42..=45).contains(&map.identifier));

            logic.purge_doors(&maps);

            let rng = Pcg64Mcg::from_rng(&mut rng());
            let seed = generate(&maps, &logic, rng);

            seed.apply(&mut maps);

            other.append(&mut maps);
            rom.maps = Some(other);

            rom.export(&mut reader);
            import_rom(PathBuf::from("Roms/randomizer.hsrom"));
        }
    }
}
