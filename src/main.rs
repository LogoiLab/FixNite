extern crate fixnite_lib as fixnite;

fn main() {
    let path = std::env::args().next_back().unwrap();
    fixnite::modify::modify::old_fix(path);
}
