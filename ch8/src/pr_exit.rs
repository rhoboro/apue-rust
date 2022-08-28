use nix::sys::wait::WaitStatus;
use nix::Result;

pub fn pr_exit(ws: Result<WaitStatus>) {
    match ws {
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
