#![allow(non_camel_case_types)]

use libc::{c_char, c_int, c_long, c_short, c_uchar, c_uint, c_ushort, c_void};
use libc::{dev_t, gid_t, pid_t, uid_t};
use std::ffi::CStr;
use std::mem;
use std::ptr;

const MAXCOMLEN: usize = 16;
const NGROUPS: usize = 16;

#[repr(C)]
struct sleep_queue {
    __p_forw: *const c_void,
    __p_back: *const c_void,
}

#[repr(C)]
struct timeval {
    tv_sec: c_long,
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
    p_flag: c_int,
    // char  p_stat;      /* S* process status. */
    p_stat: char,
    // pid_t  p_pid;      /* Process identifier. */
    p_pid: pid_t,
    // pid_t  p_oppid;   /* Save parent pid during ptrace. XXX */
    p_oppid: pid_t,
    // int  p_dupfd;   /* Sideways return value from fdopen. XXX */
    p_dupfd: c_int,
    // /* Mach related  */
    // caddr_t user_stack;  /* where user stack was allocated */
    user_stack: *const c_void,
    // void  *exit_thread;  /* XXX Which thread is exiting? */
    exit_thread: *const c_void,
    // int    p_debugger;    /* allow to debug */
    p_debugger: c_int,
    // boolean_t  sigwait;  /* indication to suspend */
    sigwait: bool,
    // /* scheduling */
    // u_int  p_estcpu;   /* Time averaged value of p_cpticks. */
    p_estcpu: c_uint,
    // int  p_cpticks;   /* Ticks of cpu time. */
    p_cpticks: c_int,
    // fixpt_t  p_pctcpu;   /* %cpu for this process during p_swtime */
    p_pctcpu: u32,
    // void  *p_wchan;   /* Sleep address. */
    p_wchan: *const c_void,
    // char  *p_wmesg;   /* Reason for sleep. */
    p_wmesg: *const c_void,
    // u_int  p_swtime;   /* Time swapped in or out. */
    p_swtime: c_uint,
    // u_int  p_slptime;   /* Time since last blocked. */
    p_slptime: c_uint,
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
    p_traceflag: c_int,
    // // struct  vnode *p_tracep;  /* Trace to vnode. */
    p_tracep: *const c_void,
    // int  p_siglist;    /* DEPRECATED. */
    p_siglist: c_int,
    // // struct  vnode *p_textvp;  /* Vnode of executable. */
    p_textvp: *const c_void,
    // int  p_holdcnt;    /* If non-zero, don't swap. */
    p_holdcnt: c_int,
    // sigset_t p_sigmask;  /* DEPRECATED. */
    p_sigmask: sigset_t,
    // sigset_t p_sigignore;  /* Signals being ignored. */
    p_sigignore: sigset_t,
    // sigset_t p_sigcatch;  /* Signals being caught by user. */
    p_sigcatch: sigset_t,
    // u_char  p_priority;  /* Process priority. */
    p_priority: c_uchar,
    // // u_char  p_usrpri;  /* User-priority based on p_cpu and p_nice. */
    p_usrpri: c_uchar,
    // char  p_nice;    /* Process "nice" value. */
    p_nice: c_char,
    // char  p_comm[MAXCOMLEN+1];
    p_comm: [c_char; MAXCOMLEN + 1],
    // struct   pgrp *p_pgrp;  /* Pointer to process group. */
    p_pgrp: *const c_void,
    // struct  user *p_addr;  /* Kernel virtual addr of u-area (PROC ONLY). */
    p_addr: *const c_void,
    // u_short  p_xstat;  /* Exit status for wait; also stop signal. */
    p_xstat: c_ushort,
    // u_short  p_acflag;  /* Accounting flags. */
    p_acflag: c_ushort,
    // struct  rusage *p_ru;  /* Exit information. XXX */
    p_ru: *const c_void,
}

#[repr(C)]
struct _pcred {
    // char pc_lock[72];  /* opaque content */
    pc_lock: [c_char; 72],
    // struct ucred *pc_ucred; /* Current credentials. */
    pc_ucred: *const c_void,
    // uid_t p_ruid;   /* Real user id. */
    p_ruid: uid_t,
    // uid_t p_svuid;  /* Saved effective user id. */
    p_svuid: uid_t,
    // gid_t p_rgid;   /* Real group id. */
    p_rgid: gid_t,
    // gid_t p_svgid;  /* Saved effective group id. */
    p_svgid: gid_t,
    // int p_refcnt;  /* Number of references. */
    p_refcnt: c_int,
}

#[repr(C)]
struct _ucred {
    // int32_t cr_ref;   /* reference count */
    cr_ref: i32,
    // uid_t cr_uid;   /* effective user id */
    cr_uid: uid_t,
    // short cr_ngroups;  /* number of groups */
    cr_ngroups: c_short,
    // gid_t cr_groups[NGROUPS]; /* groups */
    cr_groups: [gid_t; NGROUPS],
}

#[repr(C)]
struct vmspace {
    // int32_t dummy;
    dummy: i32,
    // caddr_t dummy2;
    dummy2: *const c_void,
    // int32_t dummy3[5];
    dummy3: [i32; 5],
    // caddr_t dummy4[3];
    dummy4: [*const c_void; 3],
}

#[repr(C)]
struct eproc {
    // struct proc *e_paddr;  /* address of proc */
    e_paddr: *const c_void,
    // struct session *e_sess; /* session pointer */
    e_sess: *const c_void,
    // struct _pcred e_pcred;  /* process credentials */
    e_pcred: _pcred,
    // struct _ucred e_ucred;  /* current credentials */
    e_ucred: _ucred,
    // struct  vmspace e_vm;  /* address space */
    e_vm: vmspace,
    // pid_t e_ppid;   /* parent process id */
    e_ppid: pid_t,
    // pid_t e_pgid;   /* process group id */
    e_pgid: pid_t,
    // short e_jobc;   /* job control counter */
    e_jobc: c_short,
    // dev_t e_tdev;   /* controlling tty dev */
    e_tdev: dev_t,
    // pid_t e_tpgid;  /* tty process group id */
    e_tpgid: pid_t,
    // struct session *e_tsess; /* tty session pointer */
    session: *const c_void,
    // char e_wmesg[8]; /* wchan message */
    e_wmesg: [c_char; 8],
    // segsz_t e_xsize;  /* text size */
    e_xsize: i32,
    // short e_xrssize;  /* text rss */
    e_xrssize: c_short,
    // short e_xccount;  /* text references */
    e_xccount: c_short,
    // short e_xswrss;
    e_xswrss: c_short,
    // int32_t e_flag;
    e_flag: i32,
    // char e_login[12]; /* short setlogin() name */
    e_login: [c_char; 12],
    // int32_t e_spare[4];
    e_spare: [i32; 4],
}

#[repr(C)]
struct kinfo_proc {
    kp_proc: extern_proc,
    kp_eproc: eproc,
}

pub struct Process {
    pub pid: i64,
    pub uid: i64,
    pub gid: i64,
    pub ppid: i64,
    pub tty: i32,
    pub command: String,
    pub args: Option<Vec<String>>,
}

pub fn get_processes() -> Vec<Process> {
    let mut name = [libc::CTL_KERN, libc::KERN_PROC, libc::KERN_PROC_ALL];
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

    let mut buf: Vec<c_uchar> = vec![0; length];

    err = unsafe {
        libc::sysctl(
            name.as_mut_ptr(),
            name.len() as u32,
            buf.as_mut_ptr() as *mut c_void,
            &mut length as *mut libc::size_t,
            ptr::null_mut(),
            0,
        )
    };

    if err != 0 {
        panic!();
    }
    let struct_ptr = buf.as_ptr() as *const kinfo_proc;
    let n = length / mem::size_of::<kinfo_proc>();
    let procs = unsafe { std::slice::from_raw_parts(struct_ptr, n) };

    let mut processes = Vec::new();

    for proc in procs {
        let comm = unsafe { CStr::from_ptr(proc.kp_proc.p_comm.as_ptr()) };
        let comm = comm.to_str().unwrap();

        let mut argmax: libc::size_t = 0;
        let mut size = mem::size_of::<libc::size_t>();
        let mut name: [c_int; 2] = [libc::CTL_KERN, libc::KERN_ARGMAX];
        err = unsafe {
            libc::sysctl(
                name.as_mut_ptr(),
                name.len() as u32,
                &mut argmax as *mut _ as *mut c_void,
                &mut size as *mut libc::size_t,
                ptr::null_mut(),
                0,
            )
        };
        if err != 0 {
            panic!();
        }

        let pid = proc.kp_proc.p_pid;
        let mut name: [c_int; 3] = [libc::CTL_KERN, libc::KERN_PROCARGS2, pid];
        let mut buf: Vec<c_uchar> = vec![0; argmax as usize];
        err = unsafe {
            libc::sysctl(
                name.as_mut_ptr(),
                name.len() as u32,
                buf.as_mut_ptr() as *mut c_void,
                &mut argmax as *mut _ as *mut libc::size_t,
                ptr::null_mut(),
                0,
            )
        };
        let mut optional_args = None;
        if err == 0 {
            let buf_ptr = buf.as_ptr();
            let mut off = 0;
            let nargs = unsafe { buf_ptr.read() as c_int };
            off += mem::size_of_val(&nargs);
            let exec_path = unsafe { CStr::from_ptr(buf_ptr.add(off) as *const i8) };
            off += exec_path.to_bytes().len() + 1;
            while unsafe { buf_ptr.add(off).read() } == 0 && off < buf.len() {
                off += 1;
            }
            if off >= buf.len() {
                panic!();
            }
            let mut args: Vec<String> = Vec::new();
            for _ in 0..nargs {
                let arg = unsafe { CStr::from_ptr(buf_ptr.add(off) as *const i8) };
                args.push(String::from(arg.to_str().unwrap()));
                off += arg.to_bytes().len() + 1;
            }
            optional_args = Some(args);
        }
        processes.push(Process {
            pid: proc.kp_proc.p_pid as i64,
            uid: proc.kp_eproc.e_pcred.p_ruid as i64,
            gid: proc.kp_eproc.e_pcred.p_rgid as i64,
            ppid: proc.kp_eproc.e_ppid as i64,
            tty: proc.kp_eproc.e_tdev,
            command: String::from(comm),
            args: optional_args,
        });
    }
    processes
}

