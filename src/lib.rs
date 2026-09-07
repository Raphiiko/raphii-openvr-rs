#![doc = include_str!("../README.md")]
#![deny(unsafe_op_in_unsafe_fn)]
#![allow(clippy::unnecessary_cast, reason = "SDK integer types vary by target")]

#[cfg(not(any(
    all(target_os = "windows", target_arch = "x86_64"),
    all(
        target_os = "linux",
        any(target_arch = "x86_64", target_arch = "aarch64")
    )
)))]
compile_error!("supported targets: Windows x64, Linux x64, Linux ARM64");

pub mod applications;
pub mod input;
pub mod overlay;
pub mod raw;
mod runtime;
pub mod settings;
pub mod system;
pub use runtime::Context;

use std::ffi::{CStr, CString};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    AlreadyInitialized,
    NotInitialized,
    Load(String),
    Runtime { interface: &'static str, code: u32 },
    InterfaceUnavailable { interface: &'static str, code: u32 },
    MissingFunction(&'static str),
    InvalidInput(&'static str),
    InvalidResponse(&'static str),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyInitialized => {
                f.write_str("OpenVR is already initialized through this crate")
            }
            Self::NotInitialized => f.write_str("OpenVR session has shut down"),
            Self::Load(e) => write!(f, "cannot load OpenVR: {e}"),
            Self::Runtime { interface, code } => write!(f, "{interface} error {code}"),
            Self::InterfaceUnavailable { interface, code } => {
                write!(f, "{interface} unavailable (initialization error {code})")
            }
            Self::MissingFunction(name) => write!(f, "OpenVR function {name} is missing"),
            Self::InvalidInput(reason) | Self::InvalidResponse(reason) => f.write_str(reason),
        }
    }
}
impl std::error::Error for Error {}
pub type Result<T> = std::result::Result<T, Error>;

fn check(interface: &'static str, code: u32) -> Result<()> {
    if code == 0 {
        Ok(())
    } else {
        Err(Error::Runtime { interface, code })
    }
}
fn cstring(value: &str) -> Result<CString> {
    CString::new(value).map_err(|_| Error::InvalidInput("string contains a NUL byte"))
}
fn path_string(path: &std::path::Path) -> Result<CString> {
    cstring(
        path.to_str()
            .ok_or(Error::InvalidInput("path is not UTF-8"))?,
    )
}
fn string_buffer(bytes: &[u8]) -> Result<String> {
    let value = CStr::from_bytes_until_nul(bytes)
        .map_err(|_| Error::InvalidResponse("runtime string is not NUL terminated"))?;
    value
        .to_str()
        .map(str::to_owned)
        .map_err(|_| Error::InvalidResponse("runtime string is not UTF-8"))
}

macro_rules! function {
    ($table:expr, $name:ident) => {
        $table
            .$name
            .ok_or($crate::Error::MissingFunction(stringify!($name)))?
    };
}
pub(crate) use function;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TrackedDeviceIndex(pub u32);
impl TrackedDeviceIndex {
    pub const HMD: Self = Self(0);
    pub fn new(index: u32) -> Result<Self> {
        if index < raw::k_unMaxTrackedDeviceCount as u32 {
            Ok(Self(index))
        } else {
            Err(Error::InvalidInput("tracked device index is out of range"))
        }
    }
}
pub mod pose {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Matrix3x4(pub [[f32; 4]; 3]);
}
