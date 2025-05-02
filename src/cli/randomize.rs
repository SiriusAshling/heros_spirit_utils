use std::path::PathBuf;

use rand::{rng, SeedableRng};
use rand_pcg::Pcg64Mcg;

use crate::{
    cli::import_rom,
    graphics::merge_maps,
    helpers::ResultExtension,
    map,
    rando::{generate, Logic, Visualizer},
    rom::{Rom, RomReader},
};

use super::export::{save_image, save_map_image};

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

pub fn draw_logic(rom: PathBuf) {
    let logic = Logic::parse().ok_feedback("Parse logic");

    let reader = RomReader::open(rom);
    let Some(mut reader) = reader else {
        return;
    };

    let rom = Rom::parse(&mut reader);
    let (Some(logic), Some(maps), Some(data)) = (logic, &rom.maps, rom.draw_data()) else {
        return;
    };

    let mut images = vec![];
    let visualizer = Visualizer::new(&logic);

    for map in maps {
        if !(42..=45).contains(&map.identifier) {
            continue;
        }

        let mut image = data.draw_map(map);

        visualizer.visualize_areas(map.identifier, &mut image);
        visualizer.visualize_connections(map.identifier, &mut image);

        let name = map::map_name(map.identifier);
        save_map_image("rando/visualizer", map.identifier, &image)
            .feedback(format!("Save {name} visualization"));

        images.push((map.identifier, image));
    }

    let (name, merged) = &merge_maps(images)[1];

    save_image("rando/visualizer", name, merged).feedback(format!("Save {name} visualization"));
}
