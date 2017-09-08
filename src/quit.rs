extern "C" {
    fn browse_draw();
    fn ncaddstr(r: i32, c: i32, s: *const u8) -> i32;
    fn nccreate(arg1: i32, arg2: i32, arg3: *const u8);
    static mut pstate: i32;
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
    ncaddstr(2i32, 2i32, (*b"Really quit? (y/N)\0").as_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn quit_init() {
    pstate = 5i32;
}
