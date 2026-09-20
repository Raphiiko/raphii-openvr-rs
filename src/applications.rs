use crate::{Context, Result, check, cstring, function, path_string};
use std::path::Path;

pub struct Applications<'a>(pub(crate) &'a Context);
impl Applications<'_> {
    pub fn add_application_manifest(&self, path: &Path, temporary: bool) -> Result<()> {
        let path = path_string(path)?;
        self.0.with(|s| {
            let table = s.applications.as_ref().map_err(Clone::clone)?;
            check(
                "applications",
                unsafe {
                    function!(table, AddApplicationManifest)(path.as_ptr().cast_mut(), temporary)
                }
                .0 as u32,
            )
        })
    }
    pub fn remove_application_manifest(&self, path: &Path) -> Result<()> {
        let path = path_string(path)?;
        self.0.with(|s| {
            let table = s.applications.as_ref().map_err(Clone::clone)?;
            check(
                "applications",
                unsafe { function!(table, RemoveApplicationManifest)(path.as_ptr().cast_mut()) }.0
                    as u32,
            )
        })
    }
    pub fn is_application_installed(&self, key: &str) -> Result<bool> {
        let key = cstring(key)?;
        self.0.with(|s| {
            let table = s.applications.as_ref().map_err(Clone::clone)?;
            Ok(unsafe { function!(table, IsApplicationInstalled)(key.as_ptr().cast_mut()) })
        })
    }
    pub fn get_application_auto_launch(&self, key: &str) -> Result<bool> {
        let key = cstring(key)?;
        self.0.with(|s| {
            let table = s.applications.as_ref().map_err(Clone::clone)?;
            Ok(unsafe { function!(table, GetApplicationAutoLaunch)(key.as_ptr().cast_mut()) })
        })
    }
    pub fn set_application_auto_launch(&self, key: &str, auto_launch: bool) -> Result<()> {
        let key = cstring(key)?;
        self.0.with(|s| {
            let table = s.applications.as_ref().map_err(Clone::clone)?;
            check(
                "applications",
                unsafe {
                    function!(table, SetApplicationAutoLaunch)(key.as_ptr().cast_mut(), auto_launch)
                }
                .0 as u32,
            )
        })
    }
}
