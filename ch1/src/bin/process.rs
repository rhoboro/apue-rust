use nix::sys::wait::waitpid;
use nix::unistd::{execvp, fork, ForkResult};
use std::ffi::CString;
use std::io;
use std::io::prelude::*;

fn main() {
    let stdout = io::stdout();
    {
        let mut out = stdout.lock();
        write!(out, "% ").unwrap();
        out.flush().unwrap();
    }

    let stdin = io::stdin();
    while let Some(Ok(line)) = stdin.lock().lines().next() {
        if !line.is_empty() {
            match unsafe { fork() } {
                Ok(ForkResult::Parent { child, .. }) => {
                    waitpid(child, None).unwrap();
                }
                Ok(ForkResult::Child) => {
                    let cmd = line
                        .split(" ")
                        .map(|c| CString::new(c).unwrap())
                        .collect::<Vec<_>>();
                    execvp(&cmd[0], &cmd).unwrap();
                    std::process::exit(0);
                }
                Err(_) => {
                    std::process::exit(1);
                }
            }
        }
        {
            let mut out = stdout.lock();
            write!(out, "% ").unwrap();
            out.flush().unwrap();
        }
    }
    std::process::exit(0);
}
