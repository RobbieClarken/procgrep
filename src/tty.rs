use std::ffi::CString;

pub fn get_tty(path: &str) -> i32 {
    let path = match path.starts_with("/") {
        true => String::from(path),
        false => format!("/dev/{}", path),
    };
    let path = CString::new(path).unwrap();
    let mut stat: libc::stat = unsafe { std::mem::zeroed() };
    if unsafe { libc::stat(path.as_ptr(), &mut stat) } != 0 {
        panic!();
    }
    stat.st_rdev
}
