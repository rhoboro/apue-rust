```sh
# 4.3
$ cargo run -p ch4 --bin stat Cargo.lock target /etc/passwd /etc /dev/tty
Cargo.lock :regular
target :directory
/etc/passwd :regular
/etc :symbolic link
/dev/tty :character special

# 4.7
$ echo "echo 'hoge'" > hoge
$ sudo chown root:wheel hoge
$ sudo chmod 775 hoge
$ sudo chmod u+s hoge
$ ls -la hoge
-rwsrwxr-x  1 root  wheel  12  7 24 18:56 hoge
$ cargo run -p ch4 --bin access ./hoge
read access OK
open for reading OK  # I don't understand this behavior on macos.

$ cargo run -p ch4 --bin access README.md
read access OK
open for reading OK

# 4.8
$ umask
022
$ ls -la foo bar
ls: bar: No such file or directory
ls: foo: No such file or directory
$ cargo run -p ch4 --bin umask
$ ls -la foo bar
-rw-------  1 rhoboro  staff  0  7 24 19:31 bar
-rw-rw-rw-  1 rhoboro  staff  0  7 24 19:31 foo
$ umask
022

# 4.9
$ cargo run -p ch4 --bin chmod
-rw-r--r--  1 rhoboro  staff  0  7 24 20:07 bar
-rwsrwSrwt  1 rhoboro  staff  0  7 24 20:07 foo

# 4.15
$ echo tempfile > tempfile
$ cargo run -p ch4 --bin unlink

# 4.20
$ cp README.md README.md.bk
$ ls -l README.md.bk
-rw-r--r--  1 rhoboro  staff  130  7 28 09:14 README.md.bk
$ cargo run -p ch4 --bin futimens README.md.bk
$ ls -l README.md.bk
-rw-r--r--  1 rhoboro  staff  0  7 28 09:14 README.md.bk

# 4.22
**** TODO ****

# 4.23
$ pwd
/Users/rhoboro/go/src/github.com/rhoboro/apue-rust
$ cargo run -p ch4 --bin mycd
chdir to /tmp succeeded
$ pwd
/Users/rhoboro/go/src/github.com/rhoboro/apue-rust

$ cargo run -p ch4 --bin getcwd
cwd = "/private/tmp"
$ ls -l /tmp
lrwxr-xr-x@ 1 root  wheel  11  5 10 06:30 /tmp -> private/tmp

# 4.24
# major/minor are available only on linux.
$ cargo run -p ch4 --bin dev /dev/tty /dev/ttyp0
/dev/tty: dev = -1043388647 (character) rdev = 33554432
/dev/ttyp0: dev = -1043388647 (character) rdev = 67108864
```
