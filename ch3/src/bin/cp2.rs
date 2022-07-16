use std::io;
use std::io::{Read, Write};

const BUF_SIZE: usize = 4096;

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buf = [0; BUF_SIZE];
    loop {
        let n = stdin.lock().read(&mut buf).expect("read error");
        if n <= 0 {
            break;
        }
        stdout.lock().write(&buf[..n]).expect("write error");
        stdout.flush().unwrap();
    }
}
