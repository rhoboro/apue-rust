use nix::sys::signal::{sigaction, SaFlags, SigAction, SigHandler, SigSet, SIGINT};
use nix::sys::wait::waitpid;
use nix::unistd::{execvp, fork, ForkResult};
use std::ffi::CString;
use std::io;
use std::io::prelude::*;

extern "C" fn sig_int(_: i32) {
    let stdout = io::stdout();
    let mut out = stdout.lock();
    write!(out, "interrupt\n").unwrap();
    write!(out, "% ").unwrap();
    out.flush().unwrap();
}

fn main() {
    let sig_action = SigAction::new(
        // 自作ハンドラ登録以外に下記が使える
        // SigHandler::SigIgn: 無視
        // SigHandler::SigDfl: デフォルト
        SigHandler::Handler(sig_int),
        // 図10.16
        SaFlags::empty(),
        // シグナル生起中に追加で無視するシグナル
        // 生起中シグナルは自動で追加される
        SigSet::empty(),
    );
    unsafe {
        sigaction(SIGINT, &sig_action).unwrap();
    }
    let stdout = io::stdout();
    {
        let mut out = stdout.lock();
        write!(out, "% ").unwrap();
        out.flush().unwrap();
    }

    let stdin = io::stdin();
    while let Some(Ok(line)) = stdin.lock().lines().next() {
        if !line.is_empty() {
            match unsafe { fork() } {
                Ok(ForkResult::Parent { child, .. }) => {
                    waitpid(child, None).unwrap();
                }
                Ok(ForkResult::Child) => {
                    let cmd = line
                        .split(" ")
                        .map(|c| CString::new(c).unwrap())
                        .collect::<Vec<_>>();
                    execvp(&cmd[0], &cmd).unwrap();
                    std::process::exit(0);
                }
                Err(_) => {
                    std::process::exit(1);
                }
            }
        }
        {
            let mut out = stdout.lock();
            write!(out, "% ").unwrap();
            out.flush().unwrap();
        }
    }
    std::process::exit(0);
}
