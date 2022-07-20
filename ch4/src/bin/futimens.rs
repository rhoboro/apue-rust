use nix::fcntl::{open, OFlag};
use nix::sys::stat::{futimens, stat, Mode};
use nix::sys::time::{TimeSpec, TimeValLike};
use nix::unistd::close;
use std::path::Path;

fn main() {
    for file in std::env::args() {
        let st = stat(Path::new(&file)).expect("stat error");
        let f = open(
            Path::new(&file),
            OFlag::O_RDWR | OFlag::O_TRUNC,
            Mode::empty(),
        )
        .expect("open error");
        futimens(
            f,
            &TimeSpec::seconds(st.st_atime),
            &TimeSpec::seconds(st.st_mtime),
        )
        .expect("futimens error");
        close(f).expect("close error");
    }
}
