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
mod util;

fn main() {
    util::feedback("Decode save 1", saves::decode("savedata"));
    util::feedback("Decode save 2", saves::decode("savedatb"));
    util::feedback("Decode save 3", saves::decode("savedatc"));
    util::feedback("Decode rom", rom::decode("rom~"));
}
