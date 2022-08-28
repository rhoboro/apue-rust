use nix::unistd::{geteuid, getuid};

fn main() {
    println!("real uid = {}, effective uid = {}", getuid(), geteuid());
}
