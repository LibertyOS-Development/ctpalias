#![allow(non_camel_case_types)]
#![no_std]

pub use ad::*;
pub use od::*;
pub use pwd::*;

#[cfg(any(target_arch = "aarch64", target_arch = "arm", target_arch = "armjs", target_arch = "wasm32", target_arch = "wasm64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "riscv32", target_arch = "riscv64"))]
mod ad
{
	pub type cchar = ::cuchar;
	pub type cint = i32;
	pub type cuint = u32;
}

#[cfg(any(target_arch = "mips", target_arch = "mips64", target_arch = "sparc64", target_arch = "x86", target_arch = "x86_64", target_arch = "nvptx", target_arch = "nvptx64", target_arch = "xtensa"))]
mod ad
{
	pub type cchar = crate::cschar;
	pub type cint = i32;
	pub type cuint = u32;
}

#[cfg(target_arch = "msp430")]
mod ad
{
	pub type cchar = ::cuchar;
	pub type cint = i16;
	pub type cunit = u16;
}

#[cfg(not(any(windows, target_os = "redox", target_os = "solaris")))]
mod od
{
	#[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
	pub type clong = i32;
	#[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
	pub type culong = u32;
	#[cfg(target_pointer_width = "64")]
	pub type clong = i64;
	#[cfg(target_pointer_width = "64")]
	pub type culong = u64;
}

#[cfg(windows)]
mod od
{
	pub type clong = i32;
	pub type culong = u32;
}

#[cfg(any(target_os = "redox", target_os = "solaris"))]
mod od
{
	pub type clong = i64;
	pub type culong = u64;
}

#[cfg(target_pointer_width = "32")]
mod pwd {}
#[cfg(target_pointer_width = "64")]
mod pwd {}

pub type int8t = i8;
pub type int16t = i16;
pub type int32t = i32;
pub type int64t = i64;
pub type uint8t = u8;
pub type uint16t = u16;
pub type uint32t = u32;
pub type uint64t = u64;
pub type cschar = i8;
pub type cshort = i16;
pub type clonglong = i64;
pub type cuchar = u8;
pub type cushort = u16;
pub type culonglong = u64;
pub type cfloat = f32;
pub type cdouble = f64;
pub type intmaxt = i64;
pub type uintmaxt = u64;
pub type sizet = usize;
pub type ptrdifft = isize;
pub type intptrt = isize;
pub type uintptrt = usize;
pub type ssizet = isize;
pub type cvoid = core::ffi::c_void;
