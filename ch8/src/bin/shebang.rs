use nix::libc::execl;
use nix::sys::wait::waitpid;
use nix::unistd::{fork, ForkResult};
use std::ffi::{c_void, CString};
use std::ptr;

fn main() {
    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            let testinterp_path =
                CString::new("/Users/rhoboro/go/src/github.com/rhoboro/apue-rust/ch8/testinterp")
                    .unwrap();
            let testinterp = CString::new("testinterp").unwrap();
            let myarg1 = CString::new("myarg1").unwrap();
            let myarg2 = CString::new("MY ARG2").unwrap();
            ptr::null() as *const c_void;

            if unsafe {
                execl(
                    testinterp_path.as_ptr(),
                    testinterp.as_ptr(),
                    myarg1.as_ptr(),
                    myarg2.as_ptr(),
                    ptr::null() as *const c_void,
                )
            } < 0
            {
                eprintln!("execl error");
            }
        }
        Ok(ForkResult::Parent { child, .. }) => match waitpid(child, Option::None) {
            Err(_) => {
                eprintln!("wait error");
            }
            Ok(_) => (),
        },
        Err(_) => {
            eprintln!("fork error");
        }
    }
}
