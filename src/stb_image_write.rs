use crate::*;

use std::alloc;
use std::mem;

pub trait One {
    fn one() -> Self;
}

impl One for i32 {
    fn one() -> Self {
        1
    }
}

impl One for u32 {
    fn one() -> Self {
        1
    }
}

impl One for u8 {
    fn one() -> Self {
        1
    }
}

impl One for u16 {
    fn one() -> Self {
        1
    }
}

pub unsafe fn postInc<T: std::ops::AddAssign + One + Copy>(mut a: *mut T) -> T {
    let mut result: T = *a;
    *a += One::one();
    return result;
}

pub unsafe fn preInc<T: std::ops::AddAssign + One + Copy>(mut a: *mut T) -> T {
    *a += One::one();
    return *a;
}

pub unsafe fn postDec<T: std::ops::SubAssign + One + Copy>(mut a: *mut T) -> T {
    let mut result: T = *a;
    *a -= One::one();
    return result;
}

pub unsafe fn preDec<T: std::ops::SubAssign + One + Copy>(mut a: *mut T) -> T {
    *a -= One::one();
    return *a;
}

pub unsafe fn preIncPtr<T>(mut a: *mut *mut T) -> *mut T {
    *a = (*a).offset(1);
    return *a;
}

pub unsafe fn preDecPtr<T>(mut a: *mut *mut T) -> *mut T {
    *a = (*a).offset(-1);
    return *a;
}

pub unsafe fn postIncPtr<T>(mut a: *mut *mut T) -> *mut T {
    let mut result: *mut T = *a;
    *a = (*a).offset(1);
    return result;
}

pub unsafe fn postDecPtr<T>(mut a: *mut *mut T) -> *mut T {
    let mut result: *mut T = *a;
    *a = (*a).offset(-1);
    return result;
}

pub unsafe fn memcpy(src: *mut u8, dest: *const u8, count: u64) {
    std::ptr::copy_nonoverlapping(dest, src, count as usize);
}

pub unsafe fn memset(src: *mut u8, value: i32, count: u64) {
    std::ptr::write_bytes(src, value as u8, count as usize);
}

pub unsafe fn malloc(count: u64) -> *mut u8 {
    let layout = std::alloc::Layout::from_size_align(count as usize, 1).expect("Bad layout");

    return std::alloc::alloc(layout);
}

pub unsafe fn realloc<T>(data: *mut T, count: u64) -> *mut u8 {
    if (data == std::ptr::null_mut()) {
        return malloc(count);
    }

    let layout = std::alloc::Layout::from_size_align(count as usize, 1).expect("Bad layout");

    return std::alloc::realloc(data as *mut u8, layout, count as usize);
}

pub unsafe fn free<T>(data: *mut T) {
    let layout = std::alloc::Layout::from_size_align(1, 1).expect("Bad layout");

    std::alloc::dealloc(data as *mut u8, layout);
}

pub fn _lrotl(x: u32, y: i32) -> u32 {
    return (x << y) | (x >> (32 - y));
}

pub fn abs(x: i32) -> i32 {
    return i32::abs(x);
}

pub fn pow(x: f32, p: f32) -> f32 {
    return x.powf(p);
}

pub fn fabs(x: f32) -> f32 {
    return f32::abs(x);
}

pub fn fmod(x: f32, y: f32) -> f32 {
    return x % y;
}

pub unsafe fn strlen(str: *mut u8) -> i32 {
    let mut ptr = str;
    let mut result = 0;

    while *ptr != 0 {
        ptr = ptr.offset(1);
        result += 1;
    }

    return result;
}

pub fn sqrt(x: f32) -> f32 {
    return f32::sqrt(x);
}

pub fn acos(x: f32) -> f32 {
    return f32::acos(x);
}

pub fn cos(x: f32) -> f32 {
    return f32::cos(x);
}

pub fn floor(x: f32) -> f32 {
    return f32::floor(x);
}

pub fn ceil(x: f32) -> f32 {
    return f32::ceil(x);
}

pub unsafe fn memmove(a: *mut u8, b: *mut u8, size: u64) {
    let temp = malloc(size);
    memcpy(temp, b, size);
    memcpy(a, temp, size);
}

pub unsafe fn memcmp(a: *const u8, b: *const u8, size: u64) -> i32 {
    let mut result = 0;
    let mut ap = a;
    let mut bp = b;
    for i in 0..size - 1 {
        if *ap != *bp {
            result += 1;
        }

        ap = ap.offset(1);
        bp = bp.offset(1);
    }

    return result;
}

pub static mut stbi__flip_vertically_on_write: i32 = 0;
pub static mut stbi_write_force_png_filter: i32 = -1;
pub static mut stbi_write_png_compression_level: i32 = 8;
pub static mut stbi_write_tga_with_rle: i32 = 1;
pub static mut stbiw__jpg_ZigZag: [u8; 64] = [
    ((0) as u8),
    ((1) as u8),
    ((5) as u8),
    ((6) as u8),
    ((14) as u8),
    ((15) as u8),
    ((27) as u8),
    ((28) as u8),
    ((2) as u8),
    ((4) as u8),
    ((7) as u8),
    ((13) as u8),
    ((16) as u8),
    ((26) as u8),
    ((29) as u8),
    ((42) as u8),
    ((3) as u8),
    ((8) as u8),
    ((12) as u8),
    ((17) as u8),
    ((25) as u8),
    ((30) as u8),
    ((41) as u8),
    ((43) as u8),
    ((9) as u8),
    ((11) as u8),
    ((18) as u8),
    ((24) as u8),
    ((31) as u8),
    ((40) as u8),
    ((44) as u8),
    ((53) as u8),
    ((10) as u8),
    ((19) as u8),
    ((23) as u8),
    ((32) as u8),
    ((39) as u8),
    ((45) as u8),
    ((52) as u8),
    ((54) as u8),
    ((20) as u8),
    ((22) as u8),
    ((33) as u8),
    ((38) as u8),
    ((46) as u8),
    ((51) as u8),
    ((55) as u8),
    ((60) as u8),
    ((21) as u8),
    ((34) as u8),
    ((37) as u8),
    ((47) as u8),
    ((50) as u8),
    ((56) as u8),
    ((59) as u8),
    ((61) as u8),
    ((35) as u8),
    ((36) as u8),
    ((48) as u8),
    ((49) as u8),
    ((57) as u8),
    ((58) as u8),
    ((62) as u8),
    ((63) as u8),
];
pub static mut stbi_write_jpg_core_aasf: [f32; 8] = [
    1.0f32 * 2.828427125f32,
    1.387039845f32 * 2.828427125f32,
    1.306562965f32 * 2.828427125f32,
    1.175875602f32 * 2.828427125f32,
    1.0f32 * 2.828427125f32,
    0.785694958f32 * 2.828427125f32,
    0.541196100f32 * 2.828427125f32,
    0.275899379f32 * 2.828427125f32,
];
pub static mut stbi_write_jpg_core_fillBits: [u16; 2] = [((0x7F) as u16), ((7) as u16)];
pub static mut stbi_write_jpg_core_head0: [u8; 25] = [
    ((0xFF) as u8),
    ((0xD8) as u8),
    ((0xFF) as u8),
    ((0xE0) as u8),
    ((0) as u8),
    ((0x10) as u8),
    ((74) as u8),
    ((70) as u8),
    ((73) as u8),
    ((70) as u8),
    ((0) as u8),
    ((1) as u8),
    ((1) as u8),
    ((0) as u8),
    ((0) as u8),
    ((1) as u8),
    ((0) as u8),
    ((1) as u8),
    ((0) as u8),
    ((0) as u8),
    ((0xFF) as u8),
    ((0xDB) as u8),
    ((0) as u8),
    ((0x84) as u8),
    ((0) as u8),
];
pub static mut stbi_write_jpg_core_head2: [u8; 14] = [
    ((0xFF) as u8),
    ((0xDA) as u8),
    ((0) as u8),
    ((0xC) as u8),
    ((3) as u8),
    ((1) as u8),
    ((0) as u8),
    ((2) as u8),
    ((0x11) as u8),
    ((3) as u8),
    ((0x11) as u8),
    ((0) as u8),
    ((0x3F) as u8),
    ((0) as u8),
];
pub static mut stbi_write_jpg_core_std_ac_chrominance_nrcodes: [u8; 17] = [
    ((0) as u8),
    ((0) as u8),
    ((2) as u8),
    ((1) as u8),
    ((2) as u8),
    ((4) as u8),
    ((4) as u8),
    ((3) as u8),
    ((4) as u8),
    ((7) as u8),
    ((5) as u8),
    ((4) as u8),
    ((4) as u8),
    ((0) as u8),
    ((1) as u8),
    ((2) as u8),
    ((0x77) as u8),
];
pub static mut stbi_write_jpg_core_std_ac_chrominance_values: [u8; 162] = [
    ((0x00) as u8),
    ((0x01) as u8),
    ((0x02) as u8),
    ((0x03) as u8),
    ((0x11) as u8),
    ((0x04) as u8),
    ((0x05) as u8),
    ((0x21) as u8),
    ((0x31) as u8),
    ((0x06) as u8),
    ((0x12) as u8),
    ((0x41) as u8),
    ((0x51) as u8),
    ((0x07) as u8),
    ((0x61) as u8),
    ((0x71) as u8),
    ((0x13) as u8),
    ((0x22) as u8),
    ((0x32) as u8),
    ((0x81) as u8),
    ((0x08) as u8),
    ((0x14) as u8),
    ((0x42) as u8),
    ((0x91) as u8),
    ((0xa1) as u8),
    ((0xb1) as u8),
    ((0xc1) as u8),
    ((0x09) as u8),
    ((0x23) as u8),
    ((0x33) as u8),
    ((0x52) as u8),
    ((0xf0) as u8),
    ((0x15) as u8),
    ((0x62) as u8),
    ((0x72) as u8),
    ((0xd1) as u8),
    ((0x0a) as u8),
    ((0x16) as u8),
    ((0x24) as u8),
    ((0x34) as u8),
    ((0xe1) as u8),
    ((0x25) as u8),
    ((0xf1) as u8),
    ((0x17) as u8),
    ((0x18) as u8),
    ((0x19) as u8),
    ((0x1a) as u8),
    ((0x26) as u8),
    ((0x27) as u8),
    ((0x28) as u8),
    ((0x29) as u8),
    ((0x2a) as u8),
    ((0x35) as u8),
    ((0x36) as u8),
    ((0x37) as u8),
    ((0x38) as u8),
    ((0x39) as u8),
    ((0x3a) as u8),
    ((0x43) as u8),
    ((0x44) as u8),
    ((0x45) as u8),
    ((0x46) as u8),
    ((0x47) as u8),
    ((0x48) as u8),
    ((0x49) as u8),
    ((0x4a) as u8),
    ((0x53) as u8),
    ((0x54) as u8),
    ((0x55) as u8),
    ((0x56) as u8),
    ((0x57) as u8),
    ((0x58) as u8),
    ((0x59) as u8),
    ((0x5a) as u8),
    ((0x63) as u8),
    ((0x64) as u8),
    ((0x65) as u8),
    ((0x66) as u8),
    ((0x67) as u8),
    ((0x68) as u8),
    ((0x69) as u8),
    ((0x6a) as u8),
    ((0x73) as u8),
    ((0x74) as u8),
    ((0x75) as u8),
    ((0x76) as u8),
    ((0x77) as u8),
    ((0x78) as u8),
    ((0x79) as u8),
    ((0x7a) as u8),
    ((0x82) as u8),
    ((0x83) as u8),
    ((0x84) as u8),
    ((0x85) as u8),
    ((0x86) as u8),
    ((0x87) as u8),
    ((0x88) as u8),
    ((0x89) as u8),
    ((0x8a) as u8),
    ((0x92) as u8),
    ((0x93) as u8),
    ((0x94) as u8),
    ((0x95) as u8),
    ((0x96) as u8),
    ((0x97) as u8),
    ((0x98) as u8),
    ((0x99) as u8),
    ((0x9a) as u8),
    ((0xa2) as u8),
    ((0xa3) as u8),
    ((0xa4) as u8),
    ((0xa5) as u8),
    ((0xa6) as u8),
    ((0xa7) as u8),
    ((0xa8) as u8),
    ((0xa9) as u8),
    ((0xaa) as u8),
    ((0xb2) as u8),
    ((0xb3) as u8),
    ((0xb4) as u8),
    ((0xb5) as u8),
    ((0xb6) as u8),
    ((0xb7) as u8),
    ((0xb8) as u8),
    ((0xb9) as u8),
    ((0xba) as u8),
    ((0xc2) as u8),
    ((0xc3) as u8),
    ((0xc4) as u8),
    ((0xc5) as u8),
    ((0xc6) as u8),
    ((0xc7) as u8),
    ((0xc8) as u8),
    ((0xc9) as u8),
    ((0xca) as u8),
    ((0xd2) as u8),
    ((0xd3) as u8),
    ((0xd4) as u8),
    ((0xd5) as u8),
    ((0xd6) as u8),
    ((0xd7) as u8),
    ((0xd8) as u8),
    ((0xd9) as u8),
    ((0xda) as u8),
    ((0xe2) as u8),
    ((0xe3) as u8),
    ((0xe4) as u8),
    ((0xe5) as u8),
    ((0xe6) as u8),
    ((0xe7) as u8),
    ((0xe8) as u8),
    ((0xe9) as u8),
    ((0xea) as u8),
    ((0xf2) as u8),
    ((0xf3) as u8),
    ((0xf4) as u8),
    ((0xf5) as u8),
    ((0xf6) as u8),
    ((0xf7) as u8),
    ((0xf8) as u8),
    ((0xf9) as u8),
    ((0xfa) as u8),
];
pub static mut stbi_write_jpg_core_std_ac_luminance_nrcodes: [u8; 17] = [
    ((0) as u8),
    ((0) as u8),
    ((2) as u8),
    ((1) as u8),
    ((3) as u8),
    ((3) as u8),
    ((2) as u8),
    ((4) as u8),
    ((3) as u8),
    ((5) as u8),
    ((5) as u8),
    ((4) as u8),
    ((4) as u8),
    ((0) as u8),
    ((0) as u8),
    ((1) as u8),
    ((0x7d) as u8),
];
pub static mut stbi_write_jpg_core_std_ac_luminance_values: [u8; 162] = [
    ((0x01) as u8),
    ((0x02) as u8),
    ((0x03) as u8),
    ((0x00) as u8),
    ((0x04) as u8),
    ((0x11) as u8),
    ((0x05) as u8),
    ((0x12) as u8),
    ((0x21) as u8),
    ((0x31) as u8),
    ((0x41) as u8),
    ((0x06) as u8),
    ((0x13) as u8),
    ((0x51) as u8),
    ((0x61) as u8),
    ((0x07) as u8),
    ((0x22) as u8),
    ((0x71) as u8),
    ((0x14) as u8),
    ((0x32) as u8),
    ((0x81) as u8),
    ((0x91) as u8),
    ((0xa1) as u8),
    ((0x08) as u8),
    ((0x23) as u8),
    ((0x42) as u8),
    ((0xb1) as u8),
    ((0xc1) as u8),
    ((0x15) as u8),
    ((0x52) as u8),
    ((0xd1) as u8),
    ((0xf0) as u8),
    ((0x24) as u8),
    ((0x33) as u8),
    ((0x62) as u8),
    ((0x72) as u8),
    ((0x82) as u8),
    ((0x09) as u8),
    ((0x0a) as u8),
    ((0x16) as u8),
    ((0x17) as u8),
    ((0x18) as u8),
    ((0x19) as u8),
    ((0x1a) as u8),
    ((0x25) as u8),
    ((0x26) as u8),
    ((0x27) as u8),
    ((0x28) as u8),
    ((0x29) as u8),
    ((0x2a) as u8),
    ((0x34) as u8),
    ((0x35) as u8),
    ((0x36) as u8),
    ((0x37) as u8),
    ((0x38) as u8),
    ((0x39) as u8),
    ((0x3a) as u8),
    ((0x43) as u8),
    ((0x44) as u8),
    ((0x45) as u8),
    ((0x46) as u8),
    ((0x47) as u8),
    ((0x48) as u8),
    ((0x49) as u8),
    ((0x4a) as u8),
    ((0x53) as u8),
    ((0x54) as u8),
    ((0x55) as u8),
    ((0x56) as u8),
    ((0x57) as u8),
    ((0x58) as u8),
    ((0x59) as u8),
    ((0x5a) as u8),
    ((0x63) as u8),
    ((0x64) as u8),
    ((0x65) as u8),
    ((0x66) as u8),
    ((0x67) as u8),
    ((0x68) as u8),
    ((0x69) as u8),
    ((0x6a) as u8),
    ((0x73) as u8),
    ((0x74) as u8),
    ((0x75) as u8),
    ((0x76) as u8),
    ((0x77) as u8),
    ((0x78) as u8),
    ((0x79) as u8),
    ((0x7a) as u8),
    ((0x83) as u8),
    ((0x84) as u8),
    ((0x85) as u8),
    ((0x86) as u8),
    ((0x87) as u8),
    ((0x88) as u8),
    ((0x89) as u8),
    ((0x8a) as u8),
    ((0x92) as u8),
    ((0x93) as u8),
    ((0x94) as u8),
    ((0x95) as u8),
    ((0x96) as u8),
    ((0x97) as u8),
    ((0x98) as u8),
    ((0x99) as u8),
    ((0x9a) as u8),
    ((0xa2) as u8),
    ((0xa3) as u8),
    ((0xa4) as u8),
    ((0xa5) as u8),
    ((0xa6) as u8),
    ((0xa7) as u8),
    ((0xa8) as u8),
    ((0xa9) as u8),
    ((0xaa) as u8),
    ((0xb2) as u8),
    ((0xb3) as u8),
    ((0xb4) as u8),
    ((0xb5) as u8),
    ((0xb6) as u8),
    ((0xb7) as u8),
    ((0xb8) as u8),
    ((0xb9) as u8),
    ((0xba) as u8),
    ((0xc2) as u8),
    ((0xc3) as u8),
    ((0xc4) as u8),
    ((0xc5) as u8),
    ((0xc6) as u8),
    ((0xc7) as u8),
    ((0xc8) as u8),
    ((0xc9) as u8),
    ((0xca) as u8),
    ((0xd2) as u8),
    ((0xd3) as u8),
    ((0xd4) as u8),
    ((0xd5) as u8),
    ((0xd6) as u8),
    ((0xd7) as u8),
    ((0xd8) as u8),
    ((0xd9) as u8),
    ((0xda) as u8),
    ((0xe1) as u8),
    ((0xe2) as u8),
    ((0xe3) as u8),
    ((0xe4) as u8),
    ((0xe5) as u8),
    ((0xe6) as u8),
    ((0xe7) as u8),
    ((0xe8) as u8),
    ((0xe9) as u8),
    ((0xea) as u8),
    ((0xf1) as u8),
    ((0xf2) as u8),
    ((0xf3) as u8),
    ((0xf4) as u8),
    ((0xf5) as u8),
    ((0xf6) as u8),
    ((0xf7) as u8),
    ((0xf8) as u8),
    ((0xf9) as u8),
    ((0xfa) as u8),
];
pub static mut stbi_write_jpg_core_std_dc_chrominance_nrcodes: [u8; 17] = [
    ((0) as u8),
    ((0) as u8),
    ((3) as u8),
    ((1) as u8),
    ((1) as u8),
    ((1) as u8),
    ((1) as u8),
    ((1) as u8),
    ((1) as u8),
    ((1) as u8),
    ((1) as u8),
    ((1) as u8),
    ((0) as u8),
    ((0) as u8),
    ((0) as u8),
    ((0) as u8),
    ((0) as u8),
];
pub static mut stbi_write_jpg_core_std_dc_chrominance_values: [u8; 12] = [
    ((0) as u8),
    ((1) as u8),
    ((2) as u8),
    ((3) as u8),
    ((4) as u8),
    ((5) as u8),
    ((6) as u8),
    ((7) as u8),
    ((8) as u8),
    ((9) as u8),
    ((10) as u8),
    ((11) as u8),
];
pub static mut stbi_write_jpg_core_std_dc_luminance_nrcodes: [u8; 17] = [
    ((0) as u8),
    ((0) as u8),
    ((1) as u8),
    ((5) as u8),
    ((1) as u8),
    ((1) as u8),
    ((1) as u8),
    ((1) as u8),
    ((1) as u8),
    ((1) as u8),
    ((0) as u8),
    ((0) as u8),
    ((0) as u8),
    ((0) as u8),
    ((0) as u8),
    ((0) as u8),
    ((0) as u8),
];
pub static mut stbi_write_jpg_core_std_dc_luminance_values: [u8; 12] = [
    ((0) as u8),
    ((1) as u8),
    ((2) as u8),
    ((3) as u8),
    ((4) as u8),
    ((5) as u8),
    ((6) as u8),
    ((7) as u8),
    ((8) as u8),
    ((9) as u8),
    ((10) as u8),
    ((11) as u8),
];
pub static mut stbi_write_jpg_core_UVAC_HT: [[u16; 2]; 256] = [
    [((0) as u16), ((2) as u16)],
    [((1) as u16), ((2) as u16)],
    [((4) as u16), ((3) as u16)],
    [((10) as u16), ((4) as u16)],
    [((24) as u16), ((5) as u16)],
    [((25) as u16), ((5) as u16)],
    [((56) as u16), ((6) as u16)],
    [((120) as u16), ((7) as u16)],
    [((500) as u16), ((9) as u16)],
    [((1014) as u16), ((10) as u16)],
    [((4084) as u16), ((12) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((11) as u16), ((4) as u16)],
    [((57) as u16), ((6) as u16)],
    [((246) as u16), ((8) as u16)],
    [((501) as u16), ((9) as u16)],
    [((2038) as u16), ((11) as u16)],
    [((4085) as u16), ((12) as u16)],
    [((65416) as u16), ((16) as u16)],
    [((65417) as u16), ((16) as u16)],
    [((65418) as u16), ((16) as u16)],
    [((65419) as u16), ((16) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((26) as u16), ((5) as u16)],
    [((247) as u16), ((8) as u16)],
    [((1015) as u16), ((10) as u16)],
    [((4086) as u16), ((12) as u16)],
    [((32706) as u16), ((15) as u16)],
    [((65420) as u16), ((16) as u16)],
    [((65421) as u16), ((16) as u16)],
    [((65422) as u16), ((16) as u16)],
    [((65423) as u16), ((16) as u16)],
    [((65424) as u16), ((16) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((27) as u16), ((5) as u16)],
    [((248) as u16), ((8) as u16)],
    [((1016) as u16), ((10) as u16)],
    [((4087) as u16), ((12) as u16)],
    [((65425) as u16), ((16) as u16)],
    [((65426) as u16), ((16) as u16)],
    [((65427) as u16), ((16) as u16)],
    [((65428) as u16), ((16) as u16)],
    [((65429) as u16), ((16) as u16)],
    [((65430) as u16), ((16) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((58) as u16), ((6) as u16)],
    [((502) as u16), ((9) as u16)],
    [((65431) as u16), ((16) as u16)],
    [((65432) as u16), ((16) as u16)],
    [((65433) as u16), ((16) as u16)],
    [((65434) as u16), ((16) as u16)],
    [((65435) as u16), ((16) as u16)],
    [((65436) as u16), ((16) as u16)],
    [((65437) as u16), ((16) as u16)],
    [((65438) as u16), ((16) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((59) as u16), ((6) as u16)],
    [((1017) as u16), ((10) as u16)],
    [((65439) as u16), ((16) as u16)],
    [((65440) as u16), ((16) as u16)],
    [((65441) as u16), ((16) as u16)],
    [((65442) as u16), ((16) as u16)],
    [((65443) as u16), ((16) as u16)],
    [((65444) as u16), ((16) as u16)],
    [((65445) as u16), ((16) as u16)],
    [((65446) as u16), ((16) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((121) as u16), ((7) as u16)],
    [((2039) as u16), ((11) as u16)],
    [((65447) as u16), ((16) as u16)],
    [((65448) as u16), ((16) as u16)],
    [((65449) as u16), ((16) as u16)],
    [((65450) as u16), ((16) as u16)],
    [((65451) as u16), ((16) as u16)],
    [((65452) as u16), ((16) as u16)],
    [((65453) as u16), ((16) as u16)],
    [((65454) as u16), ((16) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((122) as u16), ((7) as u16)],
    [((2040) as u16), ((11) as u16)],
    [((65455) as u16), ((16) as u16)],
    [((65456) as u16), ((16) as u16)],
    [((65457) as u16), ((16) as u16)],
    [((65458) as u16), ((16) as u16)],
    [((65459) as u16), ((16) as u16)],
    [((65460) as u16), ((16) as u16)],
    [((65461) as u16), ((16) as u16)],
    [((65462) as u16), ((16) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((249) as u16), ((8) as u16)],
    [((65463) as u16), ((16) as u16)],
    [((65464) as u16), ((16) as u16)],
    [((65465) as u16), ((16) as u16)],
    [((65466) as u16), ((16) as u16)],
    [((65467) as u16), ((16) as u16)],
    [((65468) as u16), ((16) as u16)],
    [((65469) as u16), ((16) as u16)],
    [((65470) as u16), ((16) as u16)],
    [((65471) as u16), ((16) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((503) as u16), ((9) as u16)],
    [((65472) as u16), ((16) as u16)],
    [((65473) as u16), ((16) as u16)],
    [((65474) as u16), ((16) as u16)],
    [((65475) as u16), ((16) as u16)],
    [((65476) as u16), ((16) as u16)],
    [((65477) as u16), ((16) as u16)],
    [((65478) as u16), ((16) as u16)],
    [((65479) as u16), ((16) as u16)],
    [((65480) as u16), ((16) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((504) as u16), ((9) as u16)],
    [((65481) as u16), ((16) as u16)],
    [((65482) as u16), ((16) as u16)],
    [((65483) as u16), ((16) as u16)],
    [((65484) as u16), ((16) as u16)],
    [((65485) as u16), ((16) as u16)],
    [((65486) as u16), ((16) as u16)],
    [((65487) as u16), ((16) as u16)],
    [((65488) as u16), ((16) as u16)],
    [((65489) as u16), ((16) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((505) as u16), ((9) as u16)],
    [((65490) as u16), ((16) as u16)],
    [((65491) as u16), ((16) as u16)],
    [((65492) as u16), ((16) as u16)],
    [((65493) as u16), ((16) as u16)],
    [((65494) as u16), ((16) as u16)],
    [((65495) as u16), ((16) as u16)],
    [((65496) as u16), ((16) as u16)],
    [((65497) as u16), ((16) as u16)],
    [((65498) as u16), ((16) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((506) as u16), ((9) as u16)],
    [((65499) as u16), ((16) as u16)],
    [((65500) as u16), ((16) as u16)],
    [((65501) as u16), ((16) as u16)],
    [((65502) as u16), ((16) as u16)],
    [((65503) as u16), ((16) as u16)],
    [((65504) as u16), ((16) as u16)],
    [((65505) as u16), ((16) as u16)],
    [((65506) as u16), ((16) as u16)],
    [((65507) as u16), ((16) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((2041) as u16), ((11) as u16)],
    [((65508) as u16), ((16) as u16)],
    [((65509) as u16), ((16) as u16)],
    [((65510) as u16), ((16) as u16)],
    [((65511) as u16), ((16) as u16)],
    [((65512) as u16), ((16) as u16)],
    [((65513) as u16), ((16) as u16)],
    [((65514) as u16), ((16) as u16)],
    [((65515) as u16), ((16) as u16)],
    [((65516) as u16), ((16) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((16352) as u16), ((14) as u16)],
    [((65517) as u16), ((16) as u16)],
    [((65518) as u16), ((16) as u16)],
    [((65519) as u16), ((16) as u16)],
    [((65520) as u16), ((16) as u16)],
    [((65521) as u16), ((16) as u16)],
    [((65522) as u16), ((16) as u16)],
    [((65523) as u16), ((16) as u16)],
    [((65524) as u16), ((16) as u16)],
    [((65525) as u16), ((16) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((1018) as u16), ((10) as u16)],
    [((32707) as u16), ((15) as u16)],
    [((65526) as u16), ((16) as u16)],
    [((65527) as u16), ((16) as u16)],
    [((65528) as u16), ((16) as u16)],
    [((65529) as u16), ((16) as u16)],
    [((65530) as u16), ((16) as u16)],
    [((65531) as u16), ((16) as u16)],
    [((65532) as u16), ((16) as u16)],
    [((65533) as u16), ((16) as u16)],
    [((65534) as u16), ((16) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
];
pub static mut stbi_write_jpg_core_UVDC_HT: [[u16; 2]; 12] = [
    [((0) as u16), ((2) as u16)],
    [((1) as u16), ((2) as u16)],
    [((2) as u16), ((2) as u16)],
    [((6) as u16), ((3) as u16)],
    [((14) as u16), ((4) as u16)],
    [((30) as u16), ((5) as u16)],
    [((62) as u16), ((6) as u16)],
    [((126) as u16), ((7) as u16)],
    [((254) as u16), ((8) as u16)],
    [((510) as u16), ((9) as u16)],
    [((1022) as u16), ((10) as u16)],
    [((2046) as u16), ((11) as u16)],
];
pub static mut stbi_write_jpg_core_UVQT: [i32; 64] = [
    17, 18, 24, 47, 99, 99, 99, 99, 18, 21, 26, 66, 99, 99, 99, 99, 24, 26, 56, 99, 99, 99, 99, 99,
    47, 66, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99,
    99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99,
];
pub static mut stbi_write_jpg_core_YAC_HT: [[u16; 2]; 256] = [
    [((10) as u16), ((4) as u16)],
    [((0) as u16), ((2) as u16)],
    [((1) as u16), ((2) as u16)],
    [((4) as u16), ((3) as u16)],
    [((11) as u16), ((4) as u16)],
    [((26) as u16), ((5) as u16)],
    [((120) as u16), ((7) as u16)],
    [((248) as u16), ((8) as u16)],
    [((1014) as u16), ((10) as u16)],
    [((65410) as u16), ((16) as u16)],
    [((65411) as u16), ((16) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((12) as u16), ((4) as u16)],
    [((27) as u16), ((5) as u16)],
    [((121) as u16), ((7) as u16)],
    [((502) as u16), ((9) as u16)],
    [((2038) as u16), ((11) as u16)],
    [((65412) as u16), ((16) as u16)],
    [((65413) as u16), ((16) as u16)],
    [((65414) as u16), ((16) as u16)],
    [((65415) as u16), ((16) as u16)],
    [((65416) as u16), ((16) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((28) as u16), ((5) as u16)],
    [((249) as u16), ((8) as u16)],
    [((1015) as u16), ((10) as u16)],
    [((4084) as u16), ((12) as u16)],
    [((65417) as u16), ((16) as u16)],
    [((65418) as u16), ((16) as u16)],
    [((65419) as u16), ((16) as u16)],
    [((65420) as u16), ((16) as u16)],
    [((65421) as u16), ((16) as u16)],
    [((65422) as u16), ((16) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((58) as u16), ((6) as u16)],
    [((503) as u16), ((9) as u16)],
    [((4085) as u16), ((12) as u16)],
    [((65423) as u16), ((16) as u16)],
    [((65424) as u16), ((16) as u16)],
    [((65425) as u16), ((16) as u16)],
    [((65426) as u16), ((16) as u16)],
    [((65427) as u16), ((16) as u16)],
    [((65428) as u16), ((16) as u16)],
    [((65429) as u16), ((16) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((59) as u16), ((6) as u16)],
    [((1016) as u16), ((10) as u16)],
    [((65430) as u16), ((16) as u16)],
    [((65431) as u16), ((16) as u16)],
    [((65432) as u16), ((16) as u16)],
    [((65433) as u16), ((16) as u16)],
    [((65434) as u16), ((16) as u16)],
    [((65435) as u16), ((16) as u16)],
    [((65436) as u16), ((16) as u16)],
    [((65437) as u16), ((16) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((122) as u16), ((7) as u16)],
    [((2039) as u16), ((11) as u16)],
    [((65438) as u16), ((16) as u16)],
    [((65439) as u16), ((16) as u16)],
    [((65440) as u16), ((16) as u16)],
    [((65441) as u16), ((16) as u16)],
    [((65442) as u16), ((16) as u16)],
    [((65443) as u16), ((16) as u16)],
    [((65444) as u16), ((16) as u16)],
    [((65445) as u16), ((16) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((123) as u16), ((7) as u16)],
    [((4086) as u16), ((12) as u16)],
    [((65446) as u16), ((16) as u16)],
    [((65447) as u16), ((16) as u16)],
    [((65448) as u16), ((16) as u16)],
    [((65449) as u16), ((16) as u16)],
    [((65450) as u16), ((16) as u16)],
    [((65451) as u16), ((16) as u16)],
    [((65452) as u16), ((16) as u16)],
    [((65453) as u16), ((16) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((250) as u16), ((8) as u16)],
    [((4087) as u16), ((12) as u16)],
    [((65454) as u16), ((16) as u16)],
    [((65455) as u16), ((16) as u16)],
    [((65456) as u16), ((16) as u16)],
    [((65457) as u16), ((16) as u16)],
    [((65458) as u16), ((16) as u16)],
    [((65459) as u16), ((16) as u16)],
    [((65460) as u16), ((16) as u16)],
    [((65461) as u16), ((16) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((504) as u16), ((9) as u16)],
    [((32704) as u16), ((15) as u16)],
    [((65462) as u16), ((16) as u16)],
    [((65463) as u16), ((16) as u16)],
    [((65464) as u16), ((16) as u16)],
    [((65465) as u16), ((16) as u16)],
    [((65466) as u16), ((16) as u16)],
    [((65467) as u16), ((16) as u16)],
    [((65468) as u16), ((16) as u16)],
    [((65469) as u16), ((16) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((505) as u16), ((9) as u16)],
    [((65470) as u16), ((16) as u16)],
    [((65471) as u16), ((16) as u16)],
    [((65472) as u16), ((16) as u16)],
    [((65473) as u16), ((16) as u16)],
    [((65474) as u16), ((16) as u16)],
    [((65475) as u16), ((16) as u16)],
    [((65476) as u16), ((16) as u16)],
    [((65477) as u16), ((16) as u16)],
    [((65478) as u16), ((16) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((506) as u16), ((9) as u16)],
    [((65479) as u16), ((16) as u16)],
    [((65480) as u16), ((16) as u16)],
    [((65481) as u16), ((16) as u16)],
    [((65482) as u16), ((16) as u16)],
    [((65483) as u16), ((16) as u16)],
    [((65484) as u16), ((16) as u16)],
    [((65485) as u16), ((16) as u16)],
    [((65486) as u16), ((16) as u16)],
    [((65487) as u16), ((16) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((1017) as u16), ((10) as u16)],
    [((65488) as u16), ((16) as u16)],
    [((65489) as u16), ((16) as u16)],
    [((65490) as u16), ((16) as u16)],
    [((65491) as u16), ((16) as u16)],
    [((65492) as u16), ((16) as u16)],
    [((65493) as u16), ((16) as u16)],
    [((65494) as u16), ((16) as u16)],
    [((65495) as u16), ((16) as u16)],
    [((65496) as u16), ((16) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((1018) as u16), ((10) as u16)],
    [((65497) as u16), ((16) as u16)],
    [((65498) as u16), ((16) as u16)],
    [((65499) as u16), ((16) as u16)],
    [((65500) as u16), ((16) as u16)],
    [((65501) as u16), ((16) as u16)],
    [((65502) as u16), ((16) as u16)],
    [((65503) as u16), ((16) as u16)],
    [((65504) as u16), ((16) as u16)],
    [((65505) as u16), ((16) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((2040) as u16), ((11) as u16)],
    [((65506) as u16), ((16) as u16)],
    [((65507) as u16), ((16) as u16)],
    [((65508) as u16), ((16) as u16)],
    [((65509) as u16), ((16) as u16)],
    [((65510) as u16), ((16) as u16)],
    [((65511) as u16), ((16) as u16)],
    [((65512) as u16), ((16) as u16)],
    [((65513) as u16), ((16) as u16)],
    [((65514) as u16), ((16) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((65515) as u16), ((16) as u16)],
    [((65516) as u16), ((16) as u16)],
    [((65517) as u16), ((16) as u16)],
    [((65518) as u16), ((16) as u16)],
    [((65519) as u16), ((16) as u16)],
    [((65520) as u16), ((16) as u16)],
    [((65521) as u16), ((16) as u16)],
    [((65522) as u16), ((16) as u16)],
    [((65523) as u16), ((16) as u16)],
    [((65524) as u16), ((16) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((2041) as u16), ((11) as u16)],
    [((65525) as u16), ((16) as u16)],
    [((65526) as u16), ((16) as u16)],
    [((65527) as u16), ((16) as u16)],
    [((65528) as u16), ((16) as u16)],
    [((65529) as u16), ((16) as u16)],
    [((65530) as u16), ((16) as u16)],
    [((65531) as u16), ((16) as u16)],
    [((65532) as u16), ((16) as u16)],
    [((65533) as u16), ((16) as u16)],
    [((65534) as u16), ((16) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
    [((0) as u16), ((0) as u16)],
];
pub static mut stbi_write_jpg_core_YDC_HT: [[u16; 2]; 12] = [
    [((0) as u16), ((2) as u16)],
    [((2) as u16), ((3) as u16)],
    [((3) as u16), ((3) as u16)],
    [((4) as u16), ((3) as u16)],
    [((5) as u16), ((3) as u16)],
    [((6) as u16), ((3) as u16)],
    [((14) as u16), ((4) as u16)],
    [((30) as u16), ((5) as u16)],
    [((62) as u16), ((6) as u16)],
    [((126) as u16), ((7) as u16)],
    [((254) as u16), ((8) as u16)],
    [((510) as u16), ((9) as u16)],
];
pub static mut stbi_write_jpg_core_YQT: [i32; 64] = [
    16, 11, 10, 16, 24, 40, 51, 61, 12, 12, 14, 19, 26, 58, 60, 55, 14, 13, 16, 24, 40, 57, 69, 56,
    14, 17, 22, 29, 51, 87, 80, 62, 18, 22, 37, 56, 68, 109, 103, 77, 24, 35, 55, 64, 81, 104, 113,
    92, 49, 64, 78, 87, 103, 121, 120, 101, 72, 92, 95, 98, 112, 100, 103, 99,
];
pub static mut stbi_zlib_compress_distc: [u16; 31] = [
    ((1) as u16),
    ((2) as u16),
    ((3) as u16),
    ((4) as u16),
    ((5) as u16),
    ((7) as u16),
    ((9) as u16),
    ((13) as u16),
    ((17) as u16),
    ((25) as u16),
    ((33) as u16),
    ((49) as u16),
    ((65) as u16),
    ((97) as u16),
    ((129) as u16),
    ((193) as u16),
    ((257) as u16),
    ((385) as u16),
    ((513) as u16),
    ((769) as u16),
    ((1025) as u16),
    ((1537) as u16),
    ((2049) as u16),
    ((3073) as u16),
    ((4097) as u16),
    ((6145) as u16),
    ((8193) as u16),
    ((12289) as u16),
    ((16385) as u16),
    ((24577) as u16),
    ((32768) as u16),
];
pub static mut stbi_zlib_compress_disteb: [u8; 30] = [
    ((0) as u8),
    ((0) as u8),
    ((0) as u8),
    ((0) as u8),
    ((1) as u8),
    ((1) as u8),
    ((2) as u8),
    ((2) as u8),
    ((3) as u8),
    ((3) as u8),
    ((4) as u8),
    ((4) as u8),
    ((5) as u8),
    ((5) as u8),
    ((6) as u8),
    ((6) as u8),
    ((7) as u8),
    ((7) as u8),
    ((8) as u8),
    ((8) as u8),
    ((9) as u8),
    ((9) as u8),
    ((10) as u8),
    ((10) as u8),
    ((11) as u8),
    ((11) as u8),
    ((12) as u8),
    ((12) as u8),
    ((13) as u8),
    ((13) as u8),
];
pub static mut stbi_zlib_compress_lengthc: [u16; 30] = [
    ((3) as u16),
    ((4) as u16),
    ((5) as u16),
    ((6) as u16),
    ((7) as u16),
    ((8) as u16),
    ((9) as u16),
    ((10) as u16),
    ((11) as u16),
    ((13) as u16),
    ((15) as u16),
    ((17) as u16),
    ((19) as u16),
    ((23) as u16),
    ((27) as u16),
    ((31) as u16),
    ((35) as u16),
    ((43) as u16),
    ((51) as u16),
    ((59) as u16),
    ((67) as u16),
    ((83) as u16),
    ((99) as u16),
    ((115) as u16),
    ((131) as u16),
    ((163) as u16),
    ((195) as u16),
    ((227) as u16),
    ((258) as u16),
    ((259) as u16),
];
pub static mut stbi_zlib_compress_lengtheb: [u8; 29] = [
    ((0) as u8),
    ((0) as u8),
    ((0) as u8),
    ((0) as u8),
    ((0) as u8),
    ((0) as u8),
    ((0) as u8),
    ((0) as u8),
    ((1) as u8),
    ((1) as u8),
    ((1) as u8),
    ((1) as u8),
    ((2) as u8),
    ((2) as u8),
    ((2) as u8),
    ((2) as u8),
    ((3) as u8),
    ((3) as u8),
    ((3) as u8),
    ((3) as u8),
    ((4) as u8),
    ((4) as u8),
    ((4) as u8),
    ((4) as u8),
    ((5) as u8),
    ((5) as u8),
    ((5) as u8),
    ((5) as u8),
    ((0) as u8),
];
pub static mut stbiw__crc32_crc_table: [u32; 256] = [
    ((0x00000000) as u32),
    ((0x77073096) as u32),
    0xEE0E612C,
    0x990951BA,
    ((0x076DC419) as u32),
    ((0x706AF48F) as u32),
    0xE963A535,
    0x9E6495A3,
    ((0x0eDB8832) as u32),
    ((0x79DCB8A4) as u32),
    0xE0D5E91E,
    0x97D2D988,
    ((0x09B64C2B) as u32),
    ((0x7EB17CBD) as u32),
    0xE7B82D07,
    0x90BF1D91,
    ((0x1DB71064) as u32),
    ((0x6AB020F2) as u32),
    0xF3B97148,
    0x84BE41DE,
    ((0x1ADAD47D) as u32),
    ((0x6DDDE4EB) as u32),
    0xF4D4B551,
    0x83D385C7,
    ((0x136C9856) as u32),
    ((0x646BA8C0) as u32),
    0xFD62F97A,
    0x8A65C9EC,
    ((0x14015C4F) as u32),
    ((0x63066CD9) as u32),
    0xFA0F3D63,
    0x8D080DF5,
    ((0x3B6E20C8) as u32),
    ((0x4C69105E) as u32),
    0xD56041E4,
    0xA2677172,
    ((0x3C03E4D1) as u32),
    ((0x4B04D447) as u32),
    0xD20D85FD,
    0xA50AB56B,
    ((0x35B5A8FA) as u32),
    ((0x42B2986C) as u32),
    0xDBBBC9D6,
    0xACBCF940,
    ((0x32D86CE3) as u32),
    ((0x45DF5C75) as u32),
    0xDCD60DCF,
    0xABD13D59,
    ((0x26D930AC) as u32),
    ((0x51DE003A) as u32),
    0xC8D75180,
    0xBFD06116,
    ((0x21B4F4B5) as u32),
    ((0x56B3C423) as u32),
    0xCFBA9599,
    0xB8BDA50F,
    ((0x2802B89E) as u32),
    ((0x5F058808) as u32),
    0xC60CD9B2,
    0xB10BE924,
    ((0x2F6F7C87) as u32),
    ((0x58684C11) as u32),
    0xC1611DAB,
    0xB6662D3D,
    ((0x76DC4190) as u32),
    ((0x01DB7106) as u32),
    0x98D220BC,
    0xEFD5102A,
    ((0x71B18589) as u32),
    ((0x06B6B51F) as u32),
    0x9FBFE4A5,
    0xE8B8D433,
    ((0x7807C9A2) as u32),
    ((0x0F00F934) as u32),
    0x9609A88E,
    0xE10E9818,
    ((0x7F6A0DBB) as u32),
    ((0x086D3D2D) as u32),
    0x91646C97,
    0xE6635C01,
    ((0x6B6B51F4) as u32),
    ((0x1C6C6162) as u32),
    0x856530D8,
    0xF262004E,
    ((0x6C0695ED) as u32),
    ((0x1B01A57B) as u32),
    0x8208F4C1,
    0xF50FC457,
    ((0x65B0D9C6) as u32),
    ((0x12B7E950) as u32),
    0x8BBEB8EA,
    0xFCB9887C,
    ((0x62DD1DDF) as u32),
    ((0x15DA2D49) as u32),
    0x8CD37CF3,
    0xFBD44C65,
    ((0x4DB26158) as u32),
    ((0x3AB551CE) as u32),
    0xA3BC0074,
    0xD4BB30E2,
    ((0x4ADFA541) as u32),
    ((0x3DD895D7) as u32),
    0xA4D1C46D,
    0xD3D6F4FB,
    ((0x4369E96A) as u32),
    ((0x346ED9FC) as u32),
    0xAD678846,
    0xDA60B8D0,
    ((0x44042D73) as u32),
    ((0x33031DE5) as u32),
    0xAA0A4C5F,
    0xDD0D7CC9,
    ((0x5005713C) as u32),
    ((0x270241AA) as u32),
    0xBE0B1010,
    0xC90C2086,
    ((0x5768B525) as u32),
    ((0x206F85B3) as u32),
    0xB966D409,
    0xCE61E49F,
    ((0x5EDEF90E) as u32),
    ((0x29D9C998) as u32),
    0xB0D09822,
    0xC7D7A8B4,
    ((0x59B33D17) as u32),
    ((0x2EB40D81) as u32),
    0xB7BD5C3B,
    0xC0BA6CAD,
    0xEDB88320,
    0x9ABFB3B6,
    ((0x03B6E20C) as u32),
    ((0x74B1D29A) as u32),
    0xEAD54739,
    0x9DD277AF,
    ((0x04DB2615) as u32),
    ((0x73DC1683) as u32),
    0xE3630B12,
    0x94643B84,
    ((0x0D6D6A3E) as u32),
    ((0x7A6A5AA8) as u32),
    0xE40ECF0B,
    0x9309FF9D,
    ((0x0A00AE27) as u32),
    ((0x7D079EB1) as u32),
    0xF00F9344,
    0x8708A3D2,
    ((0x1E01F268) as u32),
    ((0x6906C2FE) as u32),
    0xF762575D,
    0x806567CB,
    ((0x196C3671) as u32),
    ((0x6E6B06E7) as u32),
    0xFED41B76,
    0x89D32BE0,
    ((0x10DA7A5A) as u32),
    ((0x67DD4ACC) as u32),
    0xF9B9DF6F,
    0x8EBEEFF9,
    ((0x17B7BE43) as u32),
    ((0x60B08ED5) as u32),
    0xD6D6A3E8,
    0xA1D1937E,
    ((0x38D8C2C4) as u32),
    ((0x4FDFF252) as u32),
    0xD1BB67F1,
    0xA6BC5767,
    ((0x3FB506DD) as u32),
    ((0x48B2364B) as u32),
    0xD80D2BDA,
    0xAF0A1B4C,
    ((0x36034AF6) as u32),
    ((0x41047A60) as u32),
    0xDF60EFC3,
    0xA867DF55,
    ((0x316E8EEF) as u32),
    ((0x4669BE79) as u32),
    0xCB61B38C,
    0xBC66831A,
    ((0x256FD2A0) as u32),
    ((0x5268E236) as u32),
    0xCC0C7795,
    0xBB0B4703,
    ((0x220216B9) as u32),
    ((0x5505262F) as u32),
    0xC5BA3BBE,
    0xB2BD0B28,
    ((0x2BB45A92) as u32),
    ((0x5CB36A04) as u32),
    0xC2D7FFA7,
    0xB5D0CF31,
    ((0x2CD99E8B) as u32),
    ((0x5BDEAE1D) as u32),
    0x9B64C2B0,
    0xEC63F226,
    ((0x756AA39C) as u32),
    ((0x026D930A) as u32),
    0x9C0906A9,
    0xEB0E363F,
    ((0x72076785) as u32),
    ((0x05005713) as u32),
    0x95BF4A82,
    0xE2B87A14,
    ((0x7BB12BAE) as u32),
    ((0x0CB61B38) as u32),
    0x92D28E9B,
    0xE5D5BE0D,
    ((0x7CDCEFB7) as u32),
    ((0x0BDBDF21) as u32),
    0x86D3D2D4,
    0xF1D4E242,
    ((0x68DDB3F8) as u32),
    ((0x1FDA836E) as u32),
    0x81BE16CD,
    0xF6B9265B,
    ((0x6FB077E1) as u32),
    ((0x18B74777) as u32),
    0x88085AE6,
    0xFF0F6A70,
    ((0x66063BCA) as u32),
    ((0x11010B5C) as u32),
    0x8F659EFF,
    0xF862AE69,
    ((0x616BFFD3) as u32),
    ((0x166CCF45) as u32),
    0xA00AE278,
    0xD70DD2EE,
    ((0x4E048354) as u32),
    ((0x3903B3C2) as u32),
    0xA7672661,
    0xD06016F7,
    ((0x4969474D) as u32),
    ((0x3E6E77DB) as u32),
    0xAED16A4A,
    0xD9D65ADC,
    ((0x40DF0B66) as u32),
    ((0x37D83BF0) as u32),
    0xA9BCAE53,
    0xDEBB9EC5,
    ((0x47B2CF7F) as u32),
    ((0x30B5FFE9) as u32),
    0xBDBDF21C,
    0xCABAC28A,
    ((0x53B39330) as u32),
    ((0x24B4A3A6) as u32),
    0xBAD03605,
    0xCDD70693,
    ((0x54DE5729) as u32),
    ((0x23D967BF) as u32),
    0xB3667A2E,
    0xC4614AB8,
    ((0x5D681B02) as u32),
    ((0x2A6F2B94) as u32),
    0xB40BBE37,
    0xC30C8EA1,
    ((0x5A05DF1B) as u32),
    ((0x2D02EF8D) as u32),
];
pub static mut stbiw__encode_png_line_firstmap: [i32; 5] = [0, 1, 0, 5, 6];
pub static mut stbiw__encode_png_line_mapping: [i32; 5] = [0, 1, 2, 3, 4];

pub struct stbi__write_context {
    pub func: fn(arg0: *mut u8, arg1: *mut u8, arg2: i32),
    pub context: *mut u8,
    pub buffer: [u8; 64],
    pub buf_used: i32,
}

impl stbi__write_context {
    fn new(mut func: fn(arg0: *mut u8, arg1: *mut u8, arg2: i32)) -> Self {
        stbi__write_context {
            func: func,
            context: std::ptr::null_mut(),
            buffer: [0; 64],
            buf_used: 0,
        }
    }
}

pub unsafe fn stbi__start_write_callbacks(mut s: *mut stbi__write_context, mut context: *mut u8) {
    (*s).context = context;
}

pub unsafe fn stbi_flip_vertically_on_write(mut flag: i32) {
    stbi__flip_vertically_on_write = ((flag) as i32);
}

pub unsafe fn stbi_write_jpg_core(
    mut s: &mut stbi__write_context,
    mut width: i32,
    mut height: i32,
    mut comp: i32,
    mut data: *const u8,
    mut quality: i32,
) -> i32 {
    let mut row: i32 = 0;
    let mut col: i32 = 0;
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut subsample: i32 = 0;
    let mut fdtbl_Y: [f32; 64] = [0.0f32; 64];
    let mut fdtbl_UV: [f32; 64] = [0.0f32; 64];
    let mut YTable: [u8; 64] = [0; 64];
    let mut UVTable: [u8; 64] = [0; 64];
    if data == std::ptr::null_mut() || width == 0 || height == 0 || comp > 4 || comp < 1 {
        return ((0) as i32);
    }
    quality = ((if (quality) != 0 { quality } else { 90 }) as i32);
    subsample = ((if quality <= 90 { 1 } else { 0 }) as i32);
    quality = ((if quality < 1 {
        1
    } else {
        if quality > 100 {
            100
        } else {
            quality
        }
    }) as i32);
    quality = ((if quality < 50 {
        5000 / quality
    } else {
        200 - quality * 2
    }) as i32);
    i = ((0) as i32);
    while (i < 64) {
        let mut uvti: i32 = 0;
        let mut yti: i32 = (stbi_write_jpg_core_YQT[(i) as usize] * quality + 50) / 100;
        YTable[(stbiw__jpg_ZigZag[(i) as usize]) as usize] = ((if yti < 1 {
            1
        } else {
            if yti > 255 {
                255
            } else {
                yti
            }
        }) as u8);
        uvti = (((stbi_write_jpg_core_UVQT[(i) as usize] * quality + 50) / 100) as i32);
        UVTable[(stbiw__jpg_ZigZag[(i) as usize]) as usize] = ((if uvti < 1 {
            1
        } else {
            if uvti > 255 {
                255
            } else {
                uvti
            }
        }) as u8);
        preInc(&mut i);
    }
    row = ((0) as i32);
    k = ((0) as i32);
    while (row < 8) {
        col = ((0) as i32);
        while (col < 8) {
            fdtbl_Y[(k) as usize] = ((((1) as f32)
                / ((((YTable[(stbiw__jpg_ZigZag[(k) as usize]) as usize]) as i32) as f32)
                    * stbi_write_jpg_core_aasf[(row) as usize]
                    * stbi_write_jpg_core_aasf[(col) as usize]))
                as f32);
            fdtbl_UV[(k) as usize] = ((((1) as f32)
                / ((((UVTable[(stbiw__jpg_ZigZag[(k) as usize]) as usize]) as i32) as f32)
                    * stbi_write_jpg_core_aasf[(row) as usize]
                    * stbi_write_jpg_core_aasf[(col) as usize]))
                as f32);
            preInc(&mut col);
            preInc(&mut k);
        }
        preInc(&mut row);
    }
    {
        let mut head1: [u8; 24] = [
            ((0xFF) as u8),
            ((0xC0) as u8),
            ((0) as u8),
            ((0x11) as u8),
            ((8) as u8),
            ((height >> 8) as u8),
            (((height) & 0xff) as u8),
            ((width >> 8) as u8),
            (((width) & 0xff) as u8),
            ((3) as u8),
            ((1) as u8),
            ((if (subsample) != 0 { 0x22 } else { 0x11 }) as u8),
            ((0) as u8),
            ((2) as u8),
            ((0x11) as u8),
            ((1) as u8),
            ((3) as u8),
            ((0x11) as u8),
            ((1) as u8),
            ((0xFF) as u8),
            ((0xC4) as u8),
            ((0x01) as u8),
            ((0xA2) as u8),
            ((0) as u8),
        ];
        ((s).func)(
            (*s).context,
            ((stbi_write_jpg_core_head0.as_mut_ptr()) as *mut u8),
            ((25 * std::mem::size_of::<u8>() as u64) as i32),
        );
        ((*s).func)(
            (*s).context,
            ((YTable.as_mut_ptr()) as *mut u8),
            ((64 * std::mem::size_of::<u8>() as u64) as i32),
        );
        stbiw__putc(s, ((1) as u8));
        ((*s).func)(
            (*s).context,
            ((UVTable.as_mut_ptr()) as *mut u8),
            ((64 * std::mem::size_of::<u8>() as u64) as i32),
        );
        ((*s).func)(
            (*s).context,
            ((head1.as_mut_ptr()) as *mut u8),
            ((24 * std::mem::size_of::<u8>() as u64) as i32),
        );
        ((*s).func)(
            (*s).context,
            ((stbi_write_jpg_core_std_dc_luminance_nrcodes.as_mut_ptr()).offset((1) as isize)),
            ((17 * std::mem::size_of::<u8>() as u64 - ((1) as u64)) as i32),
        );
        ((*s).func)(
            (*s).context,
            ((stbi_write_jpg_core_std_dc_luminance_values.as_mut_ptr()) as *mut u8),
            ((12 * std::mem::size_of::<u8>() as u64) as i32),
        );
        stbiw__putc(s, ((0x10) as u8));
        ((*s).func)(
            (*s).context,
            ((stbi_write_jpg_core_std_ac_luminance_nrcodes.as_mut_ptr()).offset((1) as isize)),
            ((17 * std::mem::size_of::<u8>() as u64 - ((1) as u64)) as i32),
        );
        ((*s).func)(
            (*s).context,
            ((stbi_write_jpg_core_std_ac_luminance_values.as_mut_ptr()) as *mut u8),
            ((162 * std::mem::size_of::<u8>() as u64) as i32),
        );
        stbiw__putc(s, ((1) as u8));
        ((*s).func)(
            (*s).context,
            ((stbi_write_jpg_core_std_dc_chrominance_nrcodes.as_mut_ptr()).offset((1) as isize)),
            ((17 * std::mem::size_of::<u8>() as u64 - ((1) as u64)) as i32),
        );
        ((*s).func)(
            (*s).context,
            ((stbi_write_jpg_core_std_dc_chrominance_values.as_mut_ptr()) as *mut u8),
            ((12 * std::mem::size_of::<u8>() as u64) as i32),
        );
        stbiw__putc(s, ((0x11) as u8));
        ((*s).func)(
            (*s).context,
            ((stbi_write_jpg_core_std_ac_chrominance_nrcodes.as_mut_ptr()).offset((1) as isize)),
            ((17 * std::mem::size_of::<u8>() as u64 - ((1) as u64)) as i32),
        );
        ((*s).func)(
            (*s).context,
            ((stbi_write_jpg_core_std_ac_chrominance_values.as_mut_ptr()) as *mut u8),
            ((162 * std::mem::size_of::<u8>() as u64) as i32),
        );
        ((*s).func)(
            (*s).context,
            ((stbi_write_jpg_core_head2.as_mut_ptr()) as *mut u8),
            ((14 * std::mem::size_of::<u8>() as u64) as i32),
        );
    }
    {
        let mut DCY: i32 = 0;
        let mut DCU: i32 = 0;
        let mut DCV: i32 = 0;
        let mut bitBuf: i32 = 0;
        let mut bitCnt: i32 = 0;
        let mut ofsG: i32 = if comp > 2 { 1 } else { 0 };
        let mut ofsB: i32 = if comp > 2 { 2 } else { 0 };
        let mut dataR: *const u8 = data;
        let mut dataG: *const u8 = (dataR).offset((ofsG) as isize);
        let mut dataB: *const u8 = (dataR).offset((ofsB) as isize);
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut pos: i32 = 0;
        if (subsample) != 0 {
            y = ((0) as i32);
            while (y < height) {
                x = ((0) as i32);
                while (x < width) {
                    let mut Y: [f32; 256] = [0.0f32; 256];
                    let mut U: [f32; 256] = [0.0f32; 256];
                    let mut V: [f32; 256] = [0.0f32; 256];
                    row = ((y) as i32);
                    pos = ((0) as i32);
                    while (row < y + 16) {
                        let mut clamped_row: i32 = if (row < height) { row } else { height - 1 };
                        let mut base_p: i32 = (if (stbi__flip_vertically_on_write) != 0 {
                            (height - 1 - clamped_row)
                        } else {
                            clamped_row
                        }) * width
                            * comp;
                        col = ((x) as i32);
                        while (col < x + 16) {
                            let mut p: i32 =
                                base_p + (if (col < width) { col } else { (width - 1) }) * comp;
                            let mut r: f32 = ((*dataR.offset((p) as isize)) as f32);
                            let mut g: f32 = ((*dataG.offset((p) as isize)) as f32);
                            let mut b: f32 = ((*dataB.offset((p) as isize)) as f32);
                            Y[(pos) as usize] =
                                (0.29900f32 * r + 0.58700f32 * g + 0.11400f32 * b - ((128) as f32));
                            U[(pos) as usize] =
                                ((-0.16874f32 * r - 0.33126f32 * g + 0.50000f32 * b) as f32);
                            V[(pos) as usize] =
                                ((0.50000f32 * r - 0.41869f32 * g - 0.08131f32 * b) as f32);
                            preInc(&mut col);
                            preInc(&mut pos);
                        }
                        preInc(&mut row);
                    }
                    DCY = ((stbiw__jpg_processDU(
                        s,
                        ((&mut bitBuf) as *mut i32),
                        ((&mut bitCnt) as *mut i32),
                        (((Y.as_mut_ptr()).offset((0) as isize)) as *mut f32),
                        16,
                        ((fdtbl_Y.as_mut_ptr()) as *mut f32),
                        DCY,
                        &mut stbi_write_jpg_core_YDC_HT,
                        &mut stbi_write_jpg_core_YAC_HT,
                    )) as i32);
                    DCY = ((stbiw__jpg_processDU(
                        s,
                        ((&mut bitBuf) as *mut i32),
                        ((&mut bitCnt) as *mut i32),
                        (((Y.as_mut_ptr()).offset((8) as isize)) as *mut f32),
                        16,
                        ((fdtbl_Y.as_mut_ptr()) as *mut f32),
                        DCY,
                        &mut stbi_write_jpg_core_YDC_HT,
                        &mut stbi_write_jpg_core_YAC_HT,
                    )) as i32);
                    DCY = ((stbiw__jpg_processDU(
                        s,
                        ((&mut bitBuf) as *mut i32),
                        ((&mut bitCnt) as *mut i32),
                        (((Y.as_mut_ptr()).offset((128) as isize)) as *mut f32),
                        16,
                        ((fdtbl_Y.as_mut_ptr()) as *mut f32),
                        DCY,
                        &mut stbi_write_jpg_core_YDC_HT,
                        &mut stbi_write_jpg_core_YAC_HT,
                    )) as i32);
                    DCY = ((stbiw__jpg_processDU(
                        s,
                        ((&mut bitBuf) as *mut i32),
                        ((&mut bitCnt) as *mut i32),
                        (((Y.as_mut_ptr()).offset((136) as isize)) as *mut f32),
                        16,
                        ((fdtbl_Y.as_mut_ptr()) as *mut f32),
                        DCY,
                        &mut stbi_write_jpg_core_YDC_HT,
                        &mut stbi_write_jpg_core_YAC_HT,
                    )) as i32);
                    {
                        let mut subU: [f32; 64] = [0.0f32; 64];
                        let mut subV: [f32; 64] = [0.0f32; 64];
                        let mut yy: i32 = 0;
                        let mut xx: i32 = 0;
                        yy = ((0) as i32);
                        pos = ((0) as i32);
                        while (yy < 8) {
                            xx = ((0) as i32);
                            while (xx < 8) {
                                let mut j: i32 = yy * 32 + xx * 2;
                                subU[(pos) as usize] = (((U[(j + 0) as usize]
                                    + U[(j + 1) as usize]
                                    + U[(j + 16) as usize]
                                    + U[(j + 17) as usize])
                                    * 0.25f32)
                                    as f32);
                                subV[(pos) as usize] = (((V[(j + 0) as usize]
                                    + V[(j + 1) as usize]
                                    + V[(j + 16) as usize]
                                    + V[(j + 17) as usize])
                                    * 0.25f32)
                                    as f32);
                                preInc(&mut xx);
                                preInc(&mut pos);
                            }
                            preInc(&mut yy);
                        }
                        DCU = ((stbiw__jpg_processDU(
                            s,
                            ((&mut bitBuf) as *mut i32),
                            ((&mut bitCnt) as *mut i32),
                            ((subU.as_mut_ptr()) as *mut f32),
                            8,
                            ((fdtbl_UV.as_mut_ptr()) as *mut f32),
                            DCU,
                            &mut stbi_write_jpg_core_UVDC_HT,
                            &mut stbi_write_jpg_core_UVAC_HT,
                        )) as i32);
                        DCV = ((stbiw__jpg_processDU(
                            s,
                            ((&mut bitBuf) as *mut i32),
                            ((&mut bitCnt) as *mut i32),
                            ((subV.as_mut_ptr()) as *mut f32),
                            8,
                            ((fdtbl_UV.as_mut_ptr()) as *mut f32),
                            DCV,
                            &mut stbi_write_jpg_core_UVDC_HT,
                            &mut stbi_write_jpg_core_UVAC_HT,
                        )) as i32);
                    }
                    x += ((16) as i32);
                }
                y += ((16) as i32);
            }
        } else {
            y = ((0) as i32);
            while (y < height) {
                x = ((0) as i32);
                while (x < width) {
                    let mut Y: [f32; 64] = [0.0f32; 64];
                    let mut U: [f32; 64] = [0.0f32; 64];
                    let mut V: [f32; 64] = [0.0f32; 64];
                    row = ((y) as i32);
                    pos = ((0) as i32);
                    while (row < y + 8) {
                        let mut clamped_row: i32 = if (row < height) { row } else { height - 1 };
                        let mut base_p: i32 = (if (stbi__flip_vertically_on_write) != 0 {
                            (height - 1 - clamped_row)
                        } else {
                            clamped_row
                        }) * width
                            * comp;
                        col = ((x) as i32);
                        while (col < x + 8) {
                            let mut p: i32 =
                                base_p + (if (col < width) { col } else { (width - 1) }) * comp;
                            let mut r: f32 = ((*dataR.offset((p) as isize)) as f32);
                            let mut g: f32 = ((*dataG.offset((p) as isize)) as f32);
                            let mut b: f32 = ((*dataB.offset((p) as isize)) as f32);
                            Y[(pos) as usize] =
                                (0.29900f32 * r + 0.58700f32 * g + 0.11400f32 * b - ((128) as f32));
                            U[(pos) as usize] =
                                ((-0.16874f32 * r - 0.33126f32 * g + 0.50000f32 * b) as f32);
                            V[(pos) as usize] =
                                ((0.50000f32 * r - 0.41869f32 * g - 0.08131f32 * b) as f32);
                            preInc(&mut col);
                            preInc(&mut pos);
                        }
                        preInc(&mut row);
                    }
                    DCY = ((stbiw__jpg_processDU(
                        s,
                        ((&mut bitBuf) as *mut i32),
                        ((&mut bitCnt) as *mut i32),
                        ((Y.as_mut_ptr()) as *mut f32),
                        8,
                        ((fdtbl_Y.as_mut_ptr()) as *mut f32),
                        DCY,
                        &mut stbi_write_jpg_core_YDC_HT,
                        &mut stbi_write_jpg_core_YAC_HT,
                    )) as i32);
                    DCU = ((stbiw__jpg_processDU(
                        s,
                        ((&mut bitBuf) as *mut i32),
                        ((&mut bitCnt) as *mut i32),
                        ((U.as_mut_ptr()) as *mut f32),
                        8,
                        ((fdtbl_UV.as_mut_ptr()) as *mut f32),
                        DCU,
                        &mut stbi_write_jpg_core_UVDC_HT,
                        &mut stbi_write_jpg_core_UVAC_HT,
                    )) as i32);
                    DCV = ((stbiw__jpg_processDU(
                        s,
                        ((&mut bitBuf) as *mut i32),
                        ((&mut bitCnt) as *mut i32),
                        ((V.as_mut_ptr()) as *mut f32),
                        8,
                        ((fdtbl_UV.as_mut_ptr()) as *mut f32),
                        DCV,
                        &mut stbi_write_jpg_core_UVDC_HT,
                        &mut stbi_write_jpg_core_UVAC_HT,
                    )) as i32);
                    x += ((8) as i32);
                }
                y += ((8) as i32);
            }
        }
        stbiw__jpg_writeBits(
            s,
            ((&mut bitBuf) as *mut i32),
            ((&mut bitCnt) as *mut i32),
            ((stbi_write_jpg_core_fillBits.as_mut_ptr()) as *mut u16),
        );
    }
    stbiw__putc(s, ((0xFF) as u8));
    stbiw__putc(s, ((0xD9) as u8));
    return ((1) as i32);
}

pub unsafe fn stbi_write_jpg_to_func(
    mut func: fn(arg0: *mut u8, arg1: *mut u8, arg2: i32),
    mut context: *mut u8,
    mut x: i32,
    mut y: i32,
    mut comp: i32,
    mut data: *const u8,
    mut quality: i32,
) -> i32 {
    let mut s: stbi__write_context = stbi__write_context::new(func);
    stbi__start_write_callbacks(((&mut s) as *mut stbi__write_context), context);
    return ((stbi_write_jpg_core(&mut s, x, y, comp, data, quality)) as i32);
}

pub unsafe fn stbi_write_png_to_func(
    mut func: fn(arg0: *mut u8, arg1: *mut u8, arg2: i32),
    mut context: *mut u8,
    mut x: i32,
    mut y: i32,
    mut comp: i32,
    mut data: *const u8,
    mut stride_bytes: i32,
) -> i32 {
    let mut len: i32 = 0;
    let mut png: *mut u8 =
        stbi_write_png_to_mem(data, stride_bytes, x, y, comp, ((&mut len) as *mut i32));
    if png == std::ptr::null_mut() {
        return ((0) as i32);
    }
    (func)(context, png, len);
    free(png);
    return ((1) as i32);
}

pub unsafe fn stbi_write_png_to_mem(
    mut pixels: *const u8,
    mut stride_bytes: i32,
    mut x: i32,
    mut y: i32,
    mut n: i32,
    mut out_len: *mut i32,
) -> *mut u8 {
    let mut force_filter: i32 = stbi_write_force_png_filter;
    let mut ctype: [i32; 5] = [-1, 0, 4, 2, 6];
    let mut sig: [u8; 8] = [
        ((137) as u8),
        ((80) as u8),
        ((78) as u8),
        ((71) as u8),
        ((13) as u8),
        ((10) as u8),
        ((26) as u8),
        ((10) as u8),
    ];
    let mut out: *mut u8 = std::ptr::null_mut();
    let mut o: *mut u8 = std::ptr::null_mut();
    let mut filt: *mut u8 = std::ptr::null_mut();
    let mut zlib: *mut u8 = std::ptr::null_mut();
    let mut line_buffer: *mut i8 = std::ptr::null_mut();
    let mut j: i32 = 0;
    let mut zlen: i32 = 0;
    if stride_bytes == 0 {
        stride_bytes = ((x * n) as i32);
    }
    if force_filter >= 5 {
        force_filter = ((-1) as i32);
    }
    filt = malloc((((x * n + 1) * y) as u64));
    if filt == std::ptr::null_mut() {
        return ((std::ptr::null_mut()) as *mut u8);
    }
    line_buffer = ((malloc(((x * n) as u64))) as *mut i8);
    if line_buffer == std::ptr::null_mut() {
        free(filt);
        return ((std::ptr::null_mut()) as *mut u8);
    }
    j = ((0) as i32);
    while (j < y) {
        let mut filter_type: i32 = 0;
        if force_filter > -1 {
            filter_type = ((force_filter) as i32);
            stbiw__encode_png_line(
                (pixels),
                stride_bytes,
                x,
                y,
                j,
                n,
                force_filter,
                line_buffer,
            );
        } else {
            let mut best_filter: i32 = 0;
            let mut best_filter_val: i32 = 0x7fffffff;
            let mut est: i32 = 0;
            let mut i: i32 = 0;
            filter_type = ((0) as i32);
            while (filter_type < 5) {
                stbiw__encode_png_line(
                    (pixels),
                    stride_bytes,
                    x,
                    y,
                    j,
                    n,
                    filter_type,
                    line_buffer,
                );
                est = ((0) as i32);
                i = ((0) as i32);
                while (i < x * n) {
                    est += (abs(((*line_buffer.offset((i) as isize)) as i32)));
                    preInc(&mut i);
                }
                if est < best_filter_val {
                    best_filter_val = ((est) as i32);
                    best_filter = ((filter_type) as i32);
                }
                postInc(&mut filter_type);
            }
            if filter_type != best_filter {
                stbiw__encode_png_line(
                    (pixels),
                    stride_bytes,
                    x,
                    y,
                    j,
                    n,
                    best_filter,
                    line_buffer,
                );
                filter_type = ((best_filter) as i32);
            }
        }
        *filt.offset((j * (x * n + 1)) as isize) = ((filter_type) as u8);
        memmove(
            ((filt).offset((j * (x * n + 1)) as isize)).offset((1) as isize),
            ((line_buffer) as *mut u8),
            ((x * n) as u64),
        );
        preInc(&mut j);
    }
    free(((line_buffer) as *mut u8));
    zlib = stbi_zlib_compress(
        filt,
        y * (x * n + 1),
        ((&mut zlen) as *mut i32),
        stbi_write_png_compression_level,
    );
    free(filt);
    if zlib == std::ptr::null_mut() {
        return ((std::ptr::null_mut()) as *mut u8);
    }
    out = malloc(((8 + 12 + 13 + 12 + zlen + 12) as u64));
    if out == std::ptr::null_mut() {
        return ((std::ptr::null_mut()) as *mut u8);
    }
    *out_len = ((8 + 12 + 13 + 12 + zlen + 12) as i32);
    o = out;
    memmove(o, ((sig.as_mut_ptr()) as *mut u8), ((8) as u64));
    o = o.offset((8) as isize);
    *(o).offset((0) as isize) = ((((13) >> 24) & 0xff) as u8);
    *(o).offset((1) as isize) = ((((13) >> 16) & 0xff) as u8);
    *(o).offset((2) as isize) = ((((13) >> 8) & 0xff) as u8);
    *(o).offset((3) as isize) = (((13) & 0xff) as u8);
    (o) = (o).offset((4) as isize);

    *(o).offset((0) as isize) = (((('I') as i32) & 0xff) as u8);
    *(o).offset((1) as isize) = (((('H') as i32) & 0xff) as u8);
    *(o).offset((2) as isize) = (((('D') as i32) & 0xff) as u8);
    *(o).offset((3) as isize) = (((('R') as i32) & 0xff) as u8);
    (o) = (o).offset((4) as isize);
    *(o).offset((0) as isize) = ((((x) >> 24) & 0xff) as u8);
    *(o).offset((1) as isize) = ((((x) >> 16) & 0xff) as u8);
    *(o).offset((2) as isize) = ((((x) >> 8) & 0xff) as u8);
    *(o).offset((3) as isize) = (((x) & 0xff) as u8);
    (o) = (o).offset((4) as isize);

    *(o).offset((0) as isize) = ((((y) >> 24) & 0xff) as u8);
    *(o).offset((1) as isize) = ((((y) >> 16) & 0xff) as u8);
    *(o).offset((2) as isize) = ((((y) >> 8) & 0xff) as u8);
    *(o).offset((3) as isize) = (((y) & 0xff) as u8);
    (o) = (o).offset((4) as isize);

    *postIncPtr(&mut o) = ((8) as u8);
    *postIncPtr(&mut o) = (((ctype[(n) as usize]) & 0xff) as u8);
    *postIncPtr(&mut o) = ((0) as u8);
    *postIncPtr(&mut o) = ((0) as u8);
    *postIncPtr(&mut o) = ((0) as u8);
    stbiw__wpcrc(((&mut o) as *mut *mut u8), 13);
    *(o).offset((0) as isize) = ((((zlen) >> 24) & 0xff) as u8);
    *(o).offset((1) as isize) = ((((zlen) >> 16) & 0xff) as u8);
    *(o).offset((2) as isize) = ((((zlen) >> 8) & 0xff) as u8);
    *(o).offset((3) as isize) = (((zlen) & 0xff) as u8);
    (o) = (o).offset((4) as isize);

    *(o).offset((0) as isize) = ((('I' as i32) & 0xff) as u8);
    *(o).offset((1) as isize) = ((('D' as i32) & 0xff) as u8);
    *(o).offset((2) as isize) = (((('A') as i32) & 0xff) as u8);
    *(o).offset((3) as isize) = (((('T') as i32) & 0xff) as u8);
    (o) = (o).offset((4) as isize);
    memmove(o, zlib, ((zlen) as u64));
    o = o.offset((zlen) as isize);
    free(zlib);
    stbiw__wpcrc(((&mut o) as *mut *mut u8), zlen);
    *(o).offset((0) as isize) = ((((0) >> 24) & 0xff) as u8);
    *(o).offset((1) as isize) = ((((0) >> 16) & 0xff) as u8);
    *(o).offset((2) as isize) = ((((0) >> 8) & 0xff) as u8);
    *(o).offset((3) as isize) = (((0) & 0xff) as u8);
    (o) = (o).offset((4) as isize);

    *(o).offset((0) as isize) = (((('I') as i32) & 0xff) as u8);
    *(o).offset((1) as isize) = (((('E') as i32) & 0xff) as u8);
    *(o).offset((2) as isize) = (((('N') as i32) & 0xff) as u8);
    *(o).offset((3) as isize) = (((('D') as i32) & 0xff) as u8);
    (o) = (o).offset((4) as isize);
    stbiw__wpcrc(((&mut o) as *mut *mut u8), 0);

    return out;
}

pub unsafe fn stbi_zlib_compress(
    mut data: *const u8,
    mut data_len: i32,
    mut out_len: *mut i32,
    mut quality: i32,
) -> *mut u8 {
    let mut bitbuf: u32 = ((0) as u32);
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut bitcount: i32 = 0;
    let mut out: *mut u8 = std::ptr::null_mut();
    let mut hash_table: *mut *mut *const u8 =
        ((malloc(((16384) as u64) * std::mem::size_of::<*mut *mut u8>() as u64))
            as *mut *mut *const u8);
    if hash_table == std::ptr::null_mut() {
        return std::ptr::null_mut();
    }
    if quality < 5 {
        quality = ((5) as i32);
    }
    (if ((out) == ((std::ptr::null_mut()) as *mut u8)
        || *(((out) as *mut i32).offset(-((2) as isize))).offset((1) as isize) + (1)
            >= *(((out) as *mut i32).offset(-((2) as isize))).offset((0) as isize))
    {
        stbiw__sbgrowf(&mut (out), (1), ((std::mem::size_of::<u8>() as u64) as i32))
    } else {
        std::ptr::null_mut()
    });
    *(out).offset(
        (postInc(
            &mut *(((out) as *mut i32).offset(-((2) as isize))).offset((1) as isize),
        )) as isize,
    ) = ((0x78) as u8);

    (if ((out) == ((std::ptr::null_mut()) as *mut u8)
        || *(((out) as *mut i32).offset(-((2) as isize))).offset((1) as isize) + (1)
            >= *(((out) as *mut i32).offset(-((2) as isize))).offset((0) as isize))
    {
        stbiw__sbgrowf(&mut (out), (1), ((std::mem::size_of::<u8>() as u64) as i32))
    } else {
        std::ptr::null_mut()
    });
    *(out).offset(
        (postInc(
            &mut *(((out) as *mut i32).offset(-((2) as isize))).offset((1) as isize),
        )) as isize,
    ) = ((0x5e) as u8);

    {
        bitbuf |= (((1) << bitcount) as u32);
        bitcount += ((1) as i32);
        out = stbiw__zlib_flushf(
            out,
            ((&mut bitbuf) as *mut u32),
            ((&mut bitcount) as *mut i32),
        );
    }

    {
        bitbuf |= (((1) << bitcount) as u32);
        bitcount += ((2) as i32);
        out = stbiw__zlib_flushf(
            out,
            ((&mut bitbuf) as *mut u32),
            ((&mut bitcount) as *mut i32),
        );
    }

    i = ((0) as i32);
    while (i < 16384) {
        *hash_table.offset((i) as isize) = std::ptr::null_mut();
        preInc(&mut i);
    }
    i = ((0) as i32);
    while (i < data_len - 3) {
        let mut h: i32 =
            ((stbiw__zhash((data).offset((i) as isize)) & ((16384 - 1) as u32)) as i32);
        let mut best: i32 = 3;
        let mut bestloc: *const u8 = ((std::ptr::null_mut()) as *mut u8);
        let mut hlist: *mut *const u8 = *hash_table.offset((h) as isize);
        let mut n: i32 = (if (hlist) != std::ptr::null_mut() {
            *((((hlist) as *mut u8) as *mut i32).offset(-((2) as isize))).offset((1) as isize)
        } else {
            0
        });
        j = ((0) as i32);
        while (j < n) {
            if (*hlist.offset((j) as isize)) as isize - (data as isize) > ((i - 32768) as isize) {
                let mut d: i32 = ((stbiw__zlib_countm(
                    *hlist.offset((j) as isize),
                    (data).offset((i) as isize),
                    data_len - i,
                )) as i32);
                if d >= best {
                    best = ((d) as i32);
                    bestloc = *hlist.offset((j) as isize);
                }
            }
            preInc(&mut j);
        }
        if (*hash_table.offset((h) as isize)) != std::ptr::null_mut()
            && *((((*hash_table.offset((h) as isize)) as *mut u8) as *mut i32)
                .offset(-((2) as isize)))
            .offset((1) as isize)
                == 2 * quality
        {
            memmove(
                ((*hash_table.offset((h) as isize)) as *mut u8),
                (((*hash_table.offset((h) as isize)).offset((quality) as isize)) as *mut u8),
                std::mem::size_of::<*mut u8>() as u64 * ((quality) as u64),
            );
            *((((*hash_table.offset((h) as isize)) as *mut u8) as *mut i32)
                .offset(-((2) as isize)))
            .offset((1) as isize) = ((quality) as i32);
        }
        (if ((*hash_table.offset((h) as isize)) == std::ptr::null_mut()
            || *((((*hash_table.offset((h) as isize)) as *mut u8) as *mut i32)
                .offset(-((2) as isize)))
            .offset((1) as isize)
                + (1)
                >= *((((*hash_table.offset((h) as isize)) as *mut u8) as *mut i32)
                    .offset(-((2) as isize)))
                .offset((0) as isize))
        {
            stbiw__sbgrowf(
                hash_table.offset((h) as isize) as *mut *mut u8,
                (1),
                ((std::mem::size_of::<*mut u8>() as u64) as i32),
            )
        } else {
            std::ptr::null_mut()
        });
        *(*hash_table.offset((h) as isize)).offset(
            (postInc(
                &mut *((((*hash_table.offset((h) as isize)) as *mut u8) as *mut i32)
                    .offset(-((2) as isize)))
                .offset((1) as isize),
            )) as isize,
        ) = ((data).offset((i) as isize));
        if (bestloc) != std::ptr::null_mut() {
            h = ((stbiw__zhash(((data).offset((i) as isize)).offset((1) as isize))
                & ((16384 - 1) as u32)) as i32);
            hlist = *hash_table.offset((h) as isize);
            n = ((if (hlist) != std::ptr::null_mut() {
                *((((hlist) as *mut u8) as *mut i32).offset(-((2) as isize))).offset((1) as isize)
            } else {
                0
            }) as i32);
            j = ((0) as i32);
            while (j < n) {
                if (*hlist.offset((j) as isize)) as isize - (data as isize) > ((i - 32767) as isize)
                {
                    let mut e: i32 = ((stbiw__zlib_countm(
                        *hlist.offset((j) as isize),
                        ((data).offset((i) as isize)).offset((1) as isize),
                        data_len - i - 1,
                    )) as i32);
                    if e > best {
                        bestloc = std::ptr::null_mut();
                        break;
                    }
                }
                preInc(&mut j);
            }
        }
        if (bestloc) != std::ptr::null_mut() {
            let mut d: i32 = ((((data).offset((i) as isize)).offset(-((bestloc) as isize))) as i32);
            j = ((0) as i32);
            while (best > ((stbi_zlib_compress_lengthc[(j + 1) as usize]) as i32) - 1) {
                preInc(&mut j);
            }
            if j + 257 <= 143 {
                bitbuf |= (((stbiw__zlib_bitrev(0x30 + (j + 257), 8)) << bitcount) as u32);
                bitcount += ((8) as i32);
                out = stbiw__zlib_flushf(
                    out,
                    ((&mut bitbuf) as *mut u32),
                    ((&mut bitcount) as *mut i32),
                );
            } else {
                if j + 257 <= 255 {
                    bitbuf |=
                        (((stbiw__zlib_bitrev(0x190 + (j + 257) - 144, 9)) << bitcount) as u32);
                    bitcount += ((9) as i32);
                    out = stbiw__zlib_flushf(
                        out,
                        ((&mut bitbuf) as *mut u32),
                        ((&mut bitcount) as *mut i32),
                    );
                } else {
                    if j + 257 <= 279 {
                        bitbuf |=
                            (((stbiw__zlib_bitrev(0 + (j + 257) - 256, 7)) << bitcount) as u32);
                        bitcount += ((7) as i32);
                        out = stbiw__zlib_flushf(
                            out,
                            ((&mut bitbuf) as *mut u32),
                            ((&mut bitcount) as *mut i32),
                        );
                    } else {
                        bitbuf |=
                            (((stbiw__zlib_bitrev(0xc0 + (j + 257) - 280, 8)) << bitcount) as u32);
                        bitcount += ((8) as i32);
                        out = stbiw__zlib_flushf(
                            out,
                            ((&mut bitbuf) as *mut u32),
                            ((&mut bitcount) as *mut i32),
                        );
                    }
                }
            }
            if (stbi_zlib_compress_lengtheb[(j) as usize]) != 0 {
                bitbuf |= (((best - ((stbi_zlib_compress_lengthc[(j) as usize]) as i32))
                    << bitcount) as u32);
                bitcount += ((stbi_zlib_compress_lengtheb[(j) as usize]) as i32);
                out = stbiw__zlib_flushf(
                    out,
                    ((&mut bitbuf) as *mut u32),
                    ((&mut bitcount) as *mut i32),
                );
            }
            j = ((0) as i32);
            while (d > ((stbi_zlib_compress_distc[(j + 1) as usize]) as i32) - 1) {
                preInc(&mut j);
            }
            {
                bitbuf |= (((stbiw__zlib_bitrev(j, 5)) << bitcount) as u32);
                bitcount += ((5) as i32);
                out = stbiw__zlib_flushf(
                    out,
                    ((&mut bitbuf) as *mut u32),
                    ((&mut bitcount) as *mut i32),
                );
            }
            if (stbi_zlib_compress_disteb[(j) as usize]) != 0 {
                bitbuf |=
                    (((d - ((stbi_zlib_compress_distc[(j) as usize]) as i32)) << bitcount) as u32);
                bitcount += ((stbi_zlib_compress_disteb[(j) as usize]) as i32);
                out = stbiw__zlib_flushf(
                    out,
                    ((&mut bitbuf) as *mut u32),
                    ((&mut bitcount) as *mut i32),
                );
            }
            i += ((best) as i32);
        } else {
            if ((*data.offset((i) as isize)) as i32) <= 143 {
                bitbuf |= (((stbiw__zlib_bitrev(0x30 + ((*data.offset((i) as isize)) as i32), 8))
                    << bitcount) as u32);
                bitcount += ((8) as i32);
                out = stbiw__zlib_flushf(
                    out,
                    ((&mut bitbuf) as *mut u32),
                    ((&mut bitcount) as *mut i32),
                );
            } else {
                bitbuf |=
                    (((stbiw__zlib_bitrev(0x190 + ((*data.offset((i) as isize)) as i32) - 144, 9))
                        << bitcount) as u32);
                bitcount += ((9) as i32);
                out = stbiw__zlib_flushf(
                    out,
                    ((&mut bitbuf) as *mut u32),
                    ((&mut bitcount) as *mut i32),
                );
            }
            preInc(&mut i);
        }
    }
    while (i < data_len) {
        if ((*data.offset((i) as isize)) as i32) <= 143 {
            bitbuf |= (((stbiw__zlib_bitrev(0x30 + ((*data.offset((i) as isize)) as i32), 8))
                << bitcount) as u32);
            bitcount += ((8) as i32);
            out = stbiw__zlib_flushf(
                out,
                ((&mut bitbuf) as *mut u32),
                ((&mut bitcount) as *mut i32),
            );
        } else {
            bitbuf |=
                (((stbiw__zlib_bitrev(0x190 + ((*data.offset((i) as isize)) as i32) - 144, 9))
                    << bitcount) as u32);
            bitcount += ((9) as i32);
            out = stbiw__zlib_flushf(
                out,
                ((&mut bitbuf) as *mut u32),
                ((&mut bitcount) as *mut i32),
            );
        }
        preInc(&mut i);
    }

    if 256 <= 143 {
        bitbuf |= (((stbiw__zlib_bitrev(0x30 + (256), 8)) << bitcount) as u32);
        bitcount += ((8) as i32);
        out = stbiw__zlib_flushf(
            out,
            ((&mut bitbuf) as *mut u32),
            ((&mut bitcount) as *mut i32),
        );
    } else {
        if 256 <= 255 {
            bitbuf |= (((stbiw__zlib_bitrev(0x190 + (256) - 144, 9)) << bitcount) as u32);
            bitcount += ((9) as i32);
            out = stbiw__zlib_flushf(
                out,
                ((&mut bitbuf) as *mut u32),
                ((&mut bitcount) as *mut i32),
            );
        } else {
            if 256 <= 279 {
                bitbuf |= (((stbiw__zlib_bitrev(0 + (256) - 256, 7)) << bitcount) as u32);
                bitcount += ((7) as i32);
                out = stbiw__zlib_flushf(
                    out,
                    ((&mut bitbuf) as *mut u32),
                    ((&mut bitcount) as *mut i32),
                );
            } else {
                bitbuf |= (((stbiw__zlib_bitrev(0xc0 + (256) - 280, 8)) << bitcount) as u32);
                bitcount += ((8) as i32);
                out = stbiw__zlib_flushf(
                    out,
                    ((&mut bitbuf) as *mut u32),
                    ((&mut bitcount) as *mut i32),
                );
            }
        }
    }

    while ((bitcount) != 0) {
        bitbuf |= (((0) << bitcount) as u32);
        bitcount += ((1) as i32);
        out = stbiw__zlib_flushf(
            out,
            ((&mut bitbuf) as *mut u32),
            ((&mut bitcount) as *mut i32),
        );
    }

    i = ((0) as i32);
    while (i < 16384) {
        if (*hash_table.offset((i) as isize)) != std::ptr::null_mut() {
            free(
                (((((*hash_table.offset((i) as isize)) as *mut u8) as *mut i32)
                    .offset(-((2) as isize))) as *mut u8),
            );
        }
        preInc(&mut i);
    }
    free(((hash_table) as *mut u8));
    if *(((out) as *mut i32).offset(-((2) as isize))).offset((1) as isize)
        > data_len + 2 + ((data_len + 32766) / 32767) * 5
    {
        *(((out) as *mut i32).offset(-((2) as isize))).offset((1) as isize) = ((2) as i32);
        j = ((0) as i32);
        while (j < data_len) {
            let mut blocklen: i32 = data_len - j;
            if blocklen > 32767 {
                blocklen = ((32767) as i32);
            }
            (if ((out) == ((std::ptr::null_mut()) as *mut u8)
                || *(((out) as *mut i32).offset(-((2) as isize))).offset((1) as isize) + (1)
                    >= *(((out) as *mut i32).offset(-((2) as isize))).offset((0) as isize))
            {
                stbiw__sbgrowf(&mut (out), (1), ((std::mem::size_of::<u8>() as u64) as i32))
            } else {
                std::ptr::null_mut()
            });
            *(out).offset(
                (postInc(
                    &mut *(((out) as *mut i32).offset(-((2) as isize))).offset((1) as isize),
                )) as isize,
            ) = ((data_len - j == blocklen) as u8);
            (if ((out) == ((std::ptr::null_mut()) as *mut u8)
                || *(((out) as *mut i32).offset(-((2) as isize))).offset((1) as isize) + (1)
                    >= *(((out) as *mut i32).offset(-((2) as isize))).offset((0) as isize))
            {
                stbiw__sbgrowf(&mut (out), (1), ((std::mem::size_of::<u8>() as u64) as i32))
            } else {
                std::ptr::null_mut()
            });
            *(out).offset(
                (postInc(
                    &mut *(((out) as *mut i32).offset(-((2) as isize))).offset((1) as isize),
                )) as isize,
            ) = (((blocklen) & 0xff) as u8);
            (if ((out) == ((std::ptr::null_mut()) as *mut u8)
                || *(((out) as *mut i32).offset(-((2) as isize))).offset((1) as isize) + (1)
                    >= *(((out) as *mut i32).offset(-((2) as isize))).offset((0) as isize))
            {
                stbiw__sbgrowf(&mut (out), (1), ((std::mem::size_of::<u8>() as u64) as i32))
            } else {
                std::ptr::null_mut()
            });
            *(out).offset(
                (postInc(
                    &mut *(((out) as *mut i32).offset(-((2) as isize))).offset((1) as isize),
                )) as isize,
            ) = (((blocklen >> 8) & 0xff) as u8);
            (if ((out) == ((std::ptr::null_mut()) as *mut u8)
                || *(((out) as *mut i32).offset(-((2) as isize))).offset((1) as isize) + (1)
                    >= *(((out) as *mut i32).offset(-((2) as isize))).offset((0) as isize))
            {
                stbiw__sbgrowf(&mut (out), (1), ((std::mem::size_of::<u8>() as u64) as i32))
            } else {
                std::ptr::null_mut()
            });
            *(out).offset(
                (postInc(
                    &mut *(((out) as *mut i32).offset(-((2) as isize))).offset((1) as isize),
                )) as isize,
            ) = (((!blocklen) & 0xff) as u8);
            (if ((out) == ((std::ptr::null_mut()) as *mut u8)
                || *(((out) as *mut i32).offset(-((2) as isize))).offset((1) as isize) + (1)
                    >= *(((out) as *mut i32).offset(-((2) as isize))).offset((0) as isize))
            {
                stbiw__sbgrowf(&mut (out), (1), ((std::mem::size_of::<u8>() as u64) as i32))
            } else {
                std::ptr::null_mut()
            });
            *(out).offset(
                (postInc(
                    &mut *(((out) as *mut i32).offset(-((2) as isize))).offset((1) as isize),
                )) as isize,
            ) = (((!blocklen >> 8) & 0xff) as u8);
            memcpy(
                (out).offset(
                    (*(((out) as *mut i32).offset(-((2) as isize))).offset((1) as isize)) as isize,
                ),
                (data).offset((j) as isize),
                ((blocklen) as u64),
            );
            *(((out) as *mut i32).offset(-((2) as isize))).offset((1) as isize) +=
                ((blocklen) as i32);
            j += ((blocklen) as i32);
        }
    }
    {
        let mut s1: u32 = ((1) as u32);
        let mut s2: u32 = ((0) as u32);
        let mut blocklen: i32 = (data_len % 5552);
        j = ((0) as i32);
        while (j < data_len) {
            i = ((0) as i32);
            while (i < blocklen) {
                s1 += ((*data.offset((j + i) as isize)) as u32);
                s2 += ((s1) as u32);
                preInc(&mut i);
            }
            s1 %= ((65521) as u32);
            s2 %= ((65521) as u32);
            j += ((blocklen) as i32);
            blocklen = ((5552) as i32);
        }
        (if ((out) == ((std::ptr::null_mut()) as *mut u8)
            || *(((out) as *mut i32).offset(-((2) as isize))).offset((1) as isize) + (1)
                >= *(((out) as *mut i32).offset(-((2) as isize))).offset((0) as isize))
        {
            stbiw__sbgrowf(&mut (out), (1), ((std::mem::size_of::<u8>() as u64) as i32))
        } else {
            std::ptr::null_mut()
        });
        *(out).offset(
            (postInc(
                &mut *(((out) as *mut i32).offset(-((2) as isize))).offset((1) as isize),
            )) as isize,
        ) = (((s2 >> 8) & ((0xff) as u32)) as u8);
        (if ((out) == ((std::ptr::null_mut()) as *mut u8)
            || *(((out) as *mut i32).offset(-((2) as isize))).offset((1) as isize) + (1)
                >= *(((out) as *mut i32).offset(-((2) as isize))).offset((0) as isize))
        {
            stbiw__sbgrowf(&mut (out), (1), ((std::mem::size_of::<u8>() as u64) as i32))
        } else {
            std::ptr::null_mut()
        });
        *(out).offset(
            (postInc(
                &mut *(((out) as *mut i32).offset(-((2) as isize))).offset((1) as isize),
            )) as isize,
        ) = (((s2) & ((0xff) as u32)) as u8);
        (if ((out) == ((std::ptr::null_mut()) as *mut u8)
            || *(((out) as *mut i32).offset(-((2) as isize))).offset((1) as isize) + (1)
                >= *(((out) as *mut i32).offset(-((2) as isize))).offset((0) as isize))
        {
            stbiw__sbgrowf(&mut (out), (1), ((std::mem::size_of::<u8>() as u64) as i32))
        } else {
            std::ptr::null_mut()
        });
        *(out).offset(
            (postInc(
                &mut *(((out) as *mut i32).offset(-((2) as isize))).offset((1) as isize),
            )) as isize,
        ) = (((s1 >> 8) & ((0xff) as u32)) as u8);
        (if ((out) == ((std::ptr::null_mut()) as *mut u8)
            || *(((out) as *mut i32).offset(-((2) as isize))).offset((1) as isize) + (1)
                >= *(((out) as *mut i32).offset(-((2) as isize))).offset((0) as isize))
        {
            stbiw__sbgrowf(&mut (out), (1), ((std::mem::size_of::<u8>() as u64) as i32))
        } else {
            std::ptr::null_mut()
        });
        *(out).offset(
            (postInc(
                &mut *(((out) as *mut i32).offset(-((2) as isize))).offset((1) as isize),
            )) as isize,
        ) = (((s1) & ((0xff) as u32)) as u8);
    }
    *out_len = ((*(((out) as *mut i32).offset(-((2) as isize))).offset((1) as isize)) as i32);
    memmove(
        ((((out) as *mut i32).offset(-((2) as isize))) as *mut u8),
        out,
        ((*out_len) as u64),
    );
    return ((((out) as *mut i32).offset(-((2) as isize))) as *mut u8);
}

pub unsafe fn stbiw__crc32(mut buffer: *mut u8, mut len: i32) -> u32 {
    let mut crc: u32 = !0;
    let mut i: i32 = 0;
    i = ((0) as i32);
    while (i < len) {
        crc = (((crc >> 8)
            ^ stbiw__crc32_crc_table
                [(((*buffer.offset((i) as isize)) as u32) ^ (crc & ((0xff) as u32))) as usize])
            as u32);
        preInc(&mut i);
    }
    return ((!crc) as u32);
}

pub unsafe fn stbiw__encode_png_line(
    mut pixels: *const u8,
    mut stride_bytes: i32,
    mut width: i32,
    mut height: i32,
    mut y: i32,
    mut n: i32,
    mut filter_type: i32,
    mut line_buffer: *mut i8,
) {
    let mut mymap: *mut i32 = if (y != 0) {
        stbiw__encode_png_line_mapping.as_mut_ptr()
    } else {
        stbiw__encode_png_line_firstmap.as_mut_ptr()
    };
    let mut i: i32 = 0;
    let mut _type_: i32 = *mymap.offset((filter_type) as isize);
    let mut z: *const u8 = (pixels).offset(
        (stride_bytes
            * (if (stbi__flip_vertically_on_write) != 0 {
                height - 1 - y
            } else {
                y
            })) as isize,
    );
    let mut signed_stride: i32 = if (stbi__flip_vertically_on_write) != 0 {
        -stride_bytes
    } else {
        stride_bytes
    };
    if _type_ == 0 {
        memcpy(((line_buffer) as *mut u8), z, ((width * n) as u64));
        return;
    }
    i = ((0) as i32);
    while (i < n) {
        if _type_ == 1 {
            *line_buffer.offset((i) as isize) = ((*z.offset((i) as isize)) as i8);
        } else if _type_ == 2 {
            *line_buffer.offset((i) as isize) = ((((*z.offset((i) as isize)) as i32)
                - ((*z.offset((i - signed_stride) as isize)) as i32))
                as i8);
        } else if _type_ == 3 {
            *line_buffer.offset((i) as isize) = ((((*z.offset((i) as isize)) as i32)
                - (((*z.offset((i - signed_stride) as isize)) as i32) >> 1))
                as i8);
        } else if _type_ == 4 {
            *line_buffer.offset((i) as isize) = ((((*z.offset((i) as isize)) as i32)
                - stbiw__paeth(0, ((*z.offset((i - signed_stride) as isize)) as i32), 0) as i32)
                as i8);
        } else if _type_ == 5 {
            *line_buffer.offset((i) as isize) = ((*z.offset((i) as isize)) as i8);
        } else if _type_ == 6 {
            *line_buffer.offset((i) as isize) = ((*z.offset((i) as isize)) as i8);
        }
        preInc(&mut i);
    }
    if _type_ == 1 {
        i = ((n) as i32);
        while (i < width * n) {
            *line_buffer.offset((i) as isize) = ((((*z.offset((i) as isize)) as i32)
                - ((*z.offset((i - n) as isize)) as i32))
                as i8);
            preInc(&mut i);
        }
    } else if _type_ == 2 {
        i = ((n) as i32);
        while (i < width * n) {
            *line_buffer.offset((i) as isize) = ((((*z.offset((i) as isize)) as i32)
                - ((*z.offset((i - signed_stride) as isize)) as i32))
                as i8);
            preInc(&mut i);
        }
    } else if _type_ == 3 {
        i = ((n) as i32);
        while (i < width * n) {
            *line_buffer.offset((i) as isize) = ((((*z.offset((i) as isize)) as i32)
                - ((((*z.offset((i - n) as isize)) as i32)
                    + ((*z.offset((i - signed_stride) as isize)) as i32))
                    >> 1)) as i8);
            preInc(&mut i);
        }
    } else if _type_ == 4 {
        i = ((n) as i32);
        while (i < width * n) {
            *line_buffer.offset((i) as isize) = ((((*z.offset((i) as isize)) as i32)
                - stbiw__paeth(
                    ((*z.offset((i - n) as isize)) as i32),
                    ((*z.offset((i - signed_stride) as isize)) as i32),
                    ((*z.offset((i - signed_stride - n) as isize)) as i32),
                ) as i32) as i8);
            preInc(&mut i);
        }
    } else if _type_ == 5 {
        i = ((n) as i32);
        while (i < width * n) {
            *line_buffer.offset((i) as isize) = ((((*z.offset((i) as isize)) as i32)
                - (((*z.offset((i - n) as isize)) as i32) >> 1))
                as i8);
            preInc(&mut i);
        }
    } else if _type_ == 6 {
        i = ((n) as i32);
        while (i < width * n) {
            *line_buffer.offset((i) as isize) = ((((*z.offset((i) as isize)) as i32)
                - stbiw__paeth(((*z.offset((i - n) as isize)) as i32), 0, 0) as i32)
                as i8);
            preInc(&mut i);
        }
    }
}

pub unsafe fn stbiw__jpg_calcBits(mut val: i32, mut bits: &mut [u16; 2]) {
    let mut tmp1: i32 = if val < 0 { -val } else { val };
    val = ((if val < 0 { val - 1 } else { val }) as i32);
    bits[(1) as usize] = ((1) as u16);
    tmp1 >>= 1;
    while (tmp1 != 0) {
        preInc(&mut bits[(1) as usize]);
        tmp1 >>= 1;
    }
    bits[(0) as usize] = ((val & ((1 << ((bits[(1) as usize]) as i32)) - 1)) as u16);
}

pub unsafe fn stbiw__jpg_DCT(
    mut d0p: *mut f32,
    mut d1p: *mut f32,
    mut d2p: *mut f32,
    mut d3p: *mut f32,
    mut d4p: *mut f32,
    mut d5p: *mut f32,
    mut d6p: *mut f32,
    mut d7p: *mut f32,
) {
    let mut d0: f32 = *d0p;
    let mut d1: f32 = *d1p;
    let mut d2: f32 = *d2p;
    let mut d3: f32 = *d3p;
    let mut d4: f32 = *d4p;
    let mut d5: f32 = *d5p;
    let mut d6: f32 = *d6p;
    let mut d7: f32 = *d7p;
    let mut z1: f32 = 0.0f32;
    let mut z2: f32 = 0.0f32;
    let mut z3: f32 = 0.0f32;
    let mut z4: f32 = 0.0f32;
    let mut z5: f32 = 0.0f32;
    let mut z11: f32 = 0.0f32;
    let mut z13: f32 = 0.0f32;
    let mut tmp0: f32 = d0 + d7;
    let mut tmp7: f32 = d0 - d7;
    let mut tmp1: f32 = d1 + d6;
    let mut tmp6: f32 = d1 - d6;
    let mut tmp2: f32 = d2 + d5;
    let mut tmp5: f32 = d2 - d5;
    let mut tmp3: f32 = d3 + d4;
    let mut tmp4: f32 = d3 - d4;
    let mut tmp10: f32 = tmp0 + tmp3;
    let mut tmp13: f32 = tmp0 - tmp3;
    let mut tmp11: f32 = tmp1 + tmp2;
    let mut tmp12: f32 = tmp1 - tmp2;
    d0 = ((tmp10 + tmp11) as f32);
    d4 = ((tmp10 - tmp11) as f32);
    z1 = (((tmp12 + tmp13) * 0.707106781f32) as f32);
    d2 = ((tmp13 + z1) as f32);
    d6 = ((tmp13 - z1) as f32);
    tmp10 = ((tmp4 + tmp5) as f32);
    tmp11 = ((tmp5 + tmp6) as f32);
    tmp12 = ((tmp6 + tmp7) as f32);
    z5 = (((tmp10 - tmp12) * 0.382683433f32) as f32);
    z2 = ((tmp10 * 0.541196100f32 + z5) as f32);
    z4 = ((tmp12 * 1.306562965f32 + z5) as f32);
    z3 = ((tmp11 * 0.707106781f32) as f32);
    z11 = ((tmp7 + z3) as f32);
    z13 = ((tmp7 - z3) as f32);
    *d5p = ((z13 + z2) as f32);
    *d3p = ((z13 - z2) as f32);
    *d1p = ((z11 + z4) as f32);
    *d7p = ((z11 - z4) as f32);
    *d0p = ((d0) as f32);
    *d2p = ((d2) as f32);
    *d4p = ((d4) as f32);
    *d6p = ((d6) as f32);
}

pub unsafe fn stbiw__jpg_processDU(
    mut s: *mut stbi__write_context,
    mut bitBuf: *mut i32,
    mut bitCnt: *mut i32,
    mut CDU: *mut f32,
    mut du_stride: i32,
    mut fdtbl: *mut f32,
    mut DC: i32,
    mut HTDC: &mut [[u16; 2]; 12],
    mut HTAC: &mut [[u16; 2]; 256],
) -> i32 {
    let mut EOB: [u16; 2] = [
        HTAC[(0x00) as usize][(0) as usize],
        HTAC[(0x00) as usize][(1) as usize],
    ];
    let mut M16zeroes: [u16; 2] = [
        HTAC[(0xF0) as usize][(0) as usize],
        HTAC[(0xF0) as usize][(1) as usize],
    ];
    let mut dataOff: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut n: i32 = 0;
    let mut diff: i32 = 0;
    let mut end0pos: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut DU: [i32; 64] = [0; 64];
    dataOff = ((0) as i32);
    n = ((du_stride * 8) as i32);
    while (dataOff < n) {
        stbiw__jpg_DCT(
            ((&mut *CDU.offset((dataOff) as isize)) as *mut f32),
            ((&mut *CDU.offset((dataOff + 1) as isize)) as *mut f32),
            ((&mut *CDU.offset((dataOff + 2) as isize)) as *mut f32),
            ((&mut *CDU.offset((dataOff + 3) as isize)) as *mut f32),
            ((&mut *CDU.offset((dataOff + 4) as isize)) as *mut f32),
            ((&mut *CDU.offset((dataOff + 5) as isize)) as *mut f32),
            ((&mut *CDU.offset((dataOff + 6) as isize)) as *mut f32),
            ((&mut *CDU.offset((dataOff + 7) as isize)) as *mut f32),
        );
        dataOff += ((du_stride) as i32);
    }
    dataOff = ((0) as i32);
    while (dataOff < 8) {
        stbiw__jpg_DCT(
            ((&mut *CDU.offset((dataOff) as isize)) as *mut f32),
            ((&mut *CDU.offset((dataOff + du_stride) as isize)) as *mut f32),
            ((&mut *CDU.offset((dataOff + du_stride * 2) as isize)) as *mut f32),
            ((&mut *CDU.offset((dataOff + du_stride * 3) as isize)) as *mut f32),
            ((&mut *CDU.offset((dataOff + du_stride * 4) as isize)) as *mut f32),
            ((&mut *CDU.offset((dataOff + du_stride * 5) as isize)) as *mut f32),
            ((&mut *CDU.offset((dataOff + du_stride * 6) as isize)) as *mut f32),
            ((&mut *CDU.offset((dataOff + du_stride * 7) as isize)) as *mut f32),
        );
        preInc(&mut dataOff);
    }
    y = ((0) as i32);
    j = ((0) as i32);
    while (y < 8) {
        x = ((0) as i32);
        while (x < 8) {
            let mut v: f32 = 0.0f32;
            i = ((y * du_stride + x) as i32);
            v = ((*CDU.offset((i) as isize) * *fdtbl.offset((j) as isize)) as f32);
            DU[(stbiw__jpg_ZigZag[(j) as usize]) as usize] = ((if v < ((0) as f32) {
                v - 0.5f32
            } else {
                v + 0.5f32
            }) as i32);
            preInc(&mut x);
            preInc(&mut j);
        }
        preInc(&mut y);
    }
    diff = ((DU[(0) as usize] - DC) as i32);
    if diff == 0 {
        stbiw__jpg_writeBits(s, bitBuf, bitCnt, ((HTDC[(0) as usize]).as_mut_ptr()));
    } else {
        let mut bits: [u16; 2] = [0; 2];
        stbiw__jpg_calcBits(diff, &mut bits);
        stbiw__jpg_writeBits(
            s,
            bitBuf,
            bitCnt,
            ((HTDC[(bits[(1) as usize]) as usize]).as_mut_ptr()),
        );
        stbiw__jpg_writeBits(s, bitBuf, bitCnt, bits.as_mut_ptr());
    }
    end0pos = ((63) as i32);
    while ((end0pos > 0) && (DU[(end0pos) as usize] == 0)) {
        preDec(&mut end0pos);
    }
    if end0pos == 0 {
        stbiw__jpg_writeBits(s, bitBuf, bitCnt, ((EOB.as_mut_ptr()) as *mut u16));
        return ((DU[(0) as usize]) as i32);
    }
    i = ((1) as i32);
    while (i <= end0pos) {
        let mut startpos: i32 = i;
        let mut nrzeroes: i32 = 0;
        let mut bits: [u16; 2] = [0; 2];
        while (DU[(i) as usize] == 0 && i <= end0pos) {
            preInc(&mut i);
        }
        nrzeroes = ((i - startpos) as i32);
        if nrzeroes >= 16 {
            let mut lng: i32 = nrzeroes >> 4;
            let mut nrmarker: i32 = 0;
            nrmarker = ((1) as i32);
            while (nrmarker <= lng) {
                stbiw__jpg_writeBits(s, bitBuf, bitCnt, ((M16zeroes.as_mut_ptr()) as *mut u16));
                preInc(&mut nrmarker);
            }
            nrzeroes &= ((15) as i32);
        }
        stbiw__jpg_calcBits(DU[(i) as usize], &mut bits);
        stbiw__jpg_writeBits(
            s,
            bitBuf,
            bitCnt,
            ((HTAC[((nrzeroes << 4) + ((bits[(1) as usize]) as i32)) as usize]).as_mut_ptr()),
        );
        stbiw__jpg_writeBits(s, bitBuf, bitCnt, bits.as_mut_ptr());
        preInc(&mut i);
    }
    if end0pos != 63 {
        stbiw__jpg_writeBits(s, bitBuf, bitCnt, EOB.as_mut_ptr());
    }
    return ((DU[(0) as usize]) as i32);
}

pub unsafe fn stbiw__jpg_writeBits(
    mut s: *mut stbi__write_context,
    mut bitBufP: *mut i32,
    mut bitCntP: *mut i32,
    mut bs: *mut u16,
) {
    let mut bitBuf: i32 = *bitBufP;
    let mut bitCnt: i32 = *bitCntP;
    bitCnt += ((*bs.offset((1) as isize)) as i32);
    bitBuf |= (((*bs.offset((0) as isize)) as i32) << (24 - bitCnt));
    while (bitCnt >= 8) {
        let mut c: u8 = (((bitBuf >> 16) & 255) as u8);
        stbiw__putc(s, c);
        if ((c) as i32) == 255 {
            stbiw__putc(s, ((0) as u8));
        }
        bitBuf <<= 8;
        bitCnt -= ((8) as i32);
    }
    *bitBufP = ((bitBuf) as i32);
    *bitCntP = ((bitCnt) as i32);
}

pub unsafe fn stbiw__paeth(mut a: i32, mut b: i32, mut c: i32) -> u8 {
    let mut p: i32 = a + b - c;
    let mut pa: i32 = abs(p - a);
    let mut pb: i32 = abs(p - b);
    let mut pc: i32 = abs(p - c);
    if pa <= pb && pa <= pc {
        return (((a) & 0xff) as u8);
    }
    if pb <= pc {
        return (((b) & 0xff) as u8);
    }
    return (((c) & 0xff) as u8);
}

pub unsafe fn stbiw__putc(mut s: *mut stbi__write_context, mut c: u8) {
    ((*s).func)((*s).context, ((&mut c) as *mut u8), 1);
}

pub unsafe fn stbiw__sbgrowf(
    mut arr: *mut *mut u8,
    mut increment: i32,
    mut itemsize: i32,
) -> *mut u8 {
    let mut m: i32 = if *arr != std::ptr::null_mut() {
        2 * *(((*arr) as *mut i32).offset(-((2) as isize))).offset((0) as isize) + increment
    } else {
        increment + 1
    };
    let mut p: *mut u8 = realloc(
        ((if *arr != std::ptr::null_mut() {
            (((*arr) as *mut i32).offset(-((2) as isize)))
        } else {
            ((std::ptr::null_mut()) as *mut i32)
        }) as *mut u8),
        ((itemsize * m) as u64) + std::mem::size_of::<i32>() as u64 * ((2) as u64),
    );

    if (p) != std::ptr::null_mut() {
        if *arr == std::ptr::null_mut() {
            *((p) as *mut i32).offset((1) as isize) = ((0) as i32);
        }
        *arr = ((((p) as *mut i32).offset((2) as isize)) as *mut u8);
        *(((*arr) as *mut i32).offset(-((2) as isize))).offset((0) as isize) = ((m) as i32);
    }
    return *arr;
}

pub unsafe fn stbiw__wpcrc(mut data: *mut *mut u8, mut len: i32) {
    let mut crc: u32 = stbiw__crc32(
        ((*data).offset(-((len) as isize))).offset(-((4) as isize)),
        len + 4,
    );
    *(*data).offset((0) as isize) = ((((crc) >> 24) & ((0xff) as u32)) as u8);
    *(*data).offset((1) as isize) = ((((crc) >> 16) & ((0xff) as u32)) as u8);
    *(*data).offset((2) as isize) = ((((crc) >> 8) & ((0xff) as u32)) as u8);
    *(*data).offset((3) as isize) = (((crc) & ((0xff) as u32)) as u8);
    (*data) = (*data).offset((4) as isize);
}

pub unsafe fn stbiw__write_flush(mut s: *mut stbi__write_context) {
    if ((*s).buf_used) != 0 {
        ((*s).func)((*s).context, ((&mut (*s).buffer) as *mut u8), (*s).buf_used);
        (*s).buf_used = ((0) as i32);
    }
}

pub unsafe fn stbiw__write_pixel(
    mut s: *mut stbi__write_context,
    mut rgb_dir: i32,
    mut comp: i32,
    mut write_alpha: i32,
    mut expand_mono: i32,
    mut d: *const u8,
) {
    let mut bg: [u8; 3] = [((255) as u8), ((0) as u8), ((255) as u8)];
    let mut px: [u8; 3] = [0; 3];
    let mut k: i32 = 0;
    if write_alpha < 0 {
        stbiw__write1(s, *d.offset((comp - 1) as isize));
    }
    if comp == 2 || comp == 1 {
        if (expand_mono) != 0 {
            stbiw__write3(
                s,
                *d.offset((0) as isize),
                *d.offset((0) as isize),
                *d.offset((0) as isize),
            );
        } else {
            stbiw__write1(s, *d.offset((0) as isize));
        }
    } else if comp == 3 || comp == 4 {
        if comp == 4 && write_alpha == 0 {
            k = ((0) as i32);
            while (k < 3) {
                px[(k) as usize] = ((((bg[(k) as usize]) as i32)
                    + ((((*d.offset((k) as isize)) as i32) - ((bg[(k) as usize]) as i32))
                        * ((*d.offset((3) as isize)) as i32))
                        / 255) as u8);
                preInc(&mut k);
            }
            stbiw__write3(
                s,
                px[(1 - rgb_dir) as usize],
                px[(1) as usize],
                px[(1 + rgb_dir) as usize],
            );
        } else {
            stbiw__write3(
                s,
                *d.offset((1 - rgb_dir) as isize),
                *d.offset((1) as isize),
                *d.offset((1 + rgb_dir) as isize),
            );
        }
    }
    if write_alpha > 0 {
        stbiw__write1(s, *d.offset((comp - 1) as isize));
    }
}

pub unsafe fn stbiw__write_pixels(
    mut s: *mut stbi__write_context,
    mut rgb_dir: i32,
    mut vdir: i32,
    mut x: i32,
    mut y: i32,
    mut comp: i32,
    mut data: *const u8,
    mut write_alpha: i32,
    mut scanline_pad: i32,
    mut expand_mono: i32,
) {
    let mut zero: u32 = ((0) as u32);
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut j_end: i32 = 0;
    if y <= 0 {
        return;
    }
    if (stbi__flip_vertically_on_write) != 0 {
        vdir *= ((-1) as i32);
    }
    if vdir < 0 {
        j_end = ((-1) as i32);
        j = ((y - 1) as i32);
    } else {
        j_end = ((y) as i32);
        j = ((0) as i32);
    };
    while (j != j_end) {
        i = ((0) as i32);
        while (i < x) {
            let mut d: *const u8 = (data).offset(((j * x + i) * comp) as isize);
            stbiw__write_pixel(s, rgb_dir, comp, write_alpha, expand_mono, d);
            preInc(&mut i);
        }
        stbiw__write_flush(s);
        ((*s).func)(
            (*s).context,
            (((&mut zero) as *mut u32) as *mut u8),
            scanline_pad,
        );
        j += ((vdir) as i32);
    }
}

pub unsafe fn stbiw__write1(mut s: *mut stbi__write_context, mut a: u8) {
    if (((*s).buf_used) as u64) + ((1) as u64) > 64 * std::mem::size_of::<u8>() as u64 {
        stbiw__write_flush(s);
    }
    (*s).buffer[(postInc(&mut (*s).buf_used)) as usize] = ((a) as u8);
}

pub unsafe fn stbiw__write3(mut s: *mut stbi__write_context, mut a: u8, mut b: u8, mut c: u8) {
    let mut n: i32 = 0;
    if (((*s).buf_used) as u64) + ((3) as u64) > 64 * std::mem::size_of::<u8>() as u64 {
        stbiw__write_flush(s);
    }
    n = (((*s).buf_used) as i32);
    (*s).buf_used = ((n + 3) as i32);
    (*s).buffer[(n + 0) as usize] = ((a) as u8);
    (*s).buffer[(n + 1) as usize] = ((b) as u8);
    (*s).buffer[(n + 2) as usize] = ((c) as u8);
}

pub unsafe fn stbiw__zhash(mut data: *const u8) -> u32 {
    let mut hash: u32 = ((((*data.offset((0) as isize)) as i32)
        + (((*data.offset((1) as isize)) as i32) << 8)
        + (((*data.offset((2) as isize)) as i32) << 16)) as u32);
    hash ^= ((hash << 3) as u32);
    hash = u32::wrapping_add(hash, ((hash >> 5) as u32));
    hash ^= ((hash << 4) as u32);
    hash = u32::wrapping_add(hash, ((hash >> 17) as u32));
    hash ^= ((hash << 25) as u32);
    hash = u32::wrapping_add(hash, ((hash >> 6) as u32));
    return ((hash) as u32);
}

pub unsafe fn stbiw__zlib_bitrev(mut code: i32, mut codebits: i32) -> i32 {
    let mut res: i32 = 0;
    while ((postDec(&mut codebits)) != 0) {
        res = (((res << 1) | (code & 1)) as i32);
        code >>= 1;
    }
    return ((res) as i32);
}

pub unsafe fn stbiw__zlib_countm(mut a: *const u8, mut b: *const u8, mut limit: i32) -> u32 {
    let mut i: i32 = 0;
    i = ((0) as i32);
    while (i < limit && i < 258) {
        if ((*a.offset((i) as isize)) as i32) != ((*b.offset((i) as isize)) as i32) {
            break;
        }
        preInc(&mut i);
    }
    return ((i) as u32);
}

pub unsafe fn stbiw__zlib_flushf(
    mut data: *mut u8,
    mut bitbuffer: *mut u32,
    mut bitcount: *mut i32,
) -> *mut u8 {
    while (*bitcount >= 8) {
        (if ((data) == ((std::ptr::null_mut()) as *mut u8)
            || *(((data) as *mut i32).offset(-((2) as isize))).offset((1) as isize) + (1)
                >= *(((data) as *mut i32).offset(-((2) as isize))).offset((0) as isize))
        {
            stbiw__sbgrowf(
                &mut (data),
                (1),
                ((std::mem::size_of::<u8>() as u64) as i32),
            )
        } else {
            std::ptr::null_mut()
        });

        let value = (((*bitbuffer) & ((0xff) as u32)) as u8);
        let index = (postInc(
            &mut *(((data) as *mut i32).offset(-((2) as isize))).offset((1) as isize),
        )) as isize;
        *(data).offset(index) = value;

        *bitbuffer >>= 8;
        *bitcount -= ((8) as i32);
    }
    return data;
}
