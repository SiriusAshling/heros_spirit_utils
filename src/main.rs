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

    util::feedback_and_then("Read rom", zip::read_rom("rom"), |files| {
        util::feedback_and_then("Decode rom", rom::decode(files), |rom| {
            util::feedback("Gather stats", stats::map_stats("stats", &rom.maps));
            util::feedback("Export graphics", export::export_tilesets("rom_export/graphics", &rom.tile_data));
            util::feedback("Export sounds", export::export_audio("rom_export/sounds", &rom.sounds));
            util::feedback("Export music", export::export_audio("rom_export/music", &rom.music));

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
                            export::export_map("rom_export/maps", *identifier, map)
                        )
                        .collect::<Result<Vec<_>, _>>()
                    );
                    util::feedback("Export world map",
                        draw::merge_maps(maps).and_then(|map| export::export_full_map(&map))
                    );
                }
            );
        });
    });
}
