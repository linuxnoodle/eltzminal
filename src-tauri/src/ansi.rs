#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(register_tool)]
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn div(__numer: libc::c_int, __denom: libc::c_int) -> div_t;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct div_t {
    pub quot: libc::c_int,
    pub rem: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct selem {
    pub digit: [libc::c_uchar; 8],
    pub digitcount: libc::c_uchar,
    pub value: libc::c_long,
    pub next: pelem,
}
pub type pelem = *mut selem;
pub type telem = selem;
pub type ColorScheme = libc::c_uint;
pub const SCHEME_PINK: ColorScheme = 2;
pub const SCHEME_BLACK: ColorScheme = 1;
pub const SCHEME_WHITE: ColorScheme = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Options {
    pub colorscheme: ColorScheme,
    pub filename: *mut libc::c_char,
    pub htop_fix: libc::c_int,
    pub iso: libc::c_int,
    pub line_break: libc::c_int,
    pub no_header: libc::c_int,
    pub stylesheet: libc::c_int,
    pub title: *mut libc::c_char,
    pub word_wrap: libc::c_int,
    pub no_xml: libc::c_int,
    pub lang: *mut libc::c_char,
    pub css: *mut libc::c_char,
    pub ignore_cr: libc::c_int,
    pub bodystyle: *mut libc::c_char,
}
pub type ColorMode = libc::c_uint;
pub const MODE_24BIT: ColorMode = 2;
pub const MODE_8BIT: ColorMode = 1;
pub const MODE_3BIT: ColorMode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct State {
    pub fc: libc::c_int,
    pub bc: libc::c_int,
    pub bold: libc::c_int,
    pub italic: libc::c_int,
    pub underline: libc::c_int,
    pub blink: libc::c_int,
    pub crossedout: libc::c_int,
    pub fc_colormode: ColorMode,
    pub bc_colormode: ColorMode,
    pub highlighted: libc::c_int,
}
#[no_mangle]
pub static mut ansi_vt220_character_set: [[libc::c_char; 16]; 256] = unsafe {
    [
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x2400;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x2401;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x2402;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x2403;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x2404;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x2405;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x2406;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x2407;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x2408;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x2409;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x240a;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x240b;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x240c;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x240d;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x240e;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x240f;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x2410;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x2411;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x2412;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x2413;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x2414;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x2415;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x2416;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x2417;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x2418;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x2419;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x241a;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x241b;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x241c;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x241d;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x241e;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x241f;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b" \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"\"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"$\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"'\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b")\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"*\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b",\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"-\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b".\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"/\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b":\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b";\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&lt;\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&gt;\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"@\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"A\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"G\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"H\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"I\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"J\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"K\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"L\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"M\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"N\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"O\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"Q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"R\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"S\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"T\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"U\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"W\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"X\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"Y\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"Z\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"[\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"\\\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"^\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"_\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"`\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"a\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"b\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"c\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"d\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"e\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"f\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"h\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"i\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"j\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"k\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"l\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"m\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"o\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"p\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"v\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"w\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"x\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"y\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"z\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"{\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"|\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"}\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x2421;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x25c6;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x2592;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x2409;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x240c;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x240d;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x240a;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00b0;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00b1;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x2400;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x240b;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x2518;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x2510;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x250c;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x2514;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x253c;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x23ba;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x23bb;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x2500;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x23bc;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x23bd;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x251c;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x2524;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x2534;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x252c;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x2502;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x2264;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x2265;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&pi;    \0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x2260;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&pound;\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x0095;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x2421;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x2588;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00a1;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00a2;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00a3;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b" \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&yen;\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b" \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00a7;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00a4;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00a9;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00ba;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00qb;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b" \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b" \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b" \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b" \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x23bc;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x23bd;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00b2;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00b3;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00b4;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00b5;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00b6;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00b7;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00b8;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00b9;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00ba;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00bb;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00bc;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00bd;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00be;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00bf;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00c0;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00c1;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00c2;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00c3;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00c4;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00c5;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00c6;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00c7;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00c8;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00c9;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00ca;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00cb;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00cc;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00cd;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00ce;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00cf;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b" \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00d1;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00d2;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00d3;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00d4;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00d5;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00d6;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x0152;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00d8;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00d9;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00da;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00db;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00dc;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x0178;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b" \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00df;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00e0;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00e1;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00e2;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00e3;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00e4;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00e5;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00e6;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00e7;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00e8;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00e9;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00ea;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00eb;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00ec;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00ed;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00ee;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00ef;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b" \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00f1;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00f2;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00f3;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00f4;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00f5;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00f6;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x0153;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00f8;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00f9;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00fa;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00fb;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00fc;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x00ff;\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b" \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 16],
            &[libc::c_char; 16],
        >(b"&#x2588;\0\0\0\0\0\0\0\0"),
    ]
};
#[no_mangle]
pub unsafe extern "C" fn getNextChar(
    mut s: *mut libc::c_char,
    mut idx: *mut libc::c_uint,
    mut len: libc::c_uint,
) -> libc::c_int {
    if *idx < len {
        let fresh0 = *idx;
        *idx = (*idx).wrapping_add(1);
        return *s.offset(fresh0 as isize) as libc::c_int;
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn append_string(
    mut output: *mut *mut libc::c_char,
    mut output_len: *mut size_t,
    mut str: *const libc::c_char,
) {
    let mut new_len: size_t = (*output_len)
        .wrapping_add(strlen(str))
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    *output = realloc(*output as *mut libc::c_void, new_len) as *mut libc::c_char;
    if (*output).is_null() {
        fprintf(
            stderr,
            b"Memory allocation failed!\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    strcat(*output, str);
    *output_len = new_len;
}
#[no_mangle]
pub unsafe extern "C" fn parseInsert(mut s: *mut libc::c_char) -> pelem {
    let mut firstelem: pelem = 0 as pelem;
    let mut momelem: pelem = 0 as pelem;
    let mut digit: [libc::c_uchar; 8] = [0; 8];
    let mut digitcount: libc::c_uchar = 0 as libc::c_int as libc::c_uchar;
    let mut value: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut pos: libc::c_int = 0 as libc::c_int;
    pos = 0 as libc::c_int;
    while pos < 1024 as libc::c_int {
        if !(*s.offset(pos as isize) as libc::c_int == '[' as i32) {
            if *s.offset(pos as isize) as libc::c_int == ';' as i32
                || *s.offset(pos as isize) as libc::c_int == ':' as i32
                || *s.offset(pos as isize) as libc::c_int == 0 as libc::c_int
            {
                if digitcount as libc::c_int == 0 as libc::c_int {
                    digit[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
                    digitcount = 1 as libc::c_int as libc::c_uchar;
                }
                let mut newelem: pelem = malloc(
                    ::std::mem::size_of::<telem>() as libc::c_ulong,
                ) as pelem;
                if newelem.is_null() {
                    perror(
                        b"Failed to allocate memory for parseInsert()\0" as *const u8
                            as *const libc::c_char,
                    );
                    exit(1 as libc::c_int);
                }
                memcpy(
                    ((*newelem).digit).as_mut_ptr() as *mut libc::c_void,
                    digit.as_mut_ptr() as *const libc::c_void,
                    ::std::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong,
                );
                (*newelem).digitcount = digitcount;
                (*newelem).value = value;
                let ref mut fresh1 = (*newelem).next;
                *fresh1 = 0 as pelem;
                if momelem.is_null() {
                    firstelem = newelem;
                } else {
                    let ref mut fresh2 = (*momelem).next;
                    *fresh2 = newelem;
                }
                momelem = newelem;
                digitcount = 0 as libc::c_int as libc::c_uchar;
                memset(
                    digit.as_mut_ptr() as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong,
                );
                value = 0 as libc::c_int as libc::c_long;
                if *s.offset(pos as isize) as libc::c_int == 0 as libc::c_int {
                    break;
                }
            } else if (digitcount as libc::c_ulong)
                < ::std::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong
            {
                digit[digitcount
                    as usize] = (*s.offset(pos as isize) as libc::c_int - '0' as i32)
                    as libc::c_uchar;
                value = value * 10 as libc::c_int as libc::c_long
                    + digit[digitcount as usize] as libc::c_long;
                digitcount = digitcount.wrapping_add(1);
            }
        }
        pos += 1;
    }
    return firstelem;
}
#[no_mangle]
pub unsafe extern "C" fn deleteParse(mut elem: pelem) {
    while !elem.is_null() {
        let mut temp: pelem = (*elem).next;
        free(elem as *mut libc::c_void);
        elem = temp;
    }
}
#[no_mangle]
pub unsafe extern "C" fn printHtml(
    mut output: *mut *mut libc::c_char,
    mut output_len: *mut size_t,
    mut text: *mut libc::c_char,
) {
    loop {
        match *text as libc::c_int {
            0 => return,
            34 => {
                append_string(
                    output,
                    output_len,
                    b"&quot;\0" as *const u8 as *const libc::c_char,
                );
            }
            38 => {
                append_string(
                    output,
                    output_len,
                    b"&amp;\0" as *const u8 as *const libc::c_char,
                );
            }
            60 => {
                append_string(
                    output,
                    output_len,
                    b"&lt;\0" as *const u8 as *const libc::c_char,
                );
            }
            62 => {
                append_string(
                    output,
                    output_len,
                    b"&gt;\0" as *const u8 as *const libc::c_char,
                );
            }
            _ => {
                append_string(output, output_len, text);
            }
        }
        text = text.offset(1);
    };
}
#[no_mangle]
pub unsafe extern "C" fn divide(
    mut dividend: libc::c_int,
    mut divisor: libc::c_int,
) -> libc::c_int {
    let mut result: div_t = div_t { quot: 0, rem: 0 };
    result = div(dividend, divisor);
    return result.quot;
}
#[no_mangle]
pub unsafe extern "C" fn make_rgb(
    mut color_id: libc::c_int,
    mut str_rgb: *mut libc::c_char,
) {
    if color_id < 16 as libc::c_int || color_id > 255 as libc::c_int {
        return;
    }
    if color_id >= 232 as libc::c_int {
        let mut index: libc::c_int = color_id - 232 as libc::c_int;
        let mut grey: libc::c_int = index * 256 as libc::c_int / 24 as libc::c_int;
        sprintf(
            str_rgb,
            b"%02x%02x%02x\0" as *const u8 as *const libc::c_char,
            grey,
            grey,
            grey,
        );
        return;
    }
    let mut index_R: libc::c_int = divide(
        color_id - 16 as libc::c_int,
        36 as libc::c_int,
    );
    let mut rgb_R: libc::c_int = 0;
    if index_R > 0 as libc::c_int {
        rgb_R = 55 as libc::c_int + index_R * 40 as libc::c_int;
    } else {
        rgb_R = 0 as libc::c_int;
    }
    let mut index_G: libc::c_int = divide(
        (color_id - 16 as libc::c_int) % 36 as libc::c_int,
        6 as libc::c_int,
    );
    let mut rgb_G: libc::c_int = 0;
    if index_G > 0 as libc::c_int {
        rgb_G = 55 as libc::c_int + index_G * 40 as libc::c_int;
    } else {
        rgb_G = 0 as libc::c_int;
    }
    let mut index_B: libc::c_int = (color_id - 16 as libc::c_int) % 6 as libc::c_int;
    let mut rgb_B: libc::c_int = 0;
    if index_B > 0 as libc::c_int {
        rgb_B = 55 as libc::c_int + index_B * 40 as libc::c_int;
    } else {
        rgb_B = 0 as libc::c_int;
    }
    sprintf(
        str_rgb,
        b"%02x%02x%02x\0" as *const u8 as *const libc::c_char,
        rgb_R,
        rgb_G,
        rgb_B,
    );
}
#[no_mangle]
pub unsafe extern "C" fn swapColors(state: *mut State) {
    if (*state).bc_colormode as libc::c_uint == MODE_3BIT as libc::c_int as libc::c_uint
        && (*state).bc == -(1 as libc::c_int)
    {
        (*state).bc = 8 as libc::c_int;
    }
    if (*state).fc_colormode as libc::c_uint == MODE_3BIT as libc::c_int as libc::c_uint
        && (*state).fc == -(1 as libc::c_int)
    {
        (*state).fc = 9 as libc::c_int;
    }
    let mut temp: libc::c_int = (*state).bc;
    (*state).bc = (*state).fc;
    (*state).fc = temp;
    let mut temp_colormode: ColorMode = (*state).bc_colormode;
    (*state).bc_colormode = (*state).fc_colormode;
    (*state).fc_colormode = temp_colormode;
}
#[no_mangle]
pub static mut default_state: State = {
    let mut init = State {
        fc: -(1 as libc::c_int),
        bc: -(1 as libc::c_int),
        bold: 0 as libc::c_int,
        italic: 0 as libc::c_int,
        underline: 0 as libc::c_int,
        blink: 0 as libc::c_int,
        crossedout: 0 as libc::c_int,
        fc_colormode: MODE_3BIT,
        bc_colormode: MODE_3BIT,
        highlighted: 0 as libc::c_int,
    };
    init
};
#[no_mangle]
pub unsafe extern "C" fn statesDiffer(
    old: *const State,
    new: *const State,
) -> libc::c_int {
    return ((*old).fc != (*new).fc || (*old).bc != (*new).bc
        || (*old).bold != (*new).bold || (*old).italic != (*new).italic
        || (*old).underline != (*new).underline || (*old).blink != (*new).blink
        || (*old).crossedout != (*new).crossedout
        || (*old).fc_colormode as libc::c_uint != (*new).fc_colormode as libc::c_uint
        || (*old).bc_colormode as libc::c_uint != (*new).bc_colormode as libc::c_uint
        || (*old).highlighted != (*new).highlighted) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn printHeader(
    mut opts: *const Options,
    mut output: *mut *mut libc::c_char,
    mut output_len: *mut size_t,
) {
    let mut encoding: [libc::c_char; 16] = *::std::mem::transmute::<
        &[u8; 16],
        &mut [libc::c_char; 16],
    >(b"UTF-8\0\0\0\0\0\0\0\0\0\0\0");
    if (*opts).iso > 0 as libc::c_int {
        snprintf(
            encoding.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
            b"ISO-8859-%i\0" as *const u8 as *const libc::c_char,
            (*opts).iso,
        );
    }
    if (*opts).no_xml != 0 {
        append_string(
            output,
            output_len,
            b"<!DOCTYPE html>\n\0" as *const u8 as *const libc::c_char,
        );
    } else {
        let mut temp: *mut libc::c_char = malloc(200 as libc::c_int as libc::c_ulong)
            as *mut libc::c_char;
        sprintf(
            temp,
            b"<?xml version=\"1.0\" encoding=\"%s\" ?>\n\0" as *const u8
                as *const libc::c_char,
            encoding.as_mut_ptr(),
        );
        sprintf(
            temp,
            b"<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Strict//EN\" \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-strict.dtd\">\n\0"
                as *const u8 as *const libc::c_char,
        );
        append_string(output, output_len, temp);
        free(temp as *mut libc::c_void);
    }
    if (*opts).no_xml != 0 {
        if !((*opts).lang).is_null() {
            append_string(
                output,
                output_len,
                b"<html lang=\"\0" as *const u8 as *const libc::c_char,
            );
            printHtml(output, output_len, (*opts).lang);
            append_string(
                output,
                output_len,
                b"\">\n\0" as *const u8 as *const libc::c_char,
            );
        } else {
            append_string(
                output,
                output_len,
                b"<html>\n\0" as *const u8 as *const libc::c_char,
            );
        }
        let mut temp_0: *mut libc::c_char = malloc(100 as libc::c_int as libc::c_ulong)
            as *mut libc::c_char;
        sprintf(
            temp_0,
            b"<head>\n<meta http-equiv=\"Content-Type\" content=\"text/html; charset=%s\">\n\0"
                as *const u8 as *const libc::c_char,
            encoding.as_mut_ptr(),
        );
        append_string(output, output_len, temp_0);
        free(temp_0 as *mut libc::c_void);
    } else {
        if !((*opts).lang).is_null() {
            append_string(
                output,
                output_len,
                b"<html xmlns=\"http://www.w3.org/1999/xhtml\" lang=\"\0" as *const u8
                    as *const libc::c_char,
            );
            printHtml(output, output_len, (*opts).lang);
            append_string(
                output,
                output_len,
                b"\">\n\0" as *const u8 as *const libc::c_char,
            );
        } else {
            append_string(
                output,
                output_len,
                b"<html xmlns=\"http://www.w3.org/1999/xhtml\">\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        let mut temp_1: *mut libc::c_char = malloc(100 as libc::c_int as libc::c_ulong)
            as *mut libc::c_char;
        sprintf(
            temp_1,
            b"<head>\n<meta http-equiv=\"Content-Type\" content=\"application/xml+xhtml; charset=%s\"/>\n\0"
                as *const u8 as *const libc::c_char,
            encoding.as_mut_ptr(),
        );
        append_string(output, output_len, temp_1);
        free(temp_1 as *mut libc::c_void);
    }
    append_string(output, output_len, b"<title>\0" as *const u8 as *const libc::c_char);
    printHtml(
        output,
        output_len,
        (if !((*opts).title).is_null() {
            (*opts).title
        } else if !((*opts).filename).is_null() {
            (*opts).filename
        } else {
            b"stdin\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char,
    );
    append_string(
        output,
        output_len,
        b"</title>\n\0" as *const u8 as *const libc::c_char,
    );
    if !((*opts).css).is_null() {
        append_string(
            output,
            output_len,
            b"<link rel=\"stylesheet\" href=\"\0" as *const u8 as *const libc::c_char,
        );
        printHtml(output, output_len, (*opts).css);
        append_string(
            output,
            output_len,
            b"\">\n\0" as *const u8 as *const libc::c_char,
        );
    }
    let mut style_tag: libc::c_int = 0 as libc::c_int;
    if (*opts).stylesheet != 0 {
        append_string(
            output,
            output_len,
            b"<style type=\"text/css\">\n\0" as *const u8 as *const libc::c_char,
        );
        style_tag = 1 as libc::c_int;
        match (*opts).colorscheme as libc::c_uint {
            1 => {
                append_string(
                    output,
                    output_len,
                    b"body         {color: white; background-color: black;}\n\0"
                        as *const u8 as *const libc::c_char,
                );
                append_string(
                    output,
                    output_len,
                    b".reset       {color: white;}\n\0" as *const u8
                        as *const libc::c_char,
                );
                append_string(
                    output,
                    output_len,
                    b".bg-reset    {background-color: black;}\n\0" as *const u8
                        as *const libc::c_char,
                );
                append_string(
                    output,
                    output_len,
                    b".inverted    {color: black;}\n\0" as *const u8
                        as *const libc::c_char,
                );
                append_string(
                    output,
                    output_len,
                    b".bg-inverted {background-color: white;}\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            2 => {
                append_string(
                    output,
                    output_len,
                    b"body         {background-color: pink;}\n\0" as *const u8
                        as *const libc::c_char,
                );
                append_string(
                    output,
                    output_len,
                    b".reset       {color: black;}\n\0" as *const u8
                        as *const libc::c_char,
                );
                append_string(
                    output,
                    output_len,
                    b".bg-reset    {background-color: pink;}\n\0" as *const u8
                        as *const libc::c_char,
                );
                append_string(
                    output,
                    output_len,
                    b".inverted    {color: pink;}\n\0" as *const u8
                        as *const libc::c_char,
                );
                append_string(
                    output,
                    output_len,
                    b".bg-inverted {background-color: black;}\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            _ => {
                append_string(
                    output,
                    output_len,
                    b".reset       {color: black;}\n\0" as *const u8
                        as *const libc::c_char,
                );
                append_string(
                    output,
                    output_len,
                    b".bg-reset    {background-color: white;}\n\0" as *const u8
                        as *const libc::c_char,
                );
                append_string(
                    output,
                    output_len,
                    b".inverted    {color: white;}\n\0" as *const u8
                        as *const libc::c_char,
                );
                append_string(
                    output,
                    output_len,
                    b".bg-inverted {background-color: black;}\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
        }
        if (*opts).colorscheme as libc::c_uint
            != SCHEME_BLACK as libc::c_int as libc::c_uint
        {
            append_string(
                output,
                output_len,
                b".dimgray     {color: dimgray;}\n\0" as *const u8 as *const libc::c_char,
            );
            append_string(
                output,
                output_len,
                b".red         {color: red;}\n\0" as *const u8 as *const libc::c_char,
            );
            append_string(
                output,
                output_len,
                b".green       {color: green;}\n\0" as *const u8 as *const libc::c_char,
            );
            append_string(
                output,
                output_len,
                b".yellow      {color: olive;}\n\0" as *const u8 as *const libc::c_char,
            );
            append_string(
                output,
                output_len,
                b".blue        {color: blue;}\n\0" as *const u8 as *const libc::c_char,
            );
            append_string(
                output,
                output_len,
                b".purple      {color: purple;}\n\0" as *const u8 as *const libc::c_char,
            );
            append_string(
                output,
                output_len,
                b".cyan        {color: teal;}\n\0" as *const u8 as *const libc::c_char,
            );
            append_string(
                output,
                output_len,
                b".white       {color: gray;}\n\0" as *const u8 as *const libc::c_char,
            );
            append_string(
                output,
                output_len,
                b".bg-black    {background-color: black;}\n\0" as *const u8
                    as *const libc::c_char,
            );
            append_string(
                output,
                output_len,
                b".bg-red      {background-color: red;}\n\0" as *const u8
                    as *const libc::c_char,
            );
            append_string(
                output,
                output_len,
                b".bg-green    {background-color: green;}\n\0" as *const u8
                    as *const libc::c_char,
            );
            append_string(
                output,
                output_len,
                b".bg-yellow   {background-color: olive;}\n\0" as *const u8
                    as *const libc::c_char,
            );
            append_string(
                output,
                output_len,
                b".bg-blue     {background-color: blue;}\n\0" as *const u8
                    as *const libc::c_char,
            );
            append_string(
                output,
                output_len,
                b".bg-purple   {background-color: purple;}\n\0" as *const u8
                    as *const libc::c_char,
            );
            append_string(
                output,
                output_len,
                b".bg-cyan     {background-color: teal;}\n\0" as *const u8
                    as *const libc::c_char,
            );
            append_string(
                output,
                output_len,
                b".bg-white    {background-color: gray;}\n\0" as *const u8
                    as *const libc::c_char,
            );
        } else {
            append_string(
                output,
                output_len,
                b".dimgray     {color: dimgray;}\n\0" as *const u8 as *const libc::c_char,
            );
            append_string(
                output,
                output_len,
                b".red         {color: red;}\n\0" as *const u8 as *const libc::c_char,
            );
            append_string(
                output,
                output_len,
                b".green       {color: lime;}\n\0" as *const u8 as *const libc::c_char,
            );
            append_string(
                output,
                output_len,
                b".yellow      {color: yellow;}\n\0" as *const u8 as *const libc::c_char,
            );
            append_string(
                output,
                output_len,
                b".blue        {color: #3333FF;}\n\0" as *const u8 as *const libc::c_char,
            );
            append_string(
                output,
                output_len,
                b".purple      {color: fuchsia;}\n\0" as *const u8 as *const libc::c_char,
            );
            append_string(
                output,
                output_len,
                b".cyan        {color: aqua;}\n\0" as *const u8 as *const libc::c_char,
            );
            append_string(
                output,
                output_len,
                b".white       {color: white;}\n\0" as *const u8 as *const libc::c_char,
            );
            append_string(
                output,
                output_len,
                b".bg-black    {background-color: black;}\n\0" as *const u8
                    as *const libc::c_char,
            );
            append_string(
                output,
                output_len,
                b".bg-red      {background-color: red;}\n\0" as *const u8
                    as *const libc::c_char,
            );
            append_string(
                output,
                output_len,
                b".bg-green    {background-color: lime;}\n\0" as *const u8
                    as *const libc::c_char,
            );
            append_string(
                output,
                output_len,
                b".bg-yellow   {background-color: yellow;}\n\0" as *const u8
                    as *const libc::c_char,
            );
            append_string(
                output,
                output_len,
                b".bg-blue     {background-color: #3333FF;}\n\0" as *const u8
                    as *const libc::c_char,
            );
            append_string(
                output,
                output_len,
                b".bg-purple   {background-color: fuchsia;}\n\0" as *const u8
                    as *const libc::c_char,
            );
            append_string(
                output,
                output_len,
                b".bg-cyan     {background-color: aqua;}\n\0" as *const u8
                    as *const libc::c_char,
            );
            append_string(
                output,
                output_len,
                b".bg-white    {background-color: white;}\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        append_string(
            output,
            output_len,
            b".underline   {text-decoration: underline;}\n\0" as *const u8
                as *const libc::c_char,
        );
        append_string(
            output,
            output_len,
            b".bold        {font-weight: bold;}\n\0" as *const u8 as *const libc::c_char,
        );
        append_string(
            output,
            output_len,
            b".italic      {font-style: italic;}\n\0" as *const u8 as *const libc::c_char,
        );
        append_string(
            output,
            output_len,
            b".blink       {text-decoration: blink;}\n\0" as *const u8
                as *const libc::c_char,
        );
        append_string(
            output,
            output_len,
            b".crossed-out {text-decoration: line-through;}\n\0" as *const u8
                as *const libc::c_char,
        );
        append_string(
            output,
            output_len,
            b".highlighted {filter: contrast(70%%) brightness(190%%);}\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if (*opts).word_wrap != 0 {
        if style_tag == 0 {
            append_string(
                output,
                output_len,
                b"<style type=\"text/css\">\n\0" as *const u8 as *const libc::c_char,
            );
            style_tag = 1 as libc::c_int;
        }
        append_string(
            output,
            output_len,
            b"pre {white-space: pre-wrap; white-space: -moz-pre-wrap !important;\n\0"
                as *const u8 as *const libc::c_char,
        );
        append_string(
            output,
            output_len,
            b"white-space: -pre-wrap; white-space: -o-pre-wrap; word-wrap: break-word;}\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if style_tag != 0 {
        append_string(
            output,
            output_len,
            b"</style>\n\0" as *const u8 as *const libc::c_char,
        );
    }
    append_string(
        output,
        output_len,
        b"</head>\n\0" as *const u8 as *const libc::c_char,
    );
    if (*opts).stylesheet != 0 {
        append_string(
            output,
            output_len,
            b"<body>\n\0" as *const u8 as *const libc::c_char,
        );
    } else {
        append_string(
            output,
            output_len,
            b"<body\0" as *const u8 as *const libc::c_char,
        );
        if !((*opts).bodystyle).is_null()
            || (*opts).colorscheme as libc::c_uint
                == SCHEME_BLACK as libc::c_int as libc::c_uint
            || (*opts).colorscheme as libc::c_uint
                == SCHEME_PINK as libc::c_int as libc::c_uint
        {
            let mut styles: libc::c_int = 0 as libc::c_int;
            append_string(
                output,
                output_len,
                b" style=\"\0" as *const u8 as *const libc::c_char,
            );
            if (*opts).colorscheme as libc::c_uint
                == SCHEME_BLACK as libc::c_int as libc::c_uint
            {
                styles += 1;
                append_string(
                    output,
                    output_len,
                    b"color:white; background-color:black\0" as *const u8
                        as *const libc::c_char,
                );
            } else if (*opts).colorscheme as libc::c_uint
                == SCHEME_PINK as libc::c_int as libc::c_uint
            {
                styles += 1;
                append_string(
                    output,
                    output_len,
                    b"background-color:pink\0" as *const u8 as *const libc::c_char,
                );
            }
            if !((*opts).bodystyle).is_null() {
                if styles != 0 {
                    append_string(
                        output,
                        output_len,
                        b";\0" as *const u8 as *const libc::c_char,
                    );
                }
                printHtml(output, output_len, (*opts).bodystyle);
            }
            append_string(
                output,
                output_len,
                b"\"\0" as *const u8 as *const libc::c_char,
            );
        }
        append_string(output, output_len, b">\n\0" as *const u8 as *const libc::c_char);
    }
    append_string(output, output_len, b"<pre>\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn convert_ansi_to_html(
    mut s: *mut libc::c_char,
    mut len: libc::c_uint,
) -> *mut libc::c_char {
    let mut opts: Options = {
        let mut init = Options {
            colorscheme: SCHEME_WHITE,
            filename: 0 as *mut libc::c_char,
            htop_fix: 1 as libc::c_int,
            iso: -(1 as libc::c_int),
            line_break: 0 as libc::c_int,
            no_header: 0 as libc::c_int,
            stylesheet: 0 as libc::c_int,
            title: 0 as *mut libc::c_char,
            word_wrap: 0 as libc::c_int,
            no_xml: 0 as libc::c_int,
            lang: 0 as *mut libc::c_char,
            css: 0 as *mut libc::c_char,
            ignore_cr: 0 as libc::c_int,
            bodystyle: 0 as *mut libc::c_char,
        };
        init
    };
    let mut output: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut output_len: size_t = 0 as libc::c_int as size_t;
    let mut fcstyle: [*mut libc::c_char; 10] = [
        (if opts.stylesheet != 0 {
            b"dimgray \0" as *const u8 as *const libc::c_char
        } else {
            b"color:dimgray;\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char,
        (if opts.stylesheet != 0 {
            b"red \0" as *const u8 as *const libc::c_char
        } else {
            b"color:red;\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char,
        (if opts.stylesheet != 0 {
            b"green \0" as *const u8 as *const libc::c_char
        } else if opts.colorscheme as libc::c_uint
            == SCHEME_BLACK as libc::c_int as libc::c_uint
        {
            b"color:lime;\0" as *const u8 as *const libc::c_char
        } else {
            b"color:green;\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char,
        (if opts.stylesheet != 0 {
            b"yellow \0" as *const u8 as *const libc::c_char
        } else if opts.colorscheme as libc::c_uint
            == SCHEME_BLACK as libc::c_int as libc::c_uint
        {
            b"color:yellow;\0" as *const u8 as *const libc::c_char
        } else {
            b"color:olive;\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char,
        (if opts.stylesheet != 0 {
            b"blue \0" as *const u8 as *const libc::c_char
        } else if opts.colorscheme as libc::c_uint
            == SCHEME_BLACK as libc::c_int as libc::c_uint
        {
            b"color:#3333FF;\0" as *const u8 as *const libc::c_char
        } else {
            b"color:blue;\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char,
        (if opts.stylesheet != 0 {
            b"purple \0" as *const u8 as *const libc::c_char
        } else if opts.colorscheme as libc::c_uint
            == SCHEME_BLACK as libc::c_int as libc::c_uint
        {
            b"color:fuchsia;\0" as *const u8 as *const libc::c_char
        } else {
            b"color:purple;\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char,
        (if opts.stylesheet != 0 {
            b"cyan \0" as *const u8 as *const libc::c_char
        } else if opts.colorscheme as libc::c_uint
            == SCHEME_BLACK as libc::c_int as libc::c_uint
        {
            b"color:aqua;\0" as *const u8 as *const libc::c_char
        } else {
            b"color:teal;\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char,
        (if opts.stylesheet != 0 {
            b"white \0" as *const u8 as *const libc::c_char
        } else if opts.colorscheme as libc::c_uint
            == SCHEME_BLACK as libc::c_int as libc::c_uint
        {
            b"color:white;\0" as *const u8 as *const libc::c_char
        } else {
            b"color:gray;\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char,
        (if opts.stylesheet != 0 {
            b"inverted \0" as *const u8 as *const libc::c_char
        } else if opts.colorscheme as libc::c_uint
            == SCHEME_BLACK as libc::c_int as libc::c_uint
        {
            b"color:black;\0" as *const u8 as *const libc::c_char
        } else if opts.colorscheme as libc::c_uint
            == SCHEME_PINK as libc::c_int as libc::c_uint
        {
            b"color:pink;\0" as *const u8 as *const libc::c_char
        } else {
            b"color:white;\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char,
        (if opts.stylesheet != 0 {
            b"reset \0" as *const u8 as *const libc::c_char
        } else if opts.colorscheme as libc::c_uint
            == SCHEME_BLACK as libc::c_int as libc::c_uint
        {
            b"color:white;\0" as *const u8 as *const libc::c_char
        } else {
            b"color:black;\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char,
    ];
    let mut bcstyle: [*mut libc::c_char; 10] = [
        (if opts.stylesheet != 0 {
            b"bg-black \0" as *const u8 as *const libc::c_char
        } else {
            b"background-color:black;\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char,
        (if opts.stylesheet != 0 {
            b"bg-red \0" as *const u8 as *const libc::c_char
        } else {
            b"background-color:red;\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char,
        (if opts.stylesheet != 0 {
            b"bg-green \0" as *const u8 as *const libc::c_char
        } else if opts.colorscheme as libc::c_uint
            == SCHEME_BLACK as libc::c_int as libc::c_uint
        {
            b"background-color:lime;\0" as *const u8 as *const libc::c_char
        } else {
            b"background-color:green;\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char,
        (if opts.stylesheet != 0 {
            b"bg-yellow \0" as *const u8 as *const libc::c_char
        } else if opts.colorscheme as libc::c_uint
            == SCHEME_BLACK as libc::c_int as libc::c_uint
        {
            b"background-color:yellow;\0" as *const u8 as *const libc::c_char
        } else {
            b"background-color:olive;\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char,
        (if opts.stylesheet != 0 {
            b"bg-blue \0" as *const u8 as *const libc::c_char
        } else if opts.colorscheme as libc::c_uint
            == SCHEME_BLACK as libc::c_int as libc::c_uint
        {
            b"background-color:#3333FF;\0" as *const u8 as *const libc::c_char
        } else {
            b"background-color:blue;\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char,
        (if opts.stylesheet != 0 {
            b"bg-purple \0" as *const u8 as *const libc::c_char
        } else if opts.colorscheme as libc::c_uint
            == SCHEME_BLACK as libc::c_int as libc::c_uint
        {
            b"background-color:fuchsia;\0" as *const u8 as *const libc::c_char
        } else {
            b"background-color:purple;\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char,
        (if opts.stylesheet != 0 {
            b"bg-cyan \0" as *const u8 as *const libc::c_char
        } else if opts.colorscheme as libc::c_uint
            == SCHEME_BLACK as libc::c_int as libc::c_uint
        {
            b"background-color:aqua;\0" as *const u8 as *const libc::c_char
        } else {
            b"background-color:teal;\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char,
        (if opts.stylesheet != 0 {
            b"bg-white \0" as *const u8 as *const libc::c_char
        } else if opts.colorscheme as libc::c_uint
            == SCHEME_BLACK as libc::c_int as libc::c_uint
        {
            b"background-color:white;\0" as *const u8 as *const libc::c_char
        } else {
            b"background-color:gray;\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char,
        (if opts.stylesheet != 0 {
            b"bg-reset \0" as *const u8 as *const libc::c_char
        } else if opts.colorscheme as libc::c_uint
            == SCHEME_BLACK as libc::c_int as libc::c_uint
        {
            b"background-color:black;\0" as *const u8 as *const libc::c_char
        } else if opts.colorscheme as libc::c_uint
            == SCHEME_PINK as libc::c_int as libc::c_uint
        {
            b"background-color:pink;\0" as *const u8 as *const libc::c_char
        } else {
            b"background-color:white;\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char,
        (if opts.stylesheet != 0 {
            b"bg-inverted \0" as *const u8 as *const libc::c_char
        } else if opts.colorscheme as libc::c_uint
            == SCHEME_BLACK as libc::c_int as libc::c_uint
        {
            b"background-color:white;\0" as *const u8 as *const libc::c_char
        } else {
            b"background-color:black;\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char,
    ];
    if opts.no_header == 0 {
        printHeader(&mut opts, &mut output, &mut output_len);
    }
    let mut state: State = default_state;
    let mut oldstate: State = State {
        fc: 0,
        bc: 0,
        bold: 0,
        italic: 0,
        underline: 0,
        blink: 0,
        crossedout: 0,
        fc_colormode: MODE_3BIT,
        bc_colormode: MODE_3BIT,
        highlighted: 0,
    };
    let mut idx: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut c: libc::c_int = 0;
    let mut negative: libc::c_int = 0 as libc::c_int;
    let mut special_char: libc::c_int = 0 as libc::c_int;
    let mut line: libc::c_int = 0 as libc::c_int;
    let mut momline: libc::c_int = 0 as libc::c_int;
    let mut newline: libc::c_int = -(1 as libc::c_int);
    's_53: loop {
        c = getNextChar(s, &mut idx, len);
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        if c == '\u{1b}' as i32 {
            oldstate = state;
            c = getNextChar(s, &mut idx, len);
            if c == -(1 as libc::c_int) {
                break;
            }
            if c == '[' as i32 {
                let mut buffer: [libc::c_char; 1024] = [0; 1024];
                buffer[0 as libc::c_int as usize] = '[' as i32 as libc::c_char;
                let mut counter: libc::c_int = 1 as libc::c_int;
                while c < 'A' as i32 || c > 'Z' as i32 && c < 'a' as i32
                    || c > 'z' as i32
                {
                    c = getNextChar(s, &mut idx, len);
                    if c == -(1 as libc::c_int) {
                        break;
                    }
                    buffer[counter as usize] = c as libc::c_char;
                    if c == '>' as i32 {
                        break;
                    }
                    counter += 1;
                    if counter > 1022 as libc::c_int {
                        break;
                    }
                }
                buffer[(counter - 1 as libc::c_int)
                    as usize] = 0 as libc::c_int as libc::c_char;
                let mut elem: pelem = 0 as *mut selem;
                let mut momelem: pelem = 0 as *mut selem;
                match c {
                    109 => {
                        elem = parseInsert(buffer.as_mut_ptr());
                        momelem = elem;
                        while !momelem.is_null() {
                            match (*momelem).value {
                                0 => {
                                    state = default_state;
                                    negative = 0 as libc::c_int;
                                    special_char = 0 as libc::c_int;
                                }
                                1 => {
                                    state.bold = 1 as libc::c_int;
                                }
                                3 => {
                                    state.italic = 1 as libc::c_int;
                                }
                                4 => {
                                    state.underline = 1 as libc::c_int;
                                }
                                5 => {
                                    state.blink = 1 as libc::c_int;
                                }
                                7 => {
                                    swapColors(&mut state);
                                    negative = (negative == 0) as libc::c_int;
                                }
                                9 => {
                                    state.crossedout = 1 as libc::c_int;
                                }
                                21 | 22 => {
                                    state.bold = 0 as libc::c_int;
                                }
                                23 => {
                                    state.italic = 0 as libc::c_int;
                                }
                                24 => {
                                    state.underline = 0 as libc::c_int;
                                }
                                25 => {
                                    state.blink = 0 as libc::c_int;
                                }
                                27 => {
                                    if negative != 0 {
                                        swapColors(&mut state);
                                        negative = 0 as libc::c_int;
                                    }
                                }
                                29 => {
                                    state.crossedout = 0 as libc::c_int;
                                }
                                30 | 31 | 32 | 33 | 34 | 35 | 36 | 37 | 38 => {
                                    let mut dest: *mut libc::c_int = &mut state.fc;
                                    if negative != 0 as libc::c_int {
                                        dest = &mut state.bc;
                                    }
                                    if (*momelem).value == 38 as libc::c_int as libc::c_long
                                        && !((*momelem).next).is_null()
                                        && (*(*momelem).next).value
                                            == 5 as libc::c_int as libc::c_long
                                        && !((*(*momelem).next).next).is_null()
                                    {
                                        momelem = (*(*momelem).next).next;
                                        state.fc_colormode = MODE_8BIT;
                                        if (*momelem).value >= 8 as libc::c_int as libc::c_long
                                            && (*momelem).value <= 15 as libc::c_int as libc::c_long
                                        {
                                            state.highlighted = 1 as libc::c_int;
                                            *dest = ((*momelem).value
                                                - 8 as libc::c_int as libc::c_long) as libc::c_int;
                                        } else {
                                            state.highlighted = 0 as libc::c_int;
                                            *dest = (*momelem).value as libc::c_int;
                                        }
                                    } else if (*momelem).value
                                        == 38 as libc::c_int as libc::c_long
                                        && !((*momelem).next).is_null()
                                        && (*(*momelem).next).value
                                            == 2 as libc::c_int as libc::c_long
                                        && !((*(*momelem).next).next).is_null()
                                    {
                                        momelem = (*(*momelem).next).next;
                                        let mut r: pelem = 0 as *mut selem;
                                        let mut g: pelem = 0 as *mut selem;
                                        let mut b: pelem = 0 as *mut selem;
                                        r = momelem;
                                        momelem = (*momelem).next;
                                        g = momelem;
                                        if !momelem.is_null() {
                                            momelem = (*momelem).next;
                                        }
                                        b = momelem;
                                        if !r.is_null() && !g.is_null() && !b.is_null() {
                                            state.highlighted = 0 as libc::c_int;
                                            state.fc_colormode = MODE_24BIT;
                                            *dest = (((*r).value & 255 as libc::c_int as libc::c_long)
                                                * 65536 as libc::c_int as libc::c_long
                                                + ((*g).value & 255 as libc::c_int as libc::c_long)
                                                    * 256 as libc::c_int as libc::c_long
                                                + ((*b).value & 255 as libc::c_int as libc::c_long))
                                                as libc::c_int;
                                        }
                                    } else {
                                        state.fc_colormode = MODE_3BIT;
                                        state.highlighted = 0 as libc::c_int;
                                        *dest = ((*momelem).value
                                            - 30 as libc::c_int as libc::c_long) as libc::c_int;
                                    }
                                }
                                39 => {
                                    state.fc_colormode = MODE_3BIT;
                                    state.highlighted = 0 as libc::c_int;
                                    state.fc = -(1 as libc::c_int);
                                }
                                40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 | 48 => {
                                    let mut dest_0: *mut libc::c_int = &mut state.bc;
                                    if negative != 0 as libc::c_int {
                                        dest_0 = &mut state.fc;
                                    }
                                    if (*momelem).value == 48 as libc::c_int as libc::c_long
                                        && !((*momelem).next).is_null()
                                        && (*(*momelem).next).value
                                            == 5 as libc::c_int as libc::c_long
                                        && !((*(*momelem).next).next).is_null()
                                    {
                                        momelem = (*(*momelem).next).next;
                                        state.bc_colormode = MODE_8BIT;
                                        if (*momelem).value >= 8 as libc::c_int as libc::c_long
                                            && (*momelem).value <= 15 as libc::c_int as libc::c_long
                                        {
                                            state.highlighted = 1 as libc::c_int;
                                            *dest_0 = ((*momelem).value
                                                - 8 as libc::c_int as libc::c_long) as libc::c_int;
                                        } else {
                                            state.highlighted = 0 as libc::c_int;
                                            *dest_0 = (*momelem).value as libc::c_int;
                                        }
                                    } else if (*momelem).value
                                        == 48 as libc::c_int as libc::c_long
                                        && !((*momelem).next).is_null()
                                        && (*(*momelem).next).value
                                            == 2 as libc::c_int as libc::c_long
                                        && !((*(*momelem).next).next).is_null()
                                    {
                                        momelem = (*(*momelem).next).next;
                                        let mut r_0: pelem = 0 as *mut selem;
                                        let mut g_0: pelem = 0 as *mut selem;
                                        let mut b_0: pelem = 0 as *mut selem;
                                        r_0 = momelem;
                                        momelem = (*momelem).next;
                                        g_0 = momelem;
                                        if !momelem.is_null() {
                                            momelem = (*momelem).next;
                                        }
                                        b_0 = momelem;
                                        if !r_0.is_null() && !g_0.is_null() && !b_0.is_null() {
                                            state.bc_colormode = MODE_24BIT;
                                            state.highlighted = 0 as libc::c_int;
                                            *dest_0 = (((*r_0).value
                                                & 255 as libc::c_int as libc::c_long)
                                                * 65536 as libc::c_int as libc::c_long
                                                + ((*g_0).value & 255 as libc::c_int as libc::c_long)
                                                    * 256 as libc::c_int as libc::c_long
                                                + ((*b_0).value & 255 as libc::c_int as libc::c_long))
                                                as libc::c_int;
                                        }
                                    } else {
                                        state.bc_colormode = MODE_3BIT;
                                        state.highlighted = 0 as libc::c_int;
                                        *dest_0 = ((*momelem).value
                                            - 40 as libc::c_int as libc::c_long) as libc::c_int;
                                    }
                                }
                                49 => {
                                    state.bc_colormode = MODE_3BIT;
                                    state.highlighted = 0 as libc::c_int;
                                    state.bc = -(1 as libc::c_int);
                                }
                                90 | 91 | 92 | 93 | 94 | 95 | 96 | 97 => {
                                    let mut dest_1: *mut libc::c_int = &mut state.fc;
                                    if negative != 0 as libc::c_int {
                                        dest_1 = &mut state.bc;
                                    }
                                    state.fc_colormode = MODE_3BIT;
                                    state.highlighted = 1 as libc::c_int;
                                    *dest_1 = ((*momelem).value
                                        - 90 as libc::c_int as libc::c_long) as libc::c_int;
                                }
                                100 | 101 | 102 | 103 | 104 | 105 | 106 | 107 => {
                                    let mut dest_2: *mut libc::c_int = &mut state.bc;
                                    if negative != 0 as libc::c_int {
                                        dest_2 = &mut state.fc;
                                    }
                                    state.bc_colormode = MODE_3BIT;
                                    state.highlighted = 1 as libc::c_int;
                                    *dest_2 = ((*momelem).value
                                        - 100 as libc::c_int as libc::c_long) as libc::c_int;
                                }
                                _ => {}
                            }
                            momelem = (*momelem).next;
                        }
                        deleteParse(elem);
                    }
                    72 => {
                        if opts.htop_fix != 0 {
                            elem = parseInsert(buffer.as_mut_ptr());
                            let mut second: pelem = (*elem).next;
                            if second.is_null() {
                                second = elem;
                            }
                            newline = (*second).digit[0 as libc::c_int as usize]
                                as libc::c_int - 1 as libc::c_int;
                            if (*second).digitcount as libc::c_int > 1 as libc::c_int {
                                newline = (newline + 1 as libc::c_int) * 10 as libc::c_int
                                    + (*second).digit[1 as libc::c_int as usize] as libc::c_int
                                    - 1 as libc::c_int;
                            }
                            if (*second).digitcount as libc::c_int > 2 as libc::c_int {
                                newline = (newline + 1 as libc::c_int) * 10 as libc::c_int
                                    + (*second).digit[2 as libc::c_int as usize] as libc::c_int
                                    - 1 as libc::c_int;
                            }
                            deleteParse(elem);
                            if newline < line {
                                opts.line_break = 1 as libc::c_int;
                            }
                        }
                    }
                    _ => {}
                }
                if opts.htop_fix != 0 {
                    if opts.line_break != 0 {
                        while line < 80 as libc::c_int {
                            append_string(
                                &mut output,
                                &mut output_len,
                                b" \0" as *const u8 as *const libc::c_char,
                            );
                            line += 1;
                        }
                    }
                }
                if statesDiffer(&mut state, &mut oldstate) != 0 {
                    if statesDiffer(&mut oldstate, &default_state) != 0 {
                        append_string(
                            &mut output,
                            &mut output_len,
                            b"</span>\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    if statesDiffer(&mut state, &default_state) != 0 {
                        if opts.stylesheet != 0 {
                            append_string(
                                &mut output,
                                &mut output_len,
                                b"<span class=\"\0" as *const u8 as *const libc::c_char,
                            );
                        } else {
                            append_string(
                                &mut output,
                                &mut output_len,
                                b"<span style=\"\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        if state.underline != 0 {
                            if opts.stylesheet != 0 {
                                append_string(
                                    &mut output,
                                    &mut output_len,
                                    b"underline \0" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                append_string(
                                    &mut output,
                                    &mut output_len,
                                    b"text-decoration:underline;\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                        }
                        if state.bold != 0 {
                            if opts.stylesheet != 0 {
                                append_string(
                                    &mut output,
                                    &mut output_len,
                                    b"bold \0" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                append_string(
                                    &mut output,
                                    &mut output_len,
                                    b"font-weight:bold;\0" as *const u8 as *const libc::c_char,
                                );
                            }
                        }
                        if state.italic != 0 {
                            if opts.stylesheet != 0 {
                                append_string(
                                    &mut output,
                                    &mut output_len,
                                    b"italic \0" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                append_string(
                                    &mut output,
                                    &mut output_len,
                                    b"font-style:italic;\0" as *const u8 as *const libc::c_char,
                                );
                            }
                        }
                        if state.blink != 0 {
                            if opts.stylesheet != 0 {
                                append_string(
                                    &mut output,
                                    &mut output_len,
                                    b"blink \0" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                append_string(
                                    &mut output,
                                    &mut output_len,
                                    b"text-decoration:blink;\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                        }
                        if state.crossedout != 0 {
                            if opts.stylesheet != 0 {
                                append_string(
                                    &mut output,
                                    &mut output_len,
                                    b"crossed-out \0" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                append_string(
                                    &mut output,
                                    &mut output_len,
                                    b"text-decoration:line-through;\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                        }
                        if state.highlighted != 0 {
                            if opts.stylesheet != 0 {
                                append_string(
                                    &mut output,
                                    &mut output_len,
                                    b"highlighted \0" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                append_string(
                                    &mut output,
                                    &mut output_len,
                                    b"filter: contrast(70%%) brightness(190%%);\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                        }
                        if opts.stylesheet != 0
                            && state.fc_colormode as libc::c_uint
                                != MODE_3BIT as libc::c_int as libc::c_uint
                            && (state.fc_colormode as libc::c_uint
                                != MODE_8BIT as libc::c_int as libc::c_uint
                                || state.fc > 15 as libc::c_int)
                        {
                            append_string(
                                &mut output,
                                &mut output_len,
                                b"\" style=\"\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        match state.fc_colormode as libc::c_uint {
                            0 => {
                                if state.fc >= 0 as libc::c_int
                                    && state.fc <= 9 as libc::c_int
                                {
                                    append_string(
                                        &mut output,
                                        &mut output_len,
                                        fcstyle[state.fc as usize],
                                    );
                                }
                            }
                            1 => {
                                if state.fc >= 0 as libc::c_int
                                    && state.fc <= 7 as libc::c_int
                                {
                                    append_string(
                                        &mut output,
                                        &mut output_len,
                                        fcstyle[state.fc as usize],
                                    );
                                } else {
                                    let mut rgb: [libc::c_char; 12] = [0; 12];
                                    make_rgb(state.fc, rgb.as_mut_ptr());
                                    let mut temp: *mut libc::c_char = malloc(
                                        100 as libc::c_int as libc::c_ulong,
                                    ) as *mut libc::c_char;
                                    sprintf(
                                        temp,
                                        b"color:#%s;\0" as *const u8 as *const libc::c_char,
                                        rgb.as_mut_ptr(),
                                    );
                                    append_string(&mut output, &mut output_len, temp);
                                    free(temp as *mut libc::c_void);
                                }
                            }
                            2 => {
                                let mut temp_0: *mut libc::c_char = malloc(
                                    100 as libc::c_int as libc::c_ulong,
                                ) as *mut libc::c_char;
                                sprintf(
                                    temp_0,
                                    b"color:#%06x;\0" as *const u8 as *const libc::c_char,
                                    state.fc,
                                );
                                append_string(&mut output, &mut output_len, temp_0);
                                free(temp_0 as *mut libc::c_void);
                            }
                            _ => {}
                        }
                        if opts.stylesheet != 0
                            && !(state.fc_colormode as libc::c_uint
                                != MODE_3BIT as libc::c_int as libc::c_uint
                                && (state.fc_colormode as libc::c_uint
                                    != MODE_8BIT as libc::c_int as libc::c_uint
                                    || state.fc > 15 as libc::c_int))
                            && state.bc_colormode as libc::c_uint
                                != MODE_3BIT as libc::c_int as libc::c_uint
                            && (state.bc_colormode as libc::c_uint
                                != MODE_8BIT as libc::c_int as libc::c_uint
                                || state.bc > 15 as libc::c_int)
                        {
                            append_string(
                                &mut output,
                                &mut output_len,
                                b"\" style=\"\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        match state.bc_colormode as libc::c_uint {
                            0 => {
                                if state.bc >= 0 as libc::c_int
                                    && state.bc <= 9 as libc::c_int
                                {
                                    append_string(
                                        &mut output,
                                        &mut output_len,
                                        bcstyle[state.bc as usize],
                                    );
                                }
                            }
                            1 => {
                                if state.bc >= 0 as libc::c_int
                                    && state.bc <= 7 as libc::c_int
                                {
                                    append_string(
                                        &mut output,
                                        &mut output_len,
                                        bcstyle[state.bc as usize],
                                    );
                                } else {
                                    let mut rgb_0: [libc::c_char; 12] = [0; 12];
                                    make_rgb(state.bc, rgb_0.as_mut_ptr());
                                    let mut temp_1: *mut libc::c_char = malloc(
                                        100 as libc::c_int as libc::c_ulong,
                                    ) as *mut libc::c_char;
                                    sprintf(
                                        temp_1,
                                        b"background-color:#%s;\0" as *const u8
                                            as *const libc::c_char,
                                        rgb_0.as_mut_ptr(),
                                    );
                                    append_string(&mut output, &mut output_len, temp_1);
                                    free(temp_1 as *mut libc::c_void);
                                }
                            }
                            2 => {
                                let mut temp_2: *mut libc::c_char = malloc(
                                    100 as libc::c_int as libc::c_ulong,
                                ) as *mut libc::c_char;
                                sprintf(
                                    temp_2,
                                    b"background-color:#%06x;\0" as *const u8
                                        as *const libc::c_char,
                                    state.bc,
                                );
                                append_string(&mut output, &mut output_len, temp_2);
                                free(temp_2 as *mut libc::c_void);
                            }
                            _ => {}
                        }
                        append_string(
                            &mut output,
                            &mut output_len,
                            b"\">\0" as *const u8 as *const libc::c_char,
                        );
                    }
                }
            } else if c == ']' as i32 {
                while c != 2 as libc::c_int && c != 7 as libc::c_int
                    && c != 27 as libc::c_int
                {
                    c = getNextChar(s, &mut idx, len);
                    if c == -(1 as libc::c_int) {
                        break 's_53;
                    }
                }
                if !(c == 27 as libc::c_int) {
                    continue;
                }
                c = getNextChar(s, &mut idx, len);
                if c == -(1 as libc::c_int) {
                    break;
                }
            } else {
                if !(c == '(' as i32) {
                    continue;
                }
                c = getNextChar(s, &mut idx, len);
                if c == -(1 as libc::c_int) {
                    break;
                }
                if c == '0' as i32 {
                    special_char = 1 as libc::c_int;
                } else {
                    special_char = 0 as libc::c_int;
                }
            }
        } else if c == 13 as libc::c_int && opts.htop_fix != 0 {
            while line < 80 as libc::c_int {
                append_string(
                    &mut output,
                    &mut output_len,
                    b" \0" as *const u8 as *const libc::c_char,
                );
                line += 1;
            }
            line = 0 as libc::c_int;
            momline += 1;
            append_string(
                &mut output,
                &mut output_len,
                b"\n\0" as *const u8 as *const libc::c_char,
            );
        } else {
            if c == 13 as libc::c_int && opts.ignore_cr != 0 {
                continue;
            }
            if !(c != 8 as libc::c_int) {
                continue;
            }
            line += 1;
            if opts.line_break != 0 {
                append_string(
                    &mut output,
                    &mut output_len,
                    b"\n\0" as *const u8 as *const libc::c_char,
                );
                line = 0 as libc::c_int;
                opts.line_break = 0 as libc::c_int;
                momline += 1;
            }
            if newline >= 0 as libc::c_int {
                while newline > line {
                    append_string(
                        &mut output,
                        &mut output_len,
                        b" \0" as *const u8 as *const libc::c_char,
                    );
                    line += 1;
                }
                newline = -(1 as libc::c_int);
            }
            let mut current_block_251: u64;
            match c {
                34 => {
                    append_string(
                        &mut output,
                        &mut output_len,
                        b"&quot;\0" as *const u8 as *const libc::c_char,
                    );
                    current_block_251 = 16437638578768886202;
                }
                38 => {
                    append_string(
                        &mut output,
                        &mut output_len,
                        b"&amp;\0" as *const u8 as *const libc::c_char,
                    );
                    current_block_251 = 16437638578768886202;
                }
                60 => {
                    append_string(
                        &mut output,
                        &mut output_len,
                        b"&lt;\0" as *const u8 as *const libc::c_char,
                    );
                    current_block_251 = 16437638578768886202;
                }
                62 => {
                    append_string(
                        &mut output,
                        &mut output_len,
                        b"&gt;\0" as *const u8 as *const libc::c_char,
                    );
                    current_block_251 = 16437638578768886202;
                }
                10 | 13 => {
                    momline += 1;
                    line = 0 as libc::c_int;
                    current_block_251 = 18426084582889649261;
                }
                _ => {
                    current_block_251 = 18426084582889649261;
                }
            }
            match current_block_251 {
                18426084582889649261 => {
                    if special_char != 0 {
                        append_string(
                            &mut output,
                            &mut output_len,
                            (ansi_vt220_character_set[(c + 32 as libc::c_int
                                & 255 as libc::c_int) as usize])
                                .as_ptr(),
                        );
                    } else {
                        let mut temp_3: *mut libc::c_char = malloc(
                            2 as libc::c_int as libc::c_ulong,
                        ) as *mut libc::c_char;
                        *temp_3.offset(0 as libc::c_int as isize) = c as libc::c_char;
                        *temp_3
                            .offset(
                                1 as libc::c_int as isize,
                            ) = '\0' as i32 as libc::c_char;
                        append_string(&mut output, &mut output_len, temp_3);
                        free(temp_3 as *mut libc::c_void);
                    }
                }
                _ => {}
            }
            if !(opts.iso > 0 as libc::c_int) {
                continue;
            }
            if !(c & 128 as libc::c_int == 128 as libc::c_int) {
                continue;
            }
            let mut bits: libc::c_int = 2 as libc::c_int;
            if c & 32 as libc::c_int == 32 as libc::c_int {
                bits += 1;
            }
            if c & 16 as libc::c_int == 16 as libc::c_int {
                bits += 1;
            }
            let mut meow: libc::c_int = 1 as libc::c_int;
            while meow < bits {
                let mut temp_4: *mut libc::c_char = malloc(
                    2 as libc::c_int as libc::c_ulong,
                ) as *mut libc::c_char;
                *temp_4
                    .offset(
                        0 as libc::c_int as isize,
                    ) = getNextChar(s, &mut idx, len) as libc::c_char;
                *temp_4.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
                append_string(&mut output, &mut output_len, temp_4);
                if *temp_4.offset(0 as libc::c_int as isize) as libc::c_int
                    == -(1 as libc::c_int)
                {
                    free(temp_4 as *mut libc::c_void);
                    break 's_53;
                } else {
                    free(temp_4 as *mut libc::c_void);
                    meow += 1;
                }
            }
        }
    }
    if statesDiffer(&mut state, &default_state) != 0 {
        append_string(
            &mut output,
            &mut output_len,
            b"</span>\0" as *const u8 as *const libc::c_char,
        );
    }
    if opts.no_header == 0 as libc::c_int {
        append_string(
            &mut output,
            &mut output_len,
            b"</pre>\n\0" as *const u8 as *const libc::c_char,
        );
        append_string(
            &mut output,
            &mut output_len,
            b"</body>\n\0" as *const u8 as *const libc::c_char,
        );
        append_string(
            &mut output,
            &mut output_len,
            b"</html>\n\0" as *const u8 as *const libc::c_char,
        );
    }
    return output;
}
