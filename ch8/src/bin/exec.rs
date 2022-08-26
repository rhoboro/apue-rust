use nix::libc::{execle, execlp};
use nix::sys::wait::waitpid;
use nix::unistd::{fork, ForkResult};
use std::ffi::{c_void, CString};
use std::ptr;

fn main() {
    match unsafe { fork() } {
        Err(_) => {
            eprintln!("fork error")
        }
        Ok(ForkResult::Child) => {
            let echo_all_path = CString::new(
                "/Users/rhoboro/go/src/github.com/rhoboro/apue-rust/target/release/echoall",
            )
            .unwrap();
            let echo_all = CString::new("echoall").unwrap();
            let myarg1 = CString::new("myarg1").unwrap();
            let myarg2 = CString::new("MY ARG2").unwrap();
            let user = CString::new("USER=unknown").unwrap();
            let path = CString::new("PATH=/tmp").unwrap();
            let env = [user.as_ptr(), path.as_ptr(), ptr::null()];
            if unsafe {
                execle(
                    echo_all_path.as_ptr(),
                    echo_all.as_ptr(),
                    myarg1.as_ptr(),
                    myarg2.as_ptr(),
                    ptr::null() as *const c_void,
                    env.as_ptr(),
                )
            } < 0
            {
                eprintln!("execle error");
            }
        }
        Ok(ForkResult::Parent { child, .. }) => match waitpid(child, Option::None) {
            Err(_) => {
                eprintln!("wait error")
            }
            Ok(_) => (),
        },
    }
    match unsafe { fork() } {
        Err(_) => {
            eprintln!("fork error")
        }
        Ok(ForkResult::Child) => {
            // echoall を実行できるよう PATH に含める必要がある
            let echo_all_path = CString::new("echoall").unwrap();
            let echo_all = CString::new("echoall").unwrap();
            let myarg1 = CString::new("only 1 arg").unwrap();
            if unsafe {
                execlp(
                    echo_all_path.as_ptr(),
                    echo_all.as_ptr(),
                    myarg1.as_ptr(),
                    ptr::null() as *const c_void,
                )
            } < 0
            {
                eprintln!("execle error");
            }
        }
        Ok(_) => (std::process::exit(0)),
    }
}
