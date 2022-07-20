use nix::sys::stat::{lstat, SFlag};
use std::path::Path;

fn main() {
    for path in std::env::args().skip(1) {
        match lstat(Path::new(&path)) {
            Err(_) => eprintln!("lstat error"),
            Ok(status) => match SFlag::from_bits_truncate(status.st_mode) {
                SFlag::S_IFREG => println!("{} :regular", &path),
                SFlag::S_IFDIR => println!("{} :directory", &path),
                SFlag::S_IFCHR => println!("{} :character special", &path),
                SFlag::S_IFBLK => println!("{} :block special", &path),
                SFlag::S_IFIFO => println!("{} :fifo", &path),
                SFlag::S_IFLNK => println!("{} :symbolic link", &path),
                SFlag::S_IFSOCK => println!("{} :socket", &path),
                _ => println!("{}: **unknown mode **", &path),
            },
        }
    }
}
