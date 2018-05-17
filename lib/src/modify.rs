pub mod modify {

    use std::fs::*;
    use std::io::prelude::*;

    pub fn old_fix(path: String) {
        let mut replay_file = match File::open(&path){
            Ok(o) => o,
            Err(e) => panic!("{}", e),
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

                let mut output_file = match File::create(format!("fixed-{}", path.as_str())) {
                    Ok(o) => o,
                    Err(e) => panic!("{}", e),
                };
                match output_file.write_all(replay_data.as_slice()) {
                    Ok(_) => (),
                    Err(e) => panic!("{}", e),
                }

                match output_file.sync_all() {
                    Ok(_) => (),
                    Err(e) => panic!("{}", e),
                }
            },
            Err(e) => panic!("{}", e),
        }
    }

}
