use crate::libbb::ptr_to_globals::bb_errno;
use crate::librb::signal::__sighandler_t;

use libc;
use libc::getenv;
use libc::pid_t;
use libc::printf;
use libc::timeval;
use libc::rusage;

extern "C" {
  fn vfork() -> libc::c_int;
  static mut optind: libc::c_int;
  fn getpagesize() -> libc::c_int;
  fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;

  fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
  fn wait3(__stat_loc: *mut libc::c_int, __options: libc::c_int, __usage: *mut rusage) -> pid_t;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct resource_t {
  pub waitstatus: libc::c_int,
  pub ru: rusage,
  pub elapsed_ms: libc::c_uint,
}

pub const OPT_a: C2RustUnnamed_13 = 4;
pub const OPT_o: C2RustUnnamed_13 = 8;
pub const OPT_p: C2RustUnnamed_13 = 2;
pub const OPT_v: C2RustUnnamed_13 = 1;
pub type C2RustUnnamed_13 = libc::c_uint;
pub const OPT_f: C2RustUnnamed_13 = 16;

/* msec = milliseconds = 1/1,000 (1*10e-3) second.
usec = microseconds = 1/1,000,000 (1*10e-6) second.  */
static mut default_format: [libc::c_char; 23] = [
  114, 101, 97, 108, 9, 37, 69, 10, 117, 115, 101, 114, 9, 37, 117, 10, 115, 121, 115, 9, 37, 84, 0,
];
/* The output format for the -p option .*/
static mut posix_format: [libc::c_char; 23] = [
  114, 101, 97, 108, 32, 37, 101, 10, 117, 115, 101, 114, 32, 37, 85, 10, 115, 121, 115, 32, 37,
  83, 0,
];
/* Format string for printing all statistics verbosely.
Keep this output to 24 lines so users on terminals can see it all.*/
static mut long_format: [libc::c_char; 715] = [
  9, 67, 111, 109, 109, 97, 110, 100, 32, 98, 101, 105, 110, 103, 32, 116, 105, 109, 101, 100, 58,
  32, 34, 37, 67, 34, 10, 9, 85, 115, 101, 114, 32, 116, 105, 109, 101, 32, 40, 115, 101, 99, 111,
  110, 100, 115, 41, 58, 32, 37, 85, 10, 9, 83, 121, 115, 116, 101, 109, 32, 116, 105, 109, 101,
  32, 40, 115, 101, 99, 111, 110, 100, 115, 41, 58, 32, 37, 83, 10, 9, 80, 101, 114, 99, 101, 110,
  116, 32, 111, 102, 32, 67, 80, 85, 32, 116, 104, 105, 115, 32, 106, 111, 98, 32, 103, 111, 116,
  58, 32, 37, 80, 10, 9, 69, 108, 97, 112, 115, 101, 100, 32, 40, 119, 97, 108, 108, 32, 99, 108,
  111, 99, 107, 41, 32, 116, 105, 109, 101, 32, 40, 104, 58, 109, 109, 58, 115, 115, 32, 111, 114,
  32, 109, 58, 115, 115, 41, 58, 32, 37, 69, 10, 9, 65, 118, 101, 114, 97, 103, 101, 32, 115, 104,
  97, 114, 101, 100, 32, 116, 101, 120, 116, 32, 115, 105, 122, 101, 32, 40, 107, 98, 121, 116,
  101, 115, 41, 58, 32, 37, 88, 10, 9, 65, 118, 101, 114, 97, 103, 101, 32, 117, 110, 115, 104, 97,
  114, 101, 100, 32, 100, 97, 116, 97, 32, 115, 105, 122, 101, 32, 40, 107, 98, 121, 116, 101, 115,
  41, 58, 32, 37, 68, 10, 9, 65, 118, 101, 114, 97, 103, 101, 32, 115, 116, 97, 99, 107, 32, 115,
  105, 122, 101, 32, 40, 107, 98, 121, 116, 101, 115, 41, 58, 32, 37, 112, 10, 9, 65, 118, 101,
  114, 97, 103, 101, 32, 116, 111, 116, 97, 108, 32, 115, 105, 122, 101, 32, 40, 107, 98, 121, 116,
  101, 115, 41, 58, 32, 37, 75, 10, 9, 77, 97, 120, 105, 109, 117, 109, 32, 114, 101, 115, 105,
  100, 101, 110, 116, 32, 115, 101, 116, 32, 115, 105, 122, 101, 32, 40, 107, 98, 121, 116, 101,
  115, 41, 58, 32, 37, 77, 10, 9, 65, 118, 101, 114, 97, 103, 101, 32, 114, 101, 115, 105, 100,
  101, 110, 116, 32, 115, 101, 116, 32, 115, 105, 122, 101, 32, 40, 107, 98, 121, 116, 101, 115,
  41, 58, 32, 37, 116, 10, 9, 77, 97, 106, 111, 114, 32, 40, 114, 101, 111, 117, 105, 114, 105,
  110, 103, 32, 73, 47, 79, 41, 32, 112, 97, 103, 101, 32, 102, 97, 117, 108, 116, 115, 58, 32, 37,
  70, 10, 9, 77, 105, 110, 111, 114, 32, 40, 114, 101, 99, 108, 97, 105, 109, 105, 110, 103, 32,
  97, 32, 102, 114, 97, 109, 101, 41, 32, 112, 97, 103, 101, 32, 102, 97, 117, 108, 116, 115, 58,
  32, 37, 82, 10, 9, 86, 111, 108, 117, 110, 116, 97, 114, 121, 32, 99, 111, 110, 116, 101, 120,
  116, 32, 115, 119, 105, 116, 99, 104, 101, 115, 58, 32, 37, 119, 10, 9, 73, 110, 118, 111, 108,
  117, 110, 116, 97, 114, 121, 32, 99, 111, 110, 116, 101, 120, 116, 32, 115, 119, 105, 116, 99,
  104, 101, 115, 58, 32, 37, 99, 10, 9, 83, 119, 97, 112, 115, 58, 32, 37, 87, 10, 9, 70, 105, 108,
  101, 32, 115, 121, 115, 116, 101, 109, 32, 105, 110, 112, 117, 116, 115, 58, 32, 37, 73, 10, 9,
  70, 105, 108, 101, 32, 115, 121, 115, 116, 101, 109, 32, 111, 117, 116, 112, 117, 116, 115, 58,
  32, 37, 79, 10, 9, 83, 111, 99, 107, 101, 116, 32, 109, 101, 115, 115, 97, 103, 101, 115, 32,
  115, 101, 110, 116, 58, 32, 37, 115, 10, 9, 83, 111, 99, 107, 101, 116, 32, 109, 101, 115, 115,
  97, 103, 101, 115, 32, 114, 101, 99, 101, 105, 118, 101, 100, 58, 32, 37, 114, 10, 9, 83, 105,
  117, 110, 97, 108, 115, 32, 100, 101, 108, 105, 118, 101, 114, 101, 100, 58, 32, 37, 107, 10, 9,
  80, 97, 103, 101, 32, 115, 105, 122, 101, 32, 40, 98, 121, 116, 101, 115, 41, 58, 32, 37, 90, 10,
  9, 69, 120, 105, 116, 32, 115, 116, 97, 116, 117, 115, 58, 32, 37, 120, 0,
];

unsafe extern "C" fn resuse_end(mut pid: pid_t, mut resp: *mut resource_t) {
  let mut caught: pid_t = 0;
  loop {
    caught = wait3(&mut (*resp).waitstatus, 0, &mut (*resp).ru);
    if !(caught != pid) {
      break;
    }
    if caught == -1i32 && *bb_errno != 4i32 {
      crate::libbb::perror_msg::bb_simple_perror_msg(
        b"wait\x00" as *const u8 as *const libc::c_char,
      );
      return;
    }
  }
  (*resp).elapsed_ms = crate::libbb::time::monotonic_ms()
    .wrapping_sub((*resp).elapsed_ms as libc::c_ulonglong) as libc::c_uint;
}

unsafe extern "C" fn printargv(mut argv: *const *mut libc::c_char) {
  let mut fmt: *const libc::c_char = (b" %s\x00" as *const u8 as *const libc::c_char).offset(1);
  loop {
    printf(fmt, *argv);
    fmt = b" %s\x00" as *const u8 as *const libc::c_char;
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
  }
}

unsafe extern "C" fn ptok(pagesize: libc::c_uint, pages: libc::c_ulong) -> libc::c_ulong {
  let mut tmp: libc::c_ulong = 0;
  if pages > (9223372036854775807i64 / pagesize as libc::c_long) as libc::c_ulong {
    tmp = pages.wrapping_div(1024i32 as libc::c_ulong);
    return tmp.wrapping_mul(pagesize as libc::c_ulong);
  }
  tmp = pages.wrapping_mul(pagesize as libc::c_ulong);
  return tmp.wrapping_div(1024i32 as libc::c_ulong);
}

unsafe extern "C" fn summarize(
  mut fmt: *const libc::c_char,
  mut command: *mut *mut libc::c_char,
  mut resp: *mut resource_t,
) {
  let mut vv_ms: libc::c_uint = 0;
  let mut cpu_ticks: libc::c_uint = 0;
  let mut pagesize: libc::c_uint = getpagesize() as libc::c_uint;

  if (((*resp).waitstatus & 0x7fi32) + 1i32) as libc::c_schar as libc::c_int >> 1i32 > 0 {
    printf(
      b"Command terminated by signal %u\n\x00" as *const u8 as *const libc::c_char,
      (*resp).waitstatus & 0x7fi32,
    );
  } else if (*resp).waitstatus & 0x7fi32 == 0 && ((*resp).waitstatus & 0xff00i32) >> 8i32 != 0 {
    printf(
      b"Command exited with non-zero status %u\n\x00" as *const u8 as *const libc::c_char,
      ((*resp).waitstatus & 0xff00i32) >> 8i32,
    );
  }
  vv_ms = (((*resp).ru.ru_utime.tv_sec + (*resp).ru.ru_stime.tv_sec) * 1000i32 as libc::c_long
    + ((*resp).ru.ru_utime.tv_usec + (*resp).ru.ru_stime.tv_usec) / 1000i32 as libc::c_long)
    as libc::c_uint;
  cpu_ticks = vv_ms.wrapping_div((1000i32 / 100i32) as libc::c_uint);
  if cpu_ticks == 0 {
    cpu_ticks = 1i32 as libc::c_uint
  }
  while *fmt != 0 {
    let mut n: libc::c_int =
      strcspn(fmt, b"%\\\x00" as *const u8 as *const libc::c_char) as libc::c_int;
    if n != 0 {
      printf(b"%.*s\x00" as *const u8 as *const libc::c_char, n, fmt);
      fmt = fmt.offset(n as isize)
    } else {
      match *fmt as libc::c_int {
        37 => {
          fmt = fmt.offset(1);
          match *fmt as libc::c_int {
            67 => {
              printargv(command);
            }
            68 => {
              printf(
                b"%lu\x00" as *const u8 as *const libc::c_char,
                ptok(pagesize, (*resp).ru.ru_idrss as libc::c_ulong)
                  .wrapping_add(ptok(pagesize, (*resp).ru.ru_isrss as libc::c_ulong))
                  .wrapping_div(cpu_ticks as libc::c_ulong),
              );
            }
            69 => {
              let mut seconds: libc::c_uint =
                (*resp).elapsed_ms.wrapping_div(1000i32 as libc::c_uint);
              if seconds >= 3600i32 as libc::c_uint {
                printf(
                  b"%uh %um %02us\x00" as *const u8 as *const libc::c_char,
                  seconds.wrapping_div(3600i32 as libc::c_uint),
                  seconds
                    .wrapping_rem(3600i32 as libc::c_uint)
                    .wrapping_div(60i32 as libc::c_uint),
                  seconds.wrapping_rem(60i32 as libc::c_uint),
                );
              } else {
                printf(
                  b"%um %u.%02us\x00" as *const u8 as *const libc::c_char,
                  seconds.wrapping_div(60i32 as libc::c_uint),
                  seconds.wrapping_rem(60i32 as libc::c_uint),
                  (*resp)
                    .elapsed_ms
                    .wrapping_div(10i32 as libc::c_uint)
                    .wrapping_rem(100i32 as libc::c_uint),
                );
              }
            }
            70 => {
              printf(
                b"%lu\x00" as *const u8 as *const libc::c_char,
                (*resp).ru.ru_majflt,
              );
            }
            73 => {
              printf(
                b"%lu\x00" as *const u8 as *const libc::c_char,
                (*resp).ru.ru_inblock,
              );
            }
            75 => {
              printf(
                b"%lu\x00" as *const u8 as *const libc::c_char,
                ptok(pagesize, (*resp).ru.ru_idrss as libc::c_ulong)
                  .wrapping_add(ptok(pagesize, (*resp).ru.ru_isrss as libc::c_ulong))
                  .wrapping_add(ptok(pagesize, (*resp).ru.ru_ixrss as libc::c_ulong))
                  .wrapping_div(cpu_ticks as libc::c_ulong),
              );
            }
            77 => {
              printf(
                b"%lu\x00" as *const u8 as *const libc::c_char,
                ptok(pagesize, (*resp).ru.ru_maxrss as libc::c_ulong),
              );
            }
            79 => {
              printf(
                b"%lu\x00" as *const u8 as *const libc::c_char,
                (*resp).ru.ru_oublock,
              );
            }
            80 => {
              if (*resp).elapsed_ms > 0 as libc::c_uint {
                printf(
                  b"%u%%\x00" as *const u8 as *const libc::c_char,
                  vv_ms
                    .wrapping_mul(100i32 as libc::c_uint)
                    .wrapping_div((*resp).elapsed_ms),
                );
              } else {
                printf(b"?%%\x00" as *const u8 as *const libc::c_char);
              }
            }
            82 => {
              printf(
                b"%lu\x00" as *const u8 as *const libc::c_char,
                (*resp).ru.ru_minflt,
              );
            }
            83 => {
              printf(
                b"%u.%02u\x00" as *const u8 as *const libc::c_char,
                (*resp).ru.ru_stime.tv_sec as libc::c_uint,
                ((*resp).ru.ru_stime.tv_usec / 10000i32 as libc::c_long) as libc::c_uint,
              );
            }
            84 => {
              if (*resp).ru.ru_stime.tv_sec >= 3600i32 as libc::c_long {
                printf(
                  b"%uh %um %02us\x00" as *const u8 as *const libc::c_char,
                  ((*resp).ru.ru_stime.tv_sec / 3600i32 as libc::c_long) as libc::c_uint,
                  (((*resp).ru.ru_stime.tv_sec % 3600i32 as libc::c_long) as libc::c_uint)
                    .wrapping_div(60i32 as libc::c_uint),
                  ((*resp).ru.ru_stime.tv_sec % 60i32 as libc::c_long) as libc::c_uint,
                );
              } else {
                printf(
                  b"%um %u.%02us\x00" as *const u8 as *const libc::c_char,
                  ((*resp).ru.ru_stime.tv_sec / 60i32 as libc::c_long) as libc::c_uint,
                  ((*resp).ru.ru_stime.tv_sec % 60i32 as libc::c_long) as libc::c_uint,
                  ((*resp).ru.ru_stime.tv_usec / 10000i32 as libc::c_long) as libc::c_uint,
                );
              }
            }
            85 => {
              printf(
                b"%u.%02u\x00" as *const u8 as *const libc::c_char,
                (*resp).ru.ru_utime.tv_sec as libc::c_uint,
                ((*resp).ru.ru_utime.tv_usec / 10000i32 as libc::c_long) as libc::c_uint,
              );
            }
            117 => {
              if (*resp).ru.ru_utime.tv_sec >= 3600i32 as libc::c_long {
                printf(
                  b"%uh %um %02us\x00" as *const u8 as *const libc::c_char,
                  ((*resp).ru.ru_utime.tv_sec / 3600i32 as libc::c_long) as libc::c_uint,
                  (((*resp).ru.ru_utime.tv_sec % 3600i32 as libc::c_long) as libc::c_uint)
                    .wrapping_div(60i32 as libc::c_uint),
                  ((*resp).ru.ru_utime.tv_sec % 60i32 as libc::c_long) as libc::c_uint,
                );
              } else {
                printf(
                  b"%um %u.%02us\x00" as *const u8 as *const libc::c_char,
                  ((*resp).ru.ru_utime.tv_sec / 60i32 as libc::c_long) as libc::c_uint,
                  ((*resp).ru.ru_utime.tv_sec % 60i32 as libc::c_long) as libc::c_uint,
                  ((*resp).ru.ru_utime.tv_usec / 10000i32 as libc::c_long) as libc::c_uint,
                );
              }
            }
            87 => {
              printf(
                b"%lu\x00" as *const u8 as *const libc::c_char,
                (*resp).ru.ru_nswap,
              );
            }
            88 => {
              printf(
                b"%lu\x00" as *const u8 as *const libc::c_char,
                ptok(pagesize, (*resp).ru.ru_ixrss as libc::c_ulong)
                  .wrapping_div(cpu_ticks as libc::c_ulong),
              );
            }
            90 => {
              printf(b"%u\x00" as *const u8 as *const libc::c_char, pagesize);
            }
            99 => {
              printf(
                b"%lu\x00" as *const u8 as *const libc::c_char,
                (*resp).ru.ru_nivcsw,
              );
            }
            101 => {
              printf(
                b"%u.%02u\x00" as *const u8 as *const libc::c_char,
                (*resp).elapsed_ms.wrapping_div(1000i32 as libc::c_uint),
                (*resp)
                  .elapsed_ms
                  .wrapping_div(10i32 as libc::c_uint)
                  .wrapping_rem(100i32 as libc::c_uint),
              );
            }
            107 => {
              printf(
                b"%lu\x00" as *const u8 as *const libc::c_char,
                (*resp).ru.ru_nsignals,
              );
            }
            112 => {
              printf(
                b"%lu\x00" as *const u8 as *const libc::c_char,
                ptok(pagesize, (*resp).ru.ru_isrss as libc::c_ulong)
                  .wrapping_div(cpu_ticks as libc::c_ulong),
              );
            }
            114 => {
              printf(
                b"%lu\x00" as *const u8 as *const libc::c_char,
                (*resp).ru.ru_msgrcv,
              );
            }
            115 => {
              printf(
                b"%lu\x00" as *const u8 as *const libc::c_char,
                (*resp).ru.ru_msgsnd,
              );
            }
            116 => {
              printf(
                b"%lu\x00" as *const u8 as *const libc::c_char,
                ptok(pagesize, (*resp).ru.ru_idrss as libc::c_ulong)
                  .wrapping_div(cpu_ticks as libc::c_ulong),
              );
            }
            119 => {
              printf(
                b"%lu\x00" as *const u8 as *const libc::c_char,
                (*resp).ru.ru_nvcsw,
              );
            }
            120 => {
              printf(
                b"%u\x00" as *const u8 as *const libc::c_char,
                ((*resp).waitstatus & 0xff00i32) >> 8i32,
              );
            }
            _ => {}
          }
        }
        _ => {}
      }
      fmt = fmt.offset(1)
    }
  }
  crate::libbb::xfuncs_printf::bb_putchar('\n' as i32);
}

unsafe extern "C" fn run_command(mut cmd: *const *mut libc::c_char, mut resp: *mut resource_t) {
  let mut pid: pid_t = 0;
  let mut interrupt_signal: Option<unsafe extern "C" fn(_: libc::c_int) -> ()> = None;
  let mut quit_signal: Option<unsafe extern "C" fn(_: libc::c_int) -> ()> = None;
  (*resp).elapsed_ms = crate::libbb::time::monotonic_ms() as libc::c_uint;
  pid = {
    let mut bb__xvfork_pid: pid_t = vfork();
    if bb__xvfork_pid < 0 {
      crate::libbb::perror_msg::bb_simple_perror_msg_and_die(
        b"vfork\x00" as *const u8 as *const libc::c_char,
      );
    }
    bb__xvfork_pid
  };
  if pid == 0 {
    crate::libbb::executable::BB_EXECVP_or_die(cmd as *mut *mut libc::c_char);
  }
  interrupt_signal = signal(
    2i32,
    ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(1i32 as libc::intptr_t),
  );
  quit_signal = signal(
    3i32,
    ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(1i32 as libc::intptr_t),
  );
  resuse_end(pid, resp);
  signal(2i32, interrupt_signal);
  signal(3i32, quit_signal);
}

pub unsafe fn time_main(mut _argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
  let mut res: resource_t = std::mem::zeroed();
  let ref mut fresh0 = getenv(b"TIME\x00" as *const u8 as *const libc::c_char);
  let mut output_format: *const libc::c_char = if !(*fresh0).is_null() {
    *fresh0
  } else {
    default_format.as_ptr()
  };
  let mut output_filename: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut output_fd: libc::c_int = 0;
  let mut opt: libc::c_int = 0;
  let mut ex: libc::c_int = 0;
  opt = crate::libbb::getopt32::getopt32(
    argv,
    b"^+vpao:f:\x00-1\x00" as *const u8 as *const libc::c_char,
    &mut output_filename as *mut *mut libc::c_char,
    &mut output_format as *mut *const libc::c_char,
  ) as libc::c_int;
  argv = argv.offset(optind as isize);
  if opt & OPT_v as libc::c_int != 0 {
    output_format = long_format.as_ptr()
  }
  if opt & OPT_p as libc::c_int != 0 {
    output_format = posix_format.as_ptr()
  }
  output_fd = 2i32;
  if opt & OPT_o as libc::c_int != 0 {
    output_fd = crate::libbb::xfuncs_printf::xopen(
      output_filename,
      if opt & OPT_a as libc::c_int != 0 {
        (0o100i32 | 0o1i32 | 0o2000000i32) | 0o2000i32
      } else {
        (0o100i32 | 0o1i32 | 0o2000000i32) | 0o1000i32
      },
    );
    if 0o2000000i32 == 0 {
      crate::libbb::xfuncs::close_on_exec_on(output_fd);
    }
  }
  run_command(argv, &mut res);
  crate::libbb::xfuncs_printf::xdup2(output_fd, 1i32);
  summarize(output_format, argv, &mut res);
  ex = (res.waitstatus & 0xff00i32) >> 8i32;
  if ((res.waitstatus & 0x7fi32) + 1i32) as libc::c_schar as libc::c_int >> 1i32 > 0 {
    ex = res.waitstatus & 0x7fi32
  }
  crate::libbb::fflush_stdout_and_exit::fflush_stdout_and_exit(ex);
}
