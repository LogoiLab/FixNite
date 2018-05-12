use std::fs::*;
use std::io::prelude::*;

fn main() {
    let mut replay_file = match File::open(std::env::args().next_back().unwrap()){
        Ok(o) => o,
        Err(e) => {
            panic!("{}", e);
        },
    };
    let mut replay_data = Vec::new();
    match replay_file.read_to_end(&mut replay_data){
        Ok(_) => {
            let offset: usize = 16;
            if replay_data[offset] != 42 as u8 {
                replay_data[offset] = 42 as u8;
                replay_data[offset + 1] = 42 as u8;
                replay_data[offset + 2] = 61 as u8;
            }

            let mut output_file = match File::create("foo.test"){
                Ok(o) => o,
                Err(e) => {
                    panic!("{}", e);
                },
            };
            output_file.write_all(replay_data.as_slice());

            output_file.sync_all();
        },
         _ => (),
    }

}
