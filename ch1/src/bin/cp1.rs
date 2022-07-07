use nix::unistd::{read, write};
use std::io::{stdin, stdout};
use std::os::unix::io::AsRawFd;

const BUF_SIZE: usize = 4096;

fn main() {
    let mut buf = [0_u8; BUF_SIZE];
    loop {
        match read(stdin().as_raw_fd(), &mut buf) {
            Ok(_n @ 0) => {
                std::process::exit(0);
            }
            Ok(n) => {
                if write(stdout().as_raw_fd(), &buf[..n]).unwrap() != n {
                    std::process::exit(1);
                }
            }
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    }
}
