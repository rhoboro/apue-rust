fn main() {
    for (i, arg) in std::env::args().enumerate() {
        println!("argv[{}]: {}", i, arg);
    }
}
