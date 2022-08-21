mod error;
mod zip;
mod saves;
mod savedata;
mod global;
mod rom;
mod tile;
mod palette;
mod map;
mod inventory;
mod sprite;
mod data;
mod draw;
mod export;
mod stats;
mod util;

fn main() {
    util::feedback("Encode save 1", saves::encode("savedata"));
    util::feedback("Encode save 2", saves::encode("savedatb"));
    util::feedback("Encode save 3", saves::encode("savedatc"));

    util::feedback("Decode save 1", saves::decode("savedata"));
    util::feedback("Decode save 2", saves::decode("savedatb"));
    util::feedback("Decode save 3", saves::decode("savedatc"));
    util::feedback("Decode global save", global::decode());

    util::feedback_and_then("Encode rom", rom::encode("rom_export"), |files| {
        util::feedback("Write rom", export::export_files("rom_encoded", &files, ""));
    });

    util::feedback_and_then("Read rom", zip::read_rom("rom"), |files| {
        util::feedback_and_then("Decode rom", rom::decode(files), |rom| {
            util::feedback("Gather stats", stats::map_stats("stats", &rom.maps));
            util::feedback("Export graphics",
                export::export_tilesets("rom_export/graphics", &rom.tile_data)
                .and_then(|_| export::export_files("rom_export/graphics", &rom.images, "bmp")));
            util::feedback("Export sounds", export::export_files("rom_export/sounds", &rom.sounds, "ogg"));
            util::feedback("Export music", export::export_files("rom_export/music", &rom.music, "ogg"));
            util::feedback("Export shaders", export::export_files("rom_export/shaders", &rom.shaders, ""));
            util::feedback("Export maps", export::export_maps("rom_export/maps", &rom.maps));

            util::feedback_and_then("Draw maps",
                rom.maps.into_iter()
                    .map(|map| {
                        let identifier = map.identifier;
                        draw::draw_map(map, &rom.tile_data).map(|map| (identifier, map))
                    })
                    .collect::<Result<Vec<_>, _>>(),
                |maps| {
                    util::feedback("Export maps",
                        maps.iter().map(|(identifier, map)|
                            export::export_map_image("rom_export/maps/images", *identifier, map)
                        )
                        .collect::<Result<Vec<_>, _>>()
                    );
                    util::feedback("Export world map",
                        draw::merge_maps(maps).and_then(|map| export::export_full_map_image(&map))
                    );
                }
            );
        });
    });
}
