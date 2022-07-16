use std::fs::File;
use std::io::{Seek, SeekFrom, Write};
use std::path::Path;

fn main() {
    let path = Path::new("file.hole");
    let mut file = File::create(&path).expect("create error");
    if file.write("abcdefghij".as_bytes()).expect("write error") != 10 {
        std::process::exit(1);
    }
    file.seek(SeekFrom::Start(16384)).expect("seek error");
    if file.write("ABCDEFGHIJ".as_bytes()).expect("write error") != 10 {
        std::process::exit(1);
    }
}
