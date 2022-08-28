use nix::libc::nice;
use nix::unistd::{fork, ForkResult};
use std::time::{Duration, SystemTime};

// /Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/include/sys/syslimits.h
static N_ZERO: i32 = 20;

fn check_time(name: &str, count: u64, end: &SystemTime) -> bool {
    if SystemTime::now() >= *end {
        println!("{} count = {}", &name, count);
        true
    } else {
        false
    }
}

fn main() {
    let adj: i32 = std::env::args()
        .nth(1)
        .unwrap_or("0".to_string())
        .parse()
        .unwrap();

    let now = SystemTime::now();
    let end = now + Duration::from_secs(10);

    match unsafe { fork() } {
        Err(_) => {
            eprintln!("fork error");
        }
        Ok(ForkResult::Child) => {
            let mut count: u64 = 0;
            println!(
                "current nice value in child is {}, adjusting by {}",
                unsafe { nice(0) + N_ZERO },
                adj
            );
            let ret = unsafe { nice(adj) };
            if ret == -1 {
                eprintln!("child set scheduling priority");
            } else {
                println!("now child value is {}", ret + N_ZERO);
            }
            loop {
                count += 1;
                if check_time("child", count, &end) {
                    break;
                }
            }
        }
        Ok(ForkResult::Parent { child: _, .. }) => {
            let mut count: u64 = 0;
            println!("current nice value in parent is {}", unsafe {
                nice(0) + N_ZERO
            });
            loop {
                count += 1;
                if check_time("parent", count, &end) {
                    break;
                }
            }
        }
    }
}
