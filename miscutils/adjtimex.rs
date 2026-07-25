use crate::compat::memset;
use crate::compat::strlen;
use libc;
use libc::printf;

use crate::librb::size_t;
use libc::timeval;
pub const OPT_quiet: C2RustUnnamed = 1;
pub type C2RustUnnamed = libc::c_uint;
#[inline(always)]
unsafe fn xatol(mut str: *const libc::c_char) -> libc::c_long {
  return crate::libbb::xatonum::xatoll(str) as libc::c_long;
}

/*
 * adjtimex.c - read, and possibly modify, the Linux kernel 'timex' variables.
 *
 * Originally written: October 1997
 * Last hack: March 2001
 * Copyright 1997, 2000, 2001 Larry Doolittle <LRDoolittle@lbl.gov>
 *
 * busyboxed 20 March 2001, Larry Doolittle <ldoolitt@recycle.lbl.gov>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config ADJTIMEX
//config:	bool "adjtimex (4.7 kb)"
//config:	default y
//config:	select PLATFORM_LINUX
//config:	help
//config:	Adjtimex reads and optionally sets adjustment parameters for
//config:	the Linux clock adjustment algorithm.
//applet:IF_ADJTIMEX(APPLET_NOFORK(adjtimex, adjtimex, BB_DIR_SBIN, SUID_DROP, adjtimex))
//kbuild:lib-$(CONFIG_ADJTIMEX) += adjtimex.o
//usage:#define adjtimex_trivial_usage
//usage:       "[-q] [-o OFF] [-f FREQ] [-p TCONST] [-t TICK]"
//usage:#define adjtimex_full_usage "\n\n"
//usage:       "Read or set kernel time variables. See adjtimex(2)\n"
//usage:     "\n	-q	Quiet"
//usage:     "\n	-o OFF	Time offset, microseconds"
//usage:     "\n	-f FREQ	Frequency adjust, integer kernel units (65536 is 1ppm)"
//usage:     "\n	-t TICK	Microseconds per tick, usually 10000"
//usage:     "\n		(positive -t or -f values make clock run faster)"
//usage:     "\n	-p TCONST"
static mut statlist_bit: [u16; 14] = [
  0x1i32 as u16,
  0x2i32 as u16,
  0x4i32 as u16,
  0x8i32 as u16,
  0x10i32 as u16,
  0x20i32 as u16,
  0x40i32 as u16,
  0x80i32 as u16,
  0x100i32 as u16,
  0x200i32 as u16,
  0x400i32 as u16,
  0x800i32 as u16,
  0x1000i32 as u16,
  0 as u16,
];
static mut statlist_name: [libc::c_char; 96] = [
  80, 76, 76, 0, 80, 80, 83, 70, 82, 69, 81, 0, 80, 80, 83, 84, 73, 77, 69, 0, 70, 70, 76, 0, 73,
  78, 83, 0, 68, 69, 76, 0, 85, 78, 83, 89, 78, 67, 0, 70, 82, 69, 81, 72, 79, 76, 68, 0, 80, 80,
  83, 83, 73, 71, 78, 65, 76, 0, 80, 80, 83, 74, 73, 84, 84, 69, 82, 0, 80, 80, 83, 87, 65, 78, 68,
  69, 82, 0, 80, 80, 83, 69, 82, 82, 79, 82, 0, 67, 76, 79, 67, 75, 69, 82, 82, 0,
];
static mut ret_code_descript: [libc::c_char; 129] = [
  99, 108, 111, 99, 107, 32, 115, 121, 110, 99, 104, 114, 111, 110, 105, 122, 101, 100, 0, 105,
  110, 115, 101, 114, 116, 32, 108, 101, 97, 112, 32, 115, 101, 99, 111, 110, 100, 0, 100, 101,
  108, 101, 116, 101, 32, 108, 101, 97, 112, 32, 115, 101, 99, 111, 110, 100, 0, 108, 101, 97, 112,
  32, 115, 101, 99, 111, 110, 100, 32, 105, 110, 32, 112, 114, 111, 103, 114, 101, 115, 115, 0,
  108, 101, 97, 112, 32, 115, 101, 99, 111, 110, 100, 32, 104, 97, 115, 32, 111, 99, 99, 117, 114,
  114, 101, 100, 0, 99, 108, 111, 99, 107, 32, 110, 111, 116, 32, 115, 121, 110, 99, 104, 114, 111,
  110, 105, 122, 101, 100, 0,
];
pub unsafe fn adjtimex_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut opt: libc::c_uint = 0;
  let mut opt_o: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut opt_f: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut opt_p: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut opt_t: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut txc: libc::timex = unsafe { std::mem::zeroed() };
  let mut ret: libc::c_int = 0;
  let mut descript: *const libc::c_char = std::ptr::null();
  opt = crate::libbb::getopt32::getopt32(
    argv,
    b"^qo:f:p:t:\x00=0\x00" as *const u8 as *const libc::c_char,
    &mut opt_o as *mut *mut libc::c_char,
    &mut opt_f as *mut *mut libc::c_char,
    &mut opt_p as *mut *mut libc::c_char,
    &mut opt_t as *mut *mut libc::c_char,
  );
  //if (opt & 0x1) // -q
  if opt & 0x2i32 as libc::c_uint != 0 {
    // -o
    txc.offset = xatol(opt_o);
    txc.modes |= 0x8001i32 as libc::c_uint
  }
  if opt & 0x4i32 as libc::c_uint != 0 {
    // -f
    txc.freq = xatol(opt_f);
    txc.modes |= 0x2i32 as libc::c_uint
  }
  if opt & 0x8i32 as libc::c_uint != 0 {
    // -p
    txc.constant = xatol(opt_p);
    txc.modes |= 0x20i32 as libc::c_uint
  }
  if opt & 0x10i32 as libc::c_uint != 0 {
    // -t
    txc.tick = xatol(opt_t);
    txc.modes |= 0x4000i32 as libc::c_uint
  }
  /* It's NOFORK applet because the code is very simple:
   * just some printf. No opens, no allocs.
   * If you need to make it more complex, feel free to downgrade to NOEXEC
   */
  ret = libc::adjtimex(&mut txc);
  if ret < 0 {
    crate::libbb::perror_nomsg_and_die::bb_perror_nomsg_and_die();
  }
  if opt & OPT_quiet as libc::c_int as libc::c_uint == 0 {
    let mut sep: *const libc::c_char = std::ptr::null();
    let mut name: *const libc::c_char = std::ptr::null();
    let mut i: libc::c_int = 0;
    printf(b"    mode:         %d\n-o  offset:       %ld us\n-f  freq.adjust:  %ld (65536 = 1ppm)\n    maxerror:     %ld\n    esterror:     %ld\n    status:       %d (\x00"
                   as *const u8 as *const libc::c_char, txc.modes, txc.offset,
               txc.freq, txc.maxerror, txc.esterror, txc.status);
    /* representative output of next code fragment:
     * "PLL | PPSTIME"
     */
    name = statlist_name.as_ptr();
    sep = b"\x00" as *const u8 as *const libc::c_char;
    i = 0;
    while statlist_bit[i as usize] != 0 {
      if txc.status & statlist_bit[i as usize] as libc::c_int != 0 {
        printf(b"%s%s\x00" as *const u8 as *const libc::c_char, sep, name);
        sep = b" | \x00" as *const u8 as *const libc::c_char
      }
      name = name.offset(strlen(name).wrapping_add(1i32 as libc::c_ulong) as isize);
      i += 1
    }
    descript = b"error\x00" as *const u8 as *const libc::c_char;
    if ret <= 5i32 {
      descript = crate::libbb::compare_string_array::nth_string(ret_code_descript.as_ptr(), ret)
    }
    printf(b")\n-p  timeconstant: %ld\n    precision:    %ld us\n    tolerance:    %ld\n-t  tick:         %ld us\n    time.tv_sec:  %ld\n    time.tv_usec: %ld\n    return value: %d (%s)\n\x00"
                   as *const u8 as *const libc::c_char, txc.constant,
               txc.precision, txc.tolerance, txc.tick, txc.time.tv_sec,
               txc.time.tv_usec, ret, descript);
  }
  return 0;
}
