use nix::unistd::{chdir, getcwd};
use std::path::Path;

fn main() {
    chdir(Path::new("/tmp")).expect("chdir failed");
    let cwd = getcwd().expect("getcwd failed");
    println!("cwd = {:?}", cwd.as_os_str());
}
