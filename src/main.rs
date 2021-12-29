mod saves;
mod savedata;
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
    util::feedback("Decode save 1", saves::decode("savedata"));
    util::feedback("Decode save 2", saves::decode("savedatb"));
    util::feedback("Decode save 3", saves::decode("savedatc"));

    util::feedback_and_then("Decode rom", rom::decode("rom~"), |rom| {
        util::feedback("Gather stats", stats::map_stats("stats", &rom.maps));
        util::feedback("Export graphics", export::export_tilesets("graphics", &rom.tile_data));

        util::feedback("Export maps",
            rom.maps.into_iter()
                .map(|map| export::export_map("maps", map, &rom.tile_data))
                .collect::<Result<Vec<_>, _>>()
        );
    });
}
