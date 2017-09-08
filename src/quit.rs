extern "C" {
    fn browse_draw();
    fn ncaddstr(r: i32, c: i32, s: *const u8) -> i32;
    fn nccreate(arg1: i32, arg2: i32, arg3: *const u8);
    static mut pstate: ProgramState;
}

#[no_mangle]
pub static mut CONFIRM_QUIT: i32 = 0;

#[no_mangle]
pub static mut DIR_PROCESS: Option<unsafe extern "C" fn() -> i32> = None;

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum ProgramState {
    Calc,
    Browse,
    Del,
    Help,
    Shell,
    Quit,
}

#[no_mangle]
pub unsafe extern "C" fn quit_key(ch: i32) -> i32 {
    if ch == b'Y' as (i32) || ch == b'y' as (i32) {
        1
    } else {
        pstate = ProgramState::Browse;
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn quit_draw() {
    browse_draw();
    nccreate(4, 30, (*b"ncdu confirm quit\0").as_ptr());
    ncaddstr(2, 2, (*b"Really quit? (y/N)\0").as_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn quit_init() {
    pstate = ProgramState::Quit;
}
