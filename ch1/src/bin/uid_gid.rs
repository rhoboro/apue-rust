use nix::unistd::{getgid, getuid};

fn main() {
    println!("uid = {}, gid = {}", getuid(), getgid());
}
