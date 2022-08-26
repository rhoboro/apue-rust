use nix::unistd::{fork, ForkResult};

fn main() {
    match unsafe { fork() } {
        Err(_) => {
            eprintln!("fork error");
        }
        Ok(ForkResult::Child) => {
            println!("output from child");
        }
        Ok(ForkResult::Parent { child: _, .. }) => {
            println!("output from parent")
        }
    }
}
