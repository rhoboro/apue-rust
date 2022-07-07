use nix::dir::Dir;
use nix::fcntl::OFlag;
use nix::sys::stat::Mode;
use std::path::Path;

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let dir = Dir::open(Path::new(&path), OFlag::empty(), Mode::empty()).unwrap();
    for f in dir {
        println!("{}", f.unwrap().file_name().as_ref().to_string_lossy());
    }
}
