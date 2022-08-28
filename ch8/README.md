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

# 8.6
$ cargo run -p ch8 --bin wait
normal termination, exit status = 0
abnormal termination, signal number = SIGABRT
abnormal termination, signal number = SIGFPE

# Printed after the prompt
$ cargo run -p ch8 --bin waitpid
$ second child, parent pid = Pid(1)

# 8.10
$ cargo build -p ch8 --bin echoall --release
$ PATH=$PATH:$(pwd)/target/release cargo run -p ch8 --bin exec
args[0]: echoall
args[1]: myarg1
args[2]: MY ARG2
USER=unknown
PATH=/tmp
args[0]: echoall
args[1]: only 1 arg
CARGO=/Users/rhoboro/.rustup/toolchains/stable-aarch64-apple-darwin/bin/cargo
...

# 8.12
$ chmod +x ch8/testinterp
$ cargo run -p ch8 --bin shebang
argv[0]: /Users/rhoboro/go/src/github.com/rhoboro/apue-rust/target/release/echoarg
argv[1]: foo
argv[2]: /Users/rhoboro/go/src/github.com/rhoboro/apue-rust/ch8/testinterp
argv[3]: myarg1
argv[4]: MY ARG2

# 8.13
$ cargo run -p ch8 --bin tsys target/release/printuids
real uid = 501, effective uid = 501
normal termination, exit status = 0


# 8.16
# Might be less effective on macOS
$ cargo run -p ch8 --bin nice
current nice value in parent is 20
current nice value in child is 20, adjusting by 0
now child value is 20
parent count = 763043039
child count = 762491266

$ cargo run -p ch8 --bin nice 20
current nice value in parent is 20
current nice value in child is 20, adjusting by 20
now child value is 39
child count = 761021130
parent count = 762498712

# 8.17
$ cargo run -p ch8 --bin process_time "sleep 5" "date" "man bash >/dev/null"
command: sleep 5
real:    5.01
user:    0.00
sys:     0.00
child user:    0.01
child sys:     0.00

command: date
2022年 8月28日 日曜日 22時20分26秒 JST
real:    0.01
user:    0.00
sys:     0.00
child user:    0.00
child sys:     0.01

command: man bash >/dev/null
real:    0.10
user:    0.00
sys:     0.00
child user:    0.12
child sys:     0.03
```
