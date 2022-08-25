mod error;
mod zip;
mod savedata;
mod global;
mod rom;
mod graphics;
mod palette;
mod map;
mod tiled;
mod inventory;
mod sprite;
mod data;
mod draw;
mod export;
mod import;
mod stats;
mod util;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(version, about)]
struct Args {
    #[clap(subcommand)]
    command: Option<Command>,
}
#[derive(Subcommand)]
enum Command {
    /// Exports the rom and save files into formats suitable for viewing and editing
    /// 
    /// Will search for "rom", "savedata", "savedatb" and "savedatc" in the current directory and operate on all files it can find
    Export,
    /// Reimport the files exported earlier into the format used by the game
    /// 
    /// Does not (yet) repackage the rom, you will have to zip it yourself
    Import,
}
impl Default for Command {
    fn default() -> Self { Self::Export }
}

fn main() {
    let args = Args::parse();

    match args.command.unwrap_or_default() {
        Command::Export => export(),
        Command::Import => import(),
    }
}

fn export() {
    util::feedback("Export savedata", savedata::decode("savedata"));
    util::feedback("Export savedatb", savedata::decode("savedatb"));
    util::feedback("Export savedatc", savedata::decode("savedatc"));
    util::feedback("Export global save", global::decode());

    util::feedback_and_then("Read rom", zip::read_rom("rom").and_then(rom::decode), |rom| {
        util::feedback("Gather stats", stats::map_stats("stats", &rom.maps));
        util::feedback("Export graphics",
            export::export_tilesets("rom_files/graphics", &rom.tile_data)
            .and_then(|()| export::export_files("rom_files/graphics", &rom.images, "bmp")));
        util::feedback("Export audio",
            export::export_files("rom_files/sounds", &rom.sounds, "ogg")
            .and_then(|()| export::export_files("rom_files/music", &rom.music, "ogg")));
        util::feedback("Export shaders", export::export_files("rom_files/shaders", &rom.shaders, ""));
        util::feedback("Export maps", export::export_maps("rom_files/maps", &rom.maps, &rom.tile_data));

        let maps = rom.maps.into_iter().map(|map| {
            let identifier = map.identifier;
            let map = draw::draw_map(map, &rom.tile_data);
            util::feedback(format!("Draw map {}", map::map_name(identifier)), export::export_map_image("rom_files/maps/images", identifier, &map));
            (identifier, map)
        }).collect();
        util::feedback("Draw world map",
            draw::merge_maps(maps).and_then(|map| export::export_full_map_image(&map))
        );
    });
}

fn import() {
    util::feedback("Import savedata", savedata::encode("savedata"));
    util::feedback("Import savedatb", savedata::encode("savedatb"));
    util::feedback("Import savedatc", savedata::encode("savedatc"));

    let mut files = Vec::new();

    util::feedback("Import graphics",
        import::import_tilesets("rom_files/graphics", &mut files)
        .and_then(|()| import::import_files("rom_files/graphics", &mut files, "bmp")));
    util::feedback("Import audio",
        import::import_files("rom_files/sounds", &mut files, "ogg")
        .and_then(|()| import::import_files("rom_files/music", &mut files, "ogg")));
    util::feedback("Import shaders", import::import_files("rom_files/shaders", &mut files, ""));
    util::feedback("Import maps", import::import_maps("rom_files/maps", &mut files));

    if !files.is_empty() {
        util::feedback("Write rom", zip::write_rom("rom", files));
    }
}
