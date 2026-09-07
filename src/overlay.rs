use crate::{
    Context, Error, Result, TrackedDeviceIndex, check, cstring, function, pose::Matrix3x4, raw,
};

/// A session-scoped overlay ID. Destroy it explicitly, or shut down its context.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OverlayHandle {
    id: u64,
    session: u64,
}
pub struct Overlays<'a>(pub(crate) &'a Context);

macro_rules! setter {
    ($name:ident, $method:ident, $ty:ty) => {
        pub fn $name(&self, overlay: OverlayHandle, value: $ty) -> Result<()> {
            self.0.with(|s| {
                overlay.validate(s.id)?;
                let table = s.overlay.as_ref().map_err(Clone::clone)?;
                check(
                    "overlay",
                    unsafe { function!(table, $method)(overlay.id, value) }.0 as u32,
                )
            })
        }
    };
}
impl OverlayHandle {
    fn validate(self, session: u64) -> Result<()> {
        if self.session == session {
            Ok(())
        } else {
            Err(Error::InvalidInput(
                "overlay belongs to a different session",
            ))
        }
    }
}
impl Overlays<'_> {
    pub fn create_overlay(&self, key: &str, name: &str) -> Result<OverlayHandle> {
        let key = cstring(key)?;
        let name = cstring(name)?;
        self.0.with(|s| {
            let table = s.overlay.as_ref().map_err(Clone::clone)?;
            let mut id = 0;
            check(
                "overlay",
                unsafe {
                    function!(table, CreateOverlay)(
                        key.as_ptr().cast_mut(),
                        name.as_ptr().cast_mut(),
                        &mut id,
                    )
                }
                .0 as u32,
            )?;
            if id == 0 {
                return Err(Error::InvalidResponse(
                    "runtime returned an invalid overlay handle",
                ));
            }
            Ok(OverlayHandle { id, session: s.id })
        })
    }
    pub fn destroy_overlay(&self, overlay: OverlayHandle) -> Result<()> {
        self.0.with(|s| {
            overlay.validate(s.id)?;
            let table = s.overlay.as_ref().map_err(Clone::clone)?;
            check(
                "overlay",
                unsafe { function!(table, DestroyOverlay)(overlay.id) }.0 as u32,
            )
        })
    }
    setter!(set_opacity, SetOverlayAlpha, f32);
    setter!(set_width, SetOverlayWidthInMeters, f32);
    setter!(set_sort_order, SetOverlaySortOrder, u32);

    pub fn set_visibility(&self, overlay: OverlayHandle, visible: bool) -> Result<()> {
        self.0.with(|s| {
            overlay.validate(s.id)?;
            let table = s.overlay.as_ref().map_err(Clone::clone)?;
            let set = if visible {
                function!(table, ShowOverlay)
            } else {
                function!(table, HideOverlay)
            };
            check("overlay", unsafe { set(overlay.id) }.0 as u32)
        })
    }
    pub fn is_dashboard_visible(&self) -> Result<bool> {
        self.0.with(|s| {
            let table = s.overlay.as_ref().map_err(Clone::clone)?;
            Ok(unsafe { function!(table, IsDashboardVisible)() })
        })
    }
    pub fn set_transform_tracked_device_relative(
        &self,
        overlay: OverlayHandle,
        device: TrackedDeviceIndex,
        matrix: &Matrix3x4,
    ) -> Result<()> {
        TrackedDeviceIndex::new(device.0)?;
        let mut matrix = raw::HmdMatrix34_t { m: matrix.0 };
        self.0.with(|s| {
            overlay.validate(s.id)?;
            let table = s.overlay.as_ref().map_err(Clone::clone)?;
            check(
                "overlay",
                unsafe {
                    function!(table, SetOverlayTransformTrackedDeviceRelative)(
                        overlay.id,
                        device.0,
                        &mut matrix,
                    )
                }
                .0 as u32,
            )
        })
    }
    pub fn set_raw_data(
        &self,
        overlay: OverlayHandle,
        bytes: &[u8],
        width: u32,
        height: u32,
        bytes_per_pixel: u32,
    ) -> Result<()> {
        let length = (width as usize)
            .checked_mul(height as usize)
            .and_then(|n| n.checked_mul(bytes_per_pixel as usize));
        if width == 0
            || height == 0
            || !(1..=4).contains(&bytes_per_pixel)
            || length != Some(bytes.len())
        {
            return Err(Error::InvalidInput(
                "overlay dimensions do not match pixel buffer",
            ));
        }
        self.0.with(|s| {
            overlay.validate(s.id)?;
            let table = s.overlay.as_ref().map_err(Clone::clone)?;
            check(
                "overlay",
                unsafe {
                    function!(table, SetOverlayRaw)(
                        overlay.id,
                        bytes.as_ptr().cast_mut().cast(),
                        width,
                        height,
                        bytes_per_pixel,
                    )
                }
                .0 as u32,
            )
        })
    }
}
