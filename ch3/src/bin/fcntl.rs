use nix::fcntl::{fcntl, OFlag, F_GETFL};

fn main() {
    let fd = std::env::args().nth(1).expect("usage: a.out <descriptor#>");
    let v: i8 = fd.parse().expect("parse error");
    let val = fcntl(v.into(), F_GETFL).expect("fcntl error for fd");

    let flag = OFlag::from_bits(val).unwrap();
    match flag & OFlag::O_ACCMODE {
        OFlag::O_RDONLY => print!("read only"),
        OFlag::O_WRONLY => print!("write only"),
        OFlag::O_RDWR => print!("read write"),
        _ => eprintln!("unknown access mode"),
    }

    if (flag & OFlag::O_APPEND) == OFlag::O_APPEND {
        print!(", append");
    }
    if (flag & OFlag::O_NONBLOCK) == OFlag::O_NONBLOCK {
        print!(", nonblocking");
    }
    if (flag & OFlag::O_SYNC) == OFlag::O_SYNC {
        print!(", synchronous writes");
    }
    println!();
}
