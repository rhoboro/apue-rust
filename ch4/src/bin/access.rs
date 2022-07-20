use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use nix::unistd::{access, AccessFlags};
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: a.out <pathname>");
        std::process::exit(1);
    }
    let path = Path::new(&args[1]);
    match access(path, AccessFlags::R_OK) {
        Ok(_) => println!("read access OK"),
        Err(_) => eprintln!("access error for {:?}", path),
    }
    match open(path, OFlag::O_RDONLY, Mode::empty()) {
        Ok(_) => println!("open for reading OK"),
        Err(_) => eprintln!("open error for {:?}", path),
    }
}
