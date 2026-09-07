use crate::{Context, Result, check, function, raw};
use std::ffi::CStr;

pub struct Settings<'a>(pub(crate) &'a Context);
macro_rules! scalar {
    ($get:ident, $set:ident, $ty:ty, $get_raw:ident, $set_raw:ident) => {
        pub fn $get(&self, section: &CStr, key: &CStr) -> Result<$ty> {
            self.0.with(|s| {
                let table = s.settings.as_ref().map_err(Clone::clone)?;
                let mut error = raw::EVRSettingsError(0);
                let value = unsafe {
                    function!(table, $get_raw)(
                        section.as_ptr().cast_mut(),
                        key.as_ptr().cast_mut(),
                        &mut error,
                    )
                };
                check("settings", error.0 as u32)?;
                Ok(value)
            })
        }
        pub fn $set(&self, section: &CStr, key: &CStr, value: $ty) -> Result<()> {
            self.0.with(|s| {
                let table = s.settings.as_ref().map_err(Clone::clone)?;
                let mut error = raw::EVRSettingsError(0);
                unsafe {
                    function!(table, $set_raw)(
                        section.as_ptr().cast_mut(),
                        key.as_ptr().cast_mut(),
                        value,
                        &mut error,
                    )
                };
                check("settings", error.0 as u32)
            })
        }
    };
}
impl Settings<'_> {
    scalar!(get_float, set_float, f32, GetFloat, SetFloat);
    scalar!(get_bool, set_bool, bool, GetBool, SetBool);
    scalar!(get_int32, set_int32, i32, GetInt32, SetInt32);

    pub fn get_string(&self, section: &CStr, key: &CStr) -> Result<String> {
        self.0.with(|s| {
            let table = s.settings.as_ref().map_err(Clone::clone)?;
            let mut data = vec![0xff; 32768];
            let mut error = raw::EVRSettingsError(0);
            unsafe {
                function!(table, GetString)(
                    section.as_ptr().cast_mut(),
                    key.as_ptr().cast_mut(),
                    data.as_mut_ptr().cast(),
                    data.len() as u32,
                    &mut error,
                )
            };
            check("settings", error.0 as u32)?;
            crate::string_buffer(&data)
        })
    }
    pub fn set_string(&self, section: &CStr, key: &CStr, value: &CStr) -> Result<()> {
        self.0.with(|s| {
            let table = s.settings.as_ref().map_err(Clone::clone)?;
            let mut error = raw::EVRSettingsError(0);
            unsafe {
                function!(table, SetString)(
                    section.as_ptr().cast_mut(),
                    key.as_ptr().cast_mut(),
                    value.as_ptr().cast_mut(),
                    &mut error,
                )
            };
            check("settings", error.0 as u32)
        })
    }
    pub fn remove_key_in_section(&self, section: &CStr, key: &CStr) -> Result<()> {
        self.0.with(|s| {
            let table = s.settings.as_ref().map_err(Clone::clone)?;
            let mut error = raw::EVRSettingsError(0);
            unsafe {
                function!(table, RemoveKeyInSection)(
                    section.as_ptr().cast_mut(),
                    key.as_ptr().cast_mut(),
                    &mut error,
                )
            };
            check("settings", error.0 as u32)
        })
    }
    pub fn remove_section(&self, section: &CStr) -> Result<()> {
        self.0.with(|s| {
            let table = s.settings.as_ref().map_err(Clone::clone)?;
            let mut error = raw::EVRSettingsError(0);
            unsafe { function!(table, RemoveSection)(section.as_ptr().cast_mut(), &mut error) };
            check("settings", error.0 as u32)
        })
    }
}
