use nix::sys::stat::Mode;
use nix::sys::stat::{fchmod, stat};
use std::fs::File;
use std::os::unix::io::AsRawFd;
use std::path::Path;

fn main() {
    let foo = Path::new("foo");
    let file_stat = stat(foo).expect("stat error for foo");
    fchmod(
        File::open(foo).unwrap().as_raw_fd(),
        Mode::from_bits_truncate(file_stat.st_mode) | !Mode::S_IXGRP | Mode::S_ISGID,
    )
    .expect("chmod error for foo");

    fchmod(
        File::open(Path::new("bar")).unwrap().as_raw_fd(),
        Mode::S_IRUSR | Mode::S_IWUSR | Mode::S_IRGRP | Mode::S_IROTH,
    )
    .expect("chmod error for bar");
}
