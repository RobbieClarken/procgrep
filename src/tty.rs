use std::ffi::CString;

pub fn get_tty(path: &str) -> Option<i32> {
    let path = match path.starts_with("/") {
        true => String::from(path),
        false => format!("/dev/{}", path),
    };
    let path = CString::new(path).unwrap();
    let mut stat: libc::stat = unsafe { std::mem::zeroed() };
    match unsafe { libc::stat(path.as_ptr(), &mut stat) } {
        0 => Some(stat.st_rdev),
        _ => None,
    }
}
