use nix::unistd::getpid;

fn main() {
    println!("{}", getpid());
}
