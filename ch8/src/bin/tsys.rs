use ch8::pr_exit;
use nix::libc::system;
use nix::sys::wait::WaitStatus;
use nix::unistd::Pid;
use std::ffi::CString;

fn main() {
    let arg = std::env::args()
        .nth(1)
        .expect("command-line argument required");
    let cmd = CString::new(arg).unwrap();
    let status = unsafe { system(cmd.as_ptr()) };
    if status < 0 {
        eprintln!("system() error");
    }
    // Pid is a dummy
    pr_exit::pr_exit(WaitStatus::from_raw(Pid::from_raw(0), status));
}
