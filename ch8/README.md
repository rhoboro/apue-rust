```sh
# 8.3
$ cargo run -p ch8 --bin fork
a write to stdout
before fork
pid = Pid(82891), glob = 7, var = 89
pid = Pid(82890), glob = 6, var = 88

$ cargo run -p ch8 --bin fork > tmp.out
a write to stdout
before fork
pid = Pid(83057), glob = 7, var = 89
before fork
pid = Pid(83056), glob = 6, var = 88
```
