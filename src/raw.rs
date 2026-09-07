//! Generated SDK declarations. Calls require a live, matching OpenVR runtime.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(clippy::all, unpredictable_function_pointer_comparisons)]

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
include!("generated/x86_64-pc-windows-msvc.rs");
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
include!("generated/x86_64-unknown-linux-gnu.rs");
#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
include!("generated/aarch64-unknown-linux-gnu.rs");

pub(crate) mod exports {
    #![allow(dead_code)]
    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    include!("generated/x86_64-pc-windows-msvc-exports.rs");
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    include!("generated/x86_64-unknown-linux-gnu-exports.rs");
    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    include!("generated/aarch64-unknown-linux-gnu-exports.rs");
}
