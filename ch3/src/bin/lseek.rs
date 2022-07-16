use libc::STDIN_FILENO;
use nix::libc;
use nix::unistd::{lseek, Whence};

fn main() {
    match lseek(STDIN_FILENO, 0, Whence::SeekCur) {
        Ok(_offset @ -1) => {
            println!("cannot seek");
        }
        Ok(_) => {
            println!("seek OK");
        }
        Err(_) => {
            println!("cannot seek");
        }
    }
}
