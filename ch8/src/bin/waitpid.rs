use nix::sys::wait::waitpid;
use nix::unistd::{fork, getppid, sleep, ForkResult};

fn main() {
    match unsafe { fork() } {
        Err(_) => {
            eprintln!("fork error")
        }
        Ok(ForkResult::Child) => {
            match unsafe { fork() } {
                Err(_) => {
                    eprintln!("fork error")
                }
                Ok(ForkResult::Parent { child: _, .. }) => {
                    // 2番目のフォークの親 == 最初の子
                    std::process::exit(0);
                }
                Ok(ForkResult::Child) => {
                    sleep(2);
                    // init(pid:0) が親になっている
                    println!("second child, parent pid = {:?}", getppid());
                    std::process::exit(0);
                }
            }
        }
        Ok(ForkResult::Parent { child, .. }) => match waitpid(child, Option::None) {
            _ => (),
        },
    }
    // 親プロセス
    std::process::exit(0);
}
