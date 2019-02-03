#![feature(untagged_unions)]
#![allow(non_camel_case_types)]

use libc;
use libc::c_void;
use std::mem;
use std::ptr;

#[repr(C)]
struct sleep_queue {
    __p_forw: *const c_void,
    __p_back: *const c_void,
}

#[repr(C)]
struct timeval {
    tv_sec: i32,
    tv_usec: i32,
}

#[repr(C)]
union sleep_queue_start_time_union {
    __p_st1: sleep_queue,
    __p_starttime: timeval,
}

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
    _padding: [libc::c_uchar; 648 - 48 - 352],
    // pid_t  p_oppid;   /* Save parent pid during ptrace. XXX */
    // int  p_dupfd;   /* Sideways return value from fdopen. XXX */
    // /* Mach related  */
    // caddr_t user_stack;  /* where user stack was allocated */
    // void  *exit_thread;  /* XXX Which thread is exiting? */
    // int    p_debugger;    /* allow to debug */
    // boolean_t  sigwait;  /* indication to suspend */
    // /* scheduling */
    // u_int  p_estcpu;   /* Time averaged value of p_cpticks. */
    // int  p_cpticks;   /* Ticks of cpu time. */
    // fixpt_t  p_pctcpu;   /* %cpu for this process during p_swtime */
    // void  *p_wchan;   /* Sleep address. */
    // char  *p_wmesg;   /* Reason for sleep. */
    // u_int  p_swtime;   /* Time swapped in or out. */
    // u_int  p_slptime;   /* Time since last blocked. */
    // struct  itimerval p_realtimer;  /* Alarm timer. */
    // struct  timeval p_rtime;  /* Real time. */
    // u_quad_t p_uticks;    /* Statclock hits in user mode. */
    // u_quad_t p_sticks;    /* Statclock hits in system mode. */
    // u_quad_t p_iticks;    /* Statclock hits processing intr. */
    // int  p_traceflag;    /* Kernel trace points. */
    // struct  vnode *p_tracep;  /* Trace to vnode. */
    // int  p_siglist;    /* DEPRECATED. */
    // struct  vnode *p_textvp;  /* Vnode of executable. */
    // int  p_holdcnt;    /* If non-zero, don't swap. */
    // sigset_t p_sigmask;  /* DEPRECATED. */
    // sigset_t p_sigignore;  /* Signals being ignored. */
    // sigset_t p_sigcatch;  /* Signals being caught by user. */
    // u_char  p_priority;  /* Process priority. */
    // u_char  p_usrpri;  /* User-priority based on p_cpu and p_nice. */
    // char  p_nice;    /* Process "nice" value. */
    // char  p_comm[MAXCOMLEN+1];
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
        dbg!(proc.kp_proc.p_pid);
    }
}
