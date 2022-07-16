```sh
# 3.6
# use lseek
$ cargo run -p ch3 --bin lseek < /etc/passwd
seek OK
$ cat /etc/passwd| cargo run -p ch3 --bin lseek
cannot seek

# use Rust std::fs
$ cargo run -p ch3 --bin sparse
$ od -c file.hole
0000000    a   b   c   d   e   f   g   h   i   j  \0  \0  \0  \0  \0  \0
0000020   \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
*
0040000    A   B   C   D   E   F   G   H   I   J
0040012
# Rust std::fs::File::seek uses 40 blacks?
$ ls -sla file.hole
40 -rw-r--r--  1 rhoboro  staff  16394  7 16 22:20 file.hole

# 3.9
$ cargo run -p ch3 --bin cp1 < README.md > README.md.bk
# use Rust std::io
$ cargo run -p ch3 --bin cp2 < README.md > README.md.bk

# 3.14
$ cargo run -p ch3 --bin fcntl 0 < /dev/tty
read only
$ cargo run -p ch3 --bin fcntl 1 > tmp.foo
$ cat tmp.foo
write only
```
