use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use nix::unistd::{sleep, unlink};
use std::path::Path;

fn main() {
    let temp = Path::new("tempfile");
    let _fd = open(temp, OFlag::O_RDWR, Mode::empty()).expect("open error");
    unlink(temp).expect("unlink error");
    println!("file unlinked");
    sleep(15);
    println!("done");
}
