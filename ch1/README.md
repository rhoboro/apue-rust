```sh
# 1.4
$ cargo run -p ch1 --bin ls .
.
..
Cargo.toml
ch1
README.md.bk
target
Cargo.lock
README.md
.gitignore
.git
.idea

# 1.5
$ cargo run -p ch1 --bin cp1 < README.md > README.md.bk

# 1.6
$ cargo run -p ch1 --bin getpid
88091

$ cargo run -p ch1 --bin process
% ls
Cargo.lock  Cargo.toml  README.md     ch1     target
% pwd
/Users/rhoboro/go/src/github.com/rhoboro/apue-rust
% ^C

# 1.8
$ cargo run -p ch1 --bin uid_gid
uid = 1001, gid = 20

# 1.9
$ cargo run -p ch1 --bin signal
% ls
Cargo.lock  Cargo.toml  README.md     ch1     target
% pwd
/Users/rhoboro/go/src/github.com/rhoboro/apue-rust
% ^Cinterrupt
% ^Cinterrupt
% ^D
```
