extern "C" {
    fn browse_draw();
    fn nccreate(arg1: i32, arg2: i32, arg3: *const u8);
    static mut pstate: i32;
    static mut stdscr: *mut _win_st;
    static mut subwinc: i32;
    static mut subwinr: i32;
    fn waddnstr(arg1: *mut _win_st, arg2: *const u8, arg3: i32) -> i32;
    fn wmove(arg1: *mut _win_st, arg2: i32, arg3: i32) -> i32;
}

enum _win_st {
}

#[no_mangle]
pub static mut confirm_quit: i32 = 0i32;

#[no_mangle]
pub static mut dir_process: Option<unsafe extern "C" fn() -> i32> = None;

#[no_mangle]
pub unsafe extern "C" fn quit_key(mut ch: i32) -> i32 {
    if ch == b'Y' as (i32) || ch == b'y' as (i32) {
        1i32
    } else {
        pstate = 1i32;
        0i32
    }
}

#[no_mangle]
pub unsafe extern "C" fn quit_draw() {
    browse_draw();
    nccreate(4i32, 30i32, (*b"ncdu confirm quit\0").as_ptr());
    if wmove(stdscr, subwinr + 2i32, subwinc + 2i32) == -1i32 {
        -1i32;
    } else {
        waddnstr(stdscr, (*b"Really quit? (y/N)\0").as_ptr(), -1i32);
    }
}

#[no_mangle]
pub unsafe extern "C" fn quit_init() {
    pstate = 5i32;
}
