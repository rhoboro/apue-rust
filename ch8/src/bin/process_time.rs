use nix::libc::{sysconf, system, times, tms, _SC_CLK_TCK};
use std::ffi::CString;
use std::mem;

fn main() {
    for cmd in std::env::args().skip(1) {
        do_cmd(&cmd);
    }
}

fn do_cmd(cmd: &String) {
    let (start, start_tms) = unsafe {
        let mut start_tms = mem::MaybeUninit::<tms>::uninit();
        let s = times(start_tms.as_mut_ptr());
        (s, start_tms.assume_init())
    };
    println!("\ncommand: {}", cmd);
    let command = CString::new(cmd.as_bytes()).unwrap();
    unsafe {
        system(command.as_ptr());
    }
    let (end, end_tms) = unsafe {
        let mut end_tms = mem::MaybeUninit::<tms>::uninit();
        let s = times(end_tms.as_mut_ptr());
        (s, end_tms.assume_init())
    };
    pr_times(end - start, &start_tms, &end_tms);
}

fn pr_times(real: u64, tmsstart: &tms, tmsend: &tms) {
    let clktck = unsafe { sysconf(_SC_CLK_TCK) };
    println!("real: {:7.2}", real as f64 / clktck as f64);
    println!(
        "user: {:7.2}",
        (tmsend.tms_utime - tmsstart.tms_utime) as f64 / clktck as f64
    );
    println!(
        "sys:  {:7.2}",
        (tmsend.tms_stime - tmsstart.tms_stime) as f64 / clktck as f64
    );
    println!(
        "child user: {:7.2}",
        (tmsend.tms_cutime - tmsstart.tms_cutime) as f64 / clktck as f64
    );
    println!(
        "child sys:  {:7.2}",
        (tmsend.tms_cstime - tmsstart.tms_cstime) as f64 / clktck as f64
    );
}
