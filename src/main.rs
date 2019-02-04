#![feature(untagged_unions)]
#![allow(non_camel_case_types)]

use libc;
use libc::c_void;
use std::mem;
use std::ptr;
use std::ffi::CStr;


const MAXCOMLEN: usize = 16;

#[repr(C)]
struct sleep_queue {
    __p_forw: *const c_void,
    __p_back: *const c_void,
}

#[repr(C)]
struct timeval {
    tv_sec: libc::c_long,
    tv_usec: i32,
}

#[repr(C)]
union sleep_queue_start_time_union {
    __p_st1: sleep_queue,
    __p_starttime: timeval,
}


#[repr(C)]
struct itimerval {
    it_interval: timeval,
    it_value: timeval,
}

type u_quad_t = u64;
type sigset_t = u32;

#[repr(C)]
struct extern_proc {
    // union {
    //   struct {
    //     struct  proc *__p_forw;  /* Doubly-linked run/sleep queue. */
    //     struct  proc *__p_back;
    //   } p_st1;
    //   struct timeval __p_starttime;   /* process start time */
    // } p_un;
    p_un: sleep_queue_start_time_union,
    // struct  vmspace *p_vmspace;  /* Address space. */
    p_vmspace: *const c_void,
    // struct  sigacts *p_sigacts;  /* Signal actions, state (PROC ONLY). */
    p_sigacts: *const c_void,
    // int  p_flag;      /* P_* flags. */
    p_flag: libc::c_int,
    // char  p_stat;      /* S* process status. */
    p_stat: char,
    // pid_t  p_pid;      /* Process identifier. */
    p_pid: libc::pid_t,
    // pid_t  p_oppid;   /* Save parent pid during ptrace. XXX */
    p_oppid: libc::pid_t,
    // int  p_dupfd;   /* Sideways return value from fdopen. XXX */
    p_dupfd: libc::c_int,
    // /* Mach related  */
    // caddr_t user_stack;  /* where user stack was allocated */
    user_stack: *const c_void,
    // void  *exit_thread;  /* XXX Which thread is exiting? */
    exit_thread: *const c_void,
    // int    p_debugger;    /* allow to debug */
    p_debugger: libc::c_int,
    // boolean_t  sigwait;  /* indication to suspend */
    sigwait: bool,
    // /* scheduling */
    // u_int  p_estcpu;   /* Time averaged value of p_cpticks. */
    p_estcpu: libc::c_uint,
    // int  p_cpticks;   /* Ticks of cpu time. */
    p_cpticks: libc::c_int,
    // fixpt_t  p_pctcpu;   /* %cpu for this process during p_swtime */
    p_pctcpu: u32,
    // void  *p_wchan;   /* Sleep address. */
    p_wchan: *const c_void,
    // char  *p_wmesg;   /* Reason for sleep. */
    p_wmesg: *const libc::c_void,
    // u_int  p_swtime;   /* Time swapped in or out. */
    p_swtime: libc::c_uint,
    // u_int  p_slptime;   /* Time since last blocked. */
    p_slptime: libc::c_uint,
    // struct  itimerval p_realtimer;  /* Alarm timer. */
    p_realtimer: itimerval,
    // struct  timeval p_rtime;  /* Real time. */
    p_rtime: timeval,
    // u_quad_t p_uticks;    /* Statclock hits in user mode. */
    p_uticks: u_quad_t,
    // u_quad_t p_sticks;    /* Statclock hits in system mode. */
    p_sticks: u_quad_t,
    // u_quad_t p_iticks;    /* Statclock hits processing intr. */
    p_iticks: u_quad_t,
    // int  p_traceflag;    /* Kernel trace points. */
    p_traceflag: libc::c_int,
    // // struct  vnode *p_tracep;  /* Trace to vnode. */
    p_tracep: *const libc::c_void,
    // int  p_siglist;    /* DEPRECATED. */
    p_siglist: libc::c_int,
    // // struct  vnode *p_textvp;  /* Vnode of executable. */
    p_textvp: *const libc::c_void,
    // int  p_holdcnt;    /* If non-zero, don't swap. */
    p_holdcnt: libc::c_int,
    // sigset_t p_sigmask;  /* DEPRECATED. */
    // XXX <- should have offset 228
    p_sigmask: sigset_t,
    // sigset_t p_sigignore;  /* Signals being ignored. */
    p_sigignore: sigset_t,
    // sigset_t p_sigcatch;  /* Signals being caught by user. */
    p_sigcatch: sigset_t,
    // u_char  p_priority;  /* Process priority. */
    p_priority: libc::c_uchar,
    // // u_char  p_usrpri;  /* User-priority based on p_cpu and p_nice. */
    p_usrpri: libc::c_uchar,
    // char  p_nice;    /* Process "nice" value. */
    p_nice: libc::c_char,
    // char  p_comm[MAXCOMLEN+1];
    p_comm: [libc::c_char; MAXCOMLEN + 1],
    _padding: [libc::c_uchar; 648 - 264 - 352],
    // struct   pgrp *p_pgrp;  /* Pointer to process group. */
    // struct  user *p_addr;  /* Kernel virtual addr of u-area (PROC ONLY). */
    // u_short  p_xstat;  /* Exit status for wait; also stop signal. */
    // u_short  p_acflag;  /* Accounting flags. */
    // struct  rusage *p_ru;  /* Exit information. XXX */
}

struct kinfo_proc {
    kp_proc: extern_proc,
    _kp_eproc: [libc::c_uchar; 352],
}

fn main() {
    // dbg!(mem::size_of::<extern_proc>());
    // panic!();
    let mut name: [libc::c_int; 3] = [libc::CTL_KERN, libc::KERN_PROC, libc::KERN_PROC_ALL];
    let mut length: libc::size_t = 0;
    let mut err = unsafe {
        libc::sysctl(
            name.as_mut_ptr(),
            name.len() as u32,
            ptr::null_mut(),
            &mut length as *mut libc::size_t,
            ptr::null_mut(),
            0,
        )
    };

    if err != 0 {
        panic!();
    }

    let mut buf: Vec<libc::c_uchar> = vec![0; length];

    err = unsafe {
        libc::sysctl(
            name.as_mut_ptr(),
            name.len() as u32,
            buf.as_mut_ptr() as *mut libc::c_void,
            &mut length as *mut libc::size_t,
            ptr::null_mut(),
            0,
        )
    };

    if err != 0 {
        panic!();
    }
    let struct_ptr = buf.as_mut_ptr() as *const kinfo_proc;
    let n = length / mem::size_of::<kinfo_proc>();
    let procs = unsafe { std::slice::from_raw_parts(struct_ptr, n) };
    for proc in procs {
        let comm = unsafe { CStr::from_ptr(proc.kp_proc.p_comm.as_ptr()) };
        let comm = comm.to_str().unwrap();
        println!("{} - {}", proc.kp_proc.p_pid, comm);

        let mut argmax: libc::size_t = 0;
        let mut size = mem::size_of::<libc::size_t>();
        let mut name: [libc::c_int; 2] = [libc::CTL_KERN, libc::KERN_ARGMAX];
        err = unsafe {
            libc::sysctl(
                name.as_mut_ptr(),
                name.len() as u32,
                &mut argmax as *mut _ as *mut libc::c_void,
                &mut size as *mut libc::size_t,
                ptr::null_mut(),
                0,
            )
        };
        if err != 0 {
            panic!();
        }

        let pid = proc.kp_proc.p_pid;
        let mut name: [libc::c_int; 3] = [libc::CTL_KERN, libc::KERN_PROCARGS2, pid];
        let mut buf: Vec<libc::c_uchar> = vec![0; argmax as usize];
        err = unsafe {
            libc::sysctl(
                name.as_mut_ptr(),
                name.len() as u32,
                buf.as_mut_ptr() as *mut libc::c_void,
                &mut argmax as *mut _ as *mut libc::size_t,
                ptr::null_mut(),
                0,
            )
        };
        if err == 0 {
        }
    }
}
