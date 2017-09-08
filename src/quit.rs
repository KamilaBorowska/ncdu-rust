/* ncdu - NCurses Disk Usage

  Copyright (c) 2015-2016 Yoran Heling

  Permission is hereby granted, free of charge, to any person obtaining
  a copy of this software and associated documentation files (the
  "Software"), to deal in the Software without restriction, including
  without limitation the rights to use, copy, modify, merge, publish,
  distribute, sublicense, and/or sell copies of the Software, and to
  permit persons to whom the Software is furnished to do so, subject to
  the following conditions:

  The above copyright notice and this permission notice shall be included
  in all copies or substantial portions of the Software.

  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
  EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
  MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
  IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
  CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
  TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
  SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

*/

extern "C" {
    fn browse_draw();
    fn ncaddstr(r: i32, c: i32, s: *const u8) -> i32;
    fn nccreate(arg1: i32, arg2: i32, arg3: *const u8);
    static mut pstate: i32;
}

#[no_mangle]
pub static mut confirm_quit: i32 = 0;

#[no_mangle]
pub static mut dir_process: Option<unsafe extern "C" fn() -> i32> = None;

#[no_mangle]
pub unsafe extern "C" fn quit_key(mut ch: i32) -> i32 {
    if ch == b'Y' as (i32) || ch == b'y' as (i32) {
        1
    } else {
        pstate = 1;
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
    pstate = 5;
}
