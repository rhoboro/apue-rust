use libc::{STDIN_FILENO, STDOUT_FILENO};
use nix::libc;
use nix::unistd::{read, write};

const BUF_SIZE: usize = 4096;

fn main() {
    let mut buf = [0; BUF_SIZE];
    loop {
        let n = read(STDIN_FILENO, &mut buf).expect("read error");
        if n <= 0 {
            break;
        }
        if write(STDOUT_FILENO, &buf[..n]).expect("write error") != n {
            eprintln!("write error");
        }
    }
}
