use libc;
use std::ptr;

fn main() {
    let mut name: [libc::c_int; 3] = [libc::CTL_KERN, libc::KERN_PROC, libc::KERN_PROC_ALL];
    let mut length: libc::size_t = 0;
    let err = unsafe {
        libc::sysctl(
            name.as_mut_ptr(),
            name.len() as u32,
            ptr::null_mut(),
            &mut length as *mut libc::size_t,
            ptr::null_mut(),
            0,
        )
    };
    dbg!(err);
    dbg!(length);
}
