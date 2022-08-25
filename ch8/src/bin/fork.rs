use nix::libc::printf;
use nix::unistd::{fork, getpid, sleep, ForkResult};
use std::ffi::CString;
use std::sync::atomic::{AtomicI32, Ordering};

static GLOBAL: AtomicI32 = AtomicI32::new(6);

fn main() {
    let mut var = 88;
    println!("a write to stdout");
    print("before fork\n");
    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            GLOBAL.fetch_add(1, Ordering::SeqCst);
            var += 1;
        }
        Ok(ForkResult::Parent { child: _, .. }) => {
            sleep(2);
        }
        Err(_) => {
            println!("fork error")
        }
    }
    print(format!(
        "pid = {:?}, glob = {:?}, var = {:?}\n",
        getpid(),
        GLOBAL,
        var
    ));
}

fn print<T: Into<Vec<u8>>>(t: T) {
    // unbuffered
    let c_str = CString::new(t).unwrap();
    let ptr = c_str.as_ptr();
    unsafe { printf(ptr) };
}
