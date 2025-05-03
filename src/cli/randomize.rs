use std::path::PathBuf;

use rand::{rng, SeedableRng};
use rand_pcg::Pcg64Mcg;
use rand_seeder::Seeder;

use crate::{
    graphics::merge_maps,
    helpers::{OptionExtension, ResultExtension},
    map::{self, Map},
    rando::{generate, Logic, Visualizer},
    rom::{Index, Rom, RomReader, RomWriter},
    Result,
};

use super::{
    export::{save_image, save_map_image},
    RandomizeArgs,
};

pub fn randomize(args: RandomizeArgs) -> Result<()> {
    let rom = args.rom_args.rom.unwrap_or_prompt()?;
    let logic = Logic::parse().ok_feedback("Parse logic");

    let reader = RomReader::open(rom);
    if let Some(mut reader) = reader {
        let mut rom = Rom::parse(&mut reader);

        if let (Some(mut logic), Some(maps)) = (logic, rom.maps.take()) {
            let (mut maps, other) = maps
                .into_iter()
                .partition::<Vec<_>, _>(|map| (42..=45).contains(&map.identifier));

            logic.purge_doors(&maps);

            let rng = match args.seed {
                None => Pcg64Mcg::from_rng(&mut rng()),
                Some(seed) => Seeder::from(seed).into_rng(),
            };

            let seed = generate(&maps, &logic, rng);
            seed.apply(&mut maps);

            write_seed(maps.iter().chain(&other), reader).feedback("Write seed");
        }
    }

    Ok(())
}

fn write_seed<'a, I>(maps: I, mut reader: RomReader) -> Result<()>
where
    I: IntoIterator<Item = &'a Map>,
{
    let mut writer = RomWriter::create(PathBuf::from("Roms/randomizer.hsrom"))?;

    for map in maps {
        writer.write(&format!("Maps/map{:02}", map.identifier), &map.encode())?;
    }

    let Index {
        graphics,
        maps: _,
        map_colors,
        map_meta,
        images,
        audio,
        shaders,
        other,
    } = reader.index;

    for index in graphics
        .into_iter()
        .chain(map_colors)
        .chain(map_meta)
        .chain(images)
        .chain(audio)
        .chain(shaders)
        .chain(other)
    {
        let file = reader.archive.by_index_raw(index)?;
        writer.archive.raw_copy_file(file)?;
    }

    Ok(())
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
