mod saves;
mod savedata;
mod rom;
mod map;
mod inventory;
mod data;

use std::{fmt::Display, fs};

fn feedback<D: Display, T, E: Display>(description: D, result: Result<T, E>) {
    match result {
        Ok(_) => { eprintln!("{} - Success", description); }
        Err(err) => { eprintln!("{} - Failure: {}", description, err); }
    };
}

fn main() {
    feedback("Decode save 1", saves::decode("savedata"));
    feedback("Decode save 2", saves::decode("savedatb"));
    feedback("Decode save 3", saves::decode("savedatc"));
    feedback("Decode rom", rom::decode("rom~"));
}
