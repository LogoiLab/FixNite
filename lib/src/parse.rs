extern crate nom;

use std::env;
use std::fs::File;
use std::io::prelude::*;

pub struct replay_file {
    name: String,
    size: usize,
    content: String,
}

pub fn load_file(filename: String) -> replay_file {
    let mut f = File::open(&filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");
    return replay_file{name: filename, size: contents.len(), content: contents};
}
