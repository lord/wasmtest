use std::os::raw::{c_double, c_int};

extern "C" {
    fn meow(_: c_int) -> c_int;
}

#[no_mangle]
pub extern "C" fn add(a: c_int, b: c_int) -> c_int {
    unsafe { meow(a + b) }
}
