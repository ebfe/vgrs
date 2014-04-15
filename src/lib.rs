/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#![crate_id="github.com/kmcallister/vgrs"]
#![crate_type="lib"]
#![feature(macro_rules, asm, globs)]

use std::libc::c_uint;

#[cfg(target_arch = "x86_64", target_os = "linux")]
#[cfg(target_arch = "x86_64", target_os = "macos")]
#[path = "arch/x86_64-linux-macos.rs"]
mod arch;

mod enums;

#[doc(hidden)]
priv trait FromUint {
    fn from_uint(x: uint) -> Self;
}

impl FromUint for uint {
    fn from_uint(x: uint) -> uint { x }
}

impl FromUint for () {
    fn from_uint(_: uint) -> () { }
}

impl FromUint for *() {
    fn from_uint(x: uint) -> *() { x as *() }
}

impl FromUint for c_uint {
    fn from_uint(x: uint) -> c_uint { x as c_uint }
}

impl<T: FromUint> FromUint for Option<T> {
    fn from_uint(x: uint) -> Option<T> {
        match x {
            0 => None,
            _ => Some(FromUint::from_uint(x)),
        }
    }
}

macro_rules! wrap (
    ($nr:ident => fn $name:ident ( ) -> $t_ret:ty) => (
        #[inline(always)]
        pub unsafe fn $name() -> $t_ret {
            use super::{FromUint, arch, enums};
            FromUint::from_uint(arch::request(0, enums::$nr as uint, 0, 0, 0, 0, 0))
        }
    );

    ($nr:ident => fn $name:ident ( $a1:ident : $t1:ty ) -> $t_ret:ty) => (
        #[inline(always)]
        pub unsafe fn $name($a1: $t1) -> $t_ret {
            use super::{FromUint, arch, enums};
            FromUint::from_uint(arch::request(0, enums::$nr as uint, $a1 as uint, 0, 0, 0, 0))
        }
    );

    ($nr:ident => fn $name:ident ( $a1:ident : $t1:ty , $a2:ident : $t2:ty ) -> $t_ret:ty) => (
        #[inline(always)]
        pub unsafe fn $name($a1: $t1, $a2: $t2) -> $t_ret {
            use super::{FromUint, arch, enums};
            FromUint::from_uint(arch::request(0, enums::$nr as uint, $a1 as uint, $a2 as uint, 0, 0, 0))
        }
    );

    ($nr:ident => fn $name:ident ( $a1:ident : $t1:ty , $a2:ident : $t2:ty,
            $a3:ident : $t3:ty ) -> $t_ret:ty ) => (
        #[inline(always)]
        pub unsafe fn $name($a1: $t1, $a2: $t2, $a3: $t3) -> $t_ret {
            use super::{FromUint, arch, enums};
            FromUint::from_uint(arch::request(0, enums::$nr as uint,
                $a1 as uint, $a2 as uint, $a3 as uint, 0, 0))
        }
    );

    ($nr:ident => fn $name:ident ( $a1:ident : $t1:ty , $a2:ident : $t2:ty,
            $a3:ident : $t3:ty, $a4:ident : $t4:ty ) -> $t_ret:ty) => (
        #[inline(always)]
        pub unsafe fn $name($a1: $t1, $a2: $t2, $a3: $t3, $a4: $t4) -> $t_ret {
            use super::{FromUint, arch, enums};
            FromUint::from_uint(arch::request(0, enums::$nr as uint,
                $a1 as uint, $a2 as uint, $a3 as uint, $a4 as uint, 0))
        }
    );

    ($nr:ident => fn $name:ident ( $a1:ident : $t1:ty , $a2:ident : $t2:ty,
            $a3:ident : $t3:ty, $a4:ident : $t4:ty, $a5:ident $t5:ty ) -> $t_ret:ty) => (
        #[inline(always)]
        pub unsafe fn $name($a1: $t1, $a2: $t2, $a3: $t3, $a4: $t4, $a5: $t5) -> $t_ret {
            use super::{FromUint, arch, enums};
            FromUint::from_uint(arch::request(0, enums::$nr as uint,
                $a1 as uint, $a2 as uint, $a3 as uint, $a4 as uint, $a5 as uint))
        }
    );
)

macro_rules! wrap_str ( ($nr:ident => fn $name:ident ( $a1:ident : &str ) -> ()) => (
    #[inline(always)]
    pub unsafe fn $name($a1: &str) {
        $a1.with_c_str(|c_str| {
            use super::{arch, enums};
            arch::request(0, enums::$nr as uint, c_str as uint, 0, 0, 0, 0);
        });
    }
))

macro_rules! generic ( ($imp:ident => fn $name:ident <T>($a1:ident : *T) -> $t_ret:ty) => (
    #[inline(always)]
    pub unsafe fn $name<T>($a1: *T) -> $t_ret {
        use std::mem::size_of;
        $imp($a1 as *(), size_of::<T>())
    }
))

pub mod valgrind {
    wrap!(VG_USERREQ__RUNNING_ON_VALGRIND
        => fn running_on_valgrind() -> uint)

    wrap!(VG_USERREQ__COUNT_ERRORS
        => fn count_errors() -> uint)

    wrap!(VG_USERREQ__DISCARD_TRANSLATIONS
        => fn discard_translations(addr: *(), len: uint) -> ())

    wrap_str!(VG_USERREQ__GDB_MONITOR_COMMAND
        => fn monitor_command(cmd: &str) -> ())
}

pub mod memcheck {
    wrap!(VG_USERREQ__MALLOCLIKE_BLOCK
        => fn malloclike_block(addr: *(), size: uint, redzone: uint, is_zeroed: bool) -> ())

    wrap!(VG_USERREQ__RESIZEINPLACE_BLOCK
        => fn resizeinplace_block(addr: *(), old_size: uint, new_size: uint, redzone: uint) -> ())

    wrap!(VG_USERREQ__FREELIKE_BLOCK
        => fn freelike_block(addr: *(), redzone: uint) -> ())

    wrap!(VG_USERREQ__MAKE_MEM_NOACCESS
        => fn make_mem_noaccess(addr: *(), len: uint) -> ())

    generic!(make_mem_noaccess
        => fn make_noaccess<T>(obj: *T) -> ())

    wrap!(VG_USERREQ__MAKE_MEM_UNDEFINED
        => fn make_mem_undefined(addr: *(), len: uint) -> ())

    generic!(make_mem_undefined
        => fn make_undefined<T>(obj: *T) -> ())

    wrap!(VG_USERREQ__MAKE_MEM_DEFINED
        => fn make_mem_defined(addr: *(), len: uint) -> ())

    generic!(make_mem_defined
        => fn make_defined<T>(obj: *T) -> ())

    wrap!(VG_USERREQ__MAKE_MEM_DEFINED_IF_ADDRESSABLE
        => fn make_mem_defined_if_addressable(addr: *(), len: uint) -> ())

    generic!(make_mem_defined_if_addressable
        => fn make_defined_if_addressable<T>(obj: *T) -> ())

    wrap!(VG_USERREQ__CHECK_MEM_IS_ADDRESSABLE
        => fn check_mem_is_addressable(addr: *(), len: uint) -> Option<*()>)

    generic!(check_mem_is_addressable
        => fn check_is_addressable<T>(obj: *T) -> Option<*()>)

    wrap!(VG_USERREQ__CHECK_MEM_IS_DEFINED
        => fn check_mem_is_defined(addr: *(), len: uint) -> Option<*()>)

    generic!(check_mem_is_defined
        => fn check_is_defined<T>(obj: *T) -> Option<*()>)

    macro_rules! wrap_leak_check ( ($nr:ident($a1:expr, $a2:expr) => fn $name:ident () -> ()) => (
        #[inline(always)]
        pub unsafe fn $name() {
            use super::{arch, enums};
            arch::request(0, enums::$nr as uint, $a1, $a2, 0, 0, 0);
        }
    ))

    wrap_leak_check!(VG_USERREQ__DO_LEAK_CHECK(0, 0)
        => fn do_leak_check() -> ())

    wrap_leak_check!(VG_USERREQ__DO_LEAK_CHECK(0, 1)
        => fn do_added_leak_check() -> ())

    wrap_leak_check!(VG_USERREQ__DO_LEAK_CHECK(0, 2)
        => fn do_changed_leak_check() -> ())

    wrap_leak_check!(VG_USERREQ__DO_LEAK_CHECK(1, 0)
        => fn do_quick_leak_check() -> ())

    pub struct LeakCount {
        leaked: uint,
        dubious: uint,
        reachable: uint,
        suppressed: uint,
    }

    macro_rules! wrap_count ( ($nr:ident => fn $name:ident() -> LeakCount) => (
        #[inline(always)]
        pub unsafe fn $name() -> LeakCount {
            use super::{arch, enums};
            let mut counts = LeakCount {
                leaked: 0,
                dubious: 0,
                reachable: 0,
                suppressed: 0,
            };
            arch::request(0, enums::$nr as uint,
                (&mut counts.leaked as *mut uint) as uint,
                (&mut counts.dubious as *mut uint) as uint,
                (&mut counts.reachable as *mut uint) as uint,
                (&mut counts.suppressed as *mut uint) as uint,
                0);
            counts
        }
    ))

    wrap_count!(VG_USERREQ__COUNT_LEAKS
        => fn count_leaks() -> LeakCount)

    wrap_count!(VG_USERREQ__COUNT_LEAK_BLOCKS
        => fn count_leak_blocks() -> LeakCount)
}

pub mod callgrind {
    wrap!(VG_USERREQ__DUMP_STATS
        => fn dump_stats() -> ())

    wrap_str!(VG_USERREQ__DUMP_STATS_AT
        => fn dump_stats_at(pos: &str) -> ())

    wrap!(VG_USERREQ__ZERO_STATS
        => fn zero_stats() -> ())

    wrap!(VG_USERREQ__TOGGLE_COLLECT
        => fn toggle_collect() -> ())

    wrap!(VG_USERREQ__START_INSTRUMENTATION
        => fn start_instrumentation() -> ())

    wrap!(VG_USERREQ__STOP_INSTRUMENTATION
        => fn stop_instrumentation() -> ())
}

pub mod helgrind {
    wrap!(VG_USERREQ__HG_CLEAN_MEMORY
        => fn clean_memory(addr: *(), len: uint) -> ())

    generic!(clean_memory
        => fn clean<T>(obj: *T) -> ())
}

pub mod drd {
    use std::libc::c_uint;

    wrap!(VG_USERREQ__DRD_CLEAN_MEMORY
        => fn clean_memory(addr: *(), len: uint) -> ())

    generic!(clean_memory
        => fn clean<T>(obj: *T) -> ())

    wrap!(VG_USERREQ__DRD_GET_VALGRIND_THREAD_ID
        => fn get_valgrind_threadid() -> c_uint)

    wrap!(VG_USERREQ__DRD_GET_DRD_THREAD_ID
        => fn get_drd_threadid() -> c_uint)

    wrap!(VG_USERREQ__DRD_START_SUPPRESSION
        => fn annotate_benign_race_sized(addr: *(), len: uint) -> ())

    generic!(annotate_benign_race_sized
        => fn annotate_benign_race<T>(obj: *T) -> ())

    wrap!(VG_USERREQ__DRD_FINISH_SUPPRESSION
        => fn stop_ignoring_sized(addr: *(), len: uint) -> ())

    generic!(stop_ignoring_sized
        => fn stop_ignoring<T>(obj: *T) -> ())

    wrap!(VG_USERREQ__DRD_START_TRACE_ADDR
        => fn trace_sized(addr: *(), len: uint) -> ())

    generic!(trace_sized
        => fn trace<T>(obj: *T) -> ())

    wrap!(VG_USERREQ__DRD_STOP_TRACE_ADDR
        => fn stop_tracing_sized(addr: *(), len: uint) -> ())

    generic!(stop_tracing_sized
        => fn stop_tracing<T>(obj: *T) -> ())

    macro_rules! wrap_record( ($nr:ident($n:expr) => fn $name:ident() -> ()) => (
        #[inline(always)]
        pub unsafe fn $name() {
            use super::{arch, enums};
            arch::request(0, enums::$nr as uint, $n, 0, 0, 0, 0);
        }
    ))

    wrap_record!(VG_USERREQ__DRD_RECORD_LOADS(0)
        => fn ignore_reads_begin() -> ())

    wrap_record!(VG_USERREQ__DRD_RECORD_LOADS(1)
        => fn ignore_reads_end() -> ())

    wrap_record!(VG_USERREQ__DRD_RECORD_STORES(0)
        => fn ignore_writes_begin() -> ())

    wrap_record!(VG_USERREQ__DRD_RECORD_STORES(1)
        => fn ignore_writes_end() -> ())

    wrap_str!(VG_USERREQ__DRD_SET_THREAD_NAME
        => fn annotate_thread_name(name: &str) -> ())
}