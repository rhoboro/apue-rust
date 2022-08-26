fn main() {
    for (i, arg) in std::env::args().enumerate() {
        println!("args[{}]: {}", i, arg);
    }
    for (key, value) in std::env::vars() {
        println!("{}={}", key, value);
    }
}
