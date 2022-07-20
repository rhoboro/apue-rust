use nix::sys::stat::{stat, SFlag};
use std::path::Path;

fn main() {
    for file in std::env::args().skip(1) {
        print!("{}: ", &file);
        let st = stat(Path::new(&file)).expect("stat error");
        // major/minor are available only on linux.
        print!("dev = {}", st.st_dev);
        let flag = SFlag::from_bits_truncate(st.st_mode);
        if flag == SFlag::S_IFCHR || flag == SFlag::S_IFBLK {
            print!(
                " ({}) rdev = {}",
                if flag == SFlag::S_IFCHR {
                    "character"
                } else {
                    "block"
                },
                st.st_rdev
            );
        }
        println!();
    }
}
