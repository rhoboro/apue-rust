use nix::libc::creat;
use nix::sys::stat::{umask, Mode};
use std::ffi::CString;

fn main() {
    let rwrwrw = Mode::S_IRUSR
        | Mode::S_IWUSR
        | Mode::S_IRGRP
        | Mode::S_IWGRP
        | Mode::S_IROTH
        | Mode::S_IWOTH;
    umask(Mode::empty());
    let foo = CString::new("foo").unwrap();
    match unsafe { creat(foo.as_ptr(), rwrwrw.bits()) } {
        i if i < 0 => {
            eprintln!("creat error for foo");
            std::process::exit(1);
        }
        _ => (),
    }

    umask(Mode::S_IRGRP | Mode::S_IWGRP | Mode::S_IROTH | Mode::S_IWOTH);
    let bar = CString::new("bar").unwrap();
    match unsafe { creat(bar.as_ptr(), rwrwrw.bits()) } {
        i if i < 0 => {
            eprintln!("creat error for bar");
            std::process::exit(1);
        }
        _ => (),
    }
}
