use nix::sys::signal::{kill, Signal};
use nix::sys::wait::{wait, WaitStatus};
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
            wait_pr_exit();
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
            wait_pr_exit();
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
            wait_pr_exit();
        }
    }
}

fn wait_pr_exit() {
    match wait() {
        Err(_) => {
            eprintln!("wait error");
        }
        Ok(WaitStatus::Exited(_, status)) => {
            println!("normal termination, exit status = {:?}", status);
        }
        Ok(WaitStatus::Signaled(_, signal, has_core)) => {
            print!("abnormal termination, signal number = {:?}", signal);
            if has_core {
                println!(" (core file generated)");
            } else {
                println!();
            }
        }
        Ok(WaitStatus::Stopped(_, signal)) => {
            println!("child stopped, signal number = {:?}", signal)
        }
        Ok(_) => {
            println!("other status");
        }
    }
}
