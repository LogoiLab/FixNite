extern crate nom;

use std::fs::File;
use std::io::prelude::*;

pub struct ReplayFile {
    name: String,
    size: usize,
    content: String,
}

pub fn load_file(filename: String) -> ReplayFile {
    let mut f = File::open(&filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");
    let replay_file: ReplayFile = ReplayFile {
        name: filename,
        size: contents.len(),
        content: contents
    };

    replay_file
}
