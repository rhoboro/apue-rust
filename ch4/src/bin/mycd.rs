use nix::unistd::chdir;
use std::path::Path;

fn main() {
    chdir(Path::new("/tmp")).expect("chdir failed");
    println!("chdir to /tmp succeeded");
}
