extern crate libc;
extern crate ncurses;

use libc::{c_int, c_char};
use std::ffi::CString;
use std::env;
use std::process;
use std::ptr;
use std::os::unix::ffi::OsStringExt;

extern "C" {
    fn c_main(argc: c_int, argv: *mut *mut c_char) -> c_int;
}

fn main() {
    let mut argv_storage: Vec<_> = env::args_os()
        .map(|arg| {
            CString::new(arg.into_vec()).expect("Unexpected NUL byte in an argument")
        })
        .collect();
    let mut argv: Vec<_> = argv_storage
        .iter_mut()
        .map(|str| str.as_ptr() as *mut _)
        .chain(Some(ptr::null_mut()))
        .collect();
    process::exit(unsafe {
        c_main(argv_storage.len() as i32, argv.as_mut_ptr())
    });
}
