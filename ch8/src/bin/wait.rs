use ch8::pr_exit;
use nix::sys::signal::{kill, Signal};
use nix::sys::wait::wait;
use nix::unistd::{fork, getpid, ForkResult};

fn main() {
    match unsafe { fork() } {
        Err(_) => {
            eprintln!("fork error");
        }
        Ok(ForkResult::Child) => {
            std::process::exit(0);
        }
        Ok(ForkResult::Parent { child: _, .. }) => {
            pr_exit::pr_exit(wait());
        }
    }
    match unsafe { fork() } {
        Err(_) => {
            eprintln!("fork error");
        }
        Ok(ForkResult::Child) => {
            std::process::abort();
        }
        Ok(ForkResult::Parent { child: _, .. }) => {
            pr_exit::pr_exit(wait());
        }
    }
    match unsafe { fork() } {
        Err(_) => {
            eprintln!("fork error");
        }
        Ok(ForkResult::Child) => {
            // ゼロ除算で SIGFPE を生成する代わり
            kill(getpid(), Signal::SIGFPE).unwrap();
        }
        Ok(ForkResult::Parent { child: _, .. }) => {
            pr_exit::pr_exit(wait());
        }
    }
}