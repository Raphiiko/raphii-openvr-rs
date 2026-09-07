use crate::{Context, Error, Result, TrackedDeviceIndex, check, function, raw};

pub struct System<'a>(pub(crate) &'a Context);
mod private {
    pub trait Sealed {}
}
pub trait TrackedDeviceProperty: private::Sealed + Sized {
    fn get(
        system: &System<'_>,
        index: TrackedDeviceIndex,
        property: raw::ETrackedDeviceProperty,
    ) -> Result<Self>;
}
macro_rules! property {
    ($ty:ty, $method:ident) => {
        impl private::Sealed for $ty {}
        impl TrackedDeviceProperty for $ty {
            fn get(
                system: &System<'_>,
                index: TrackedDeviceIndex,
                property: raw::ETrackedDeviceProperty,
            ) -> Result<Self> {
                TrackedDeviceIndex::new(index.0)?;
                system.0.with(|s| {
                    let table = s.system.as_ref().map_err(Clone::clone)?;
                    let mut error = raw::ETrackedPropertyError(0);
                    let value = unsafe { function!(table, $method)(index.0, property, &mut error) };
                    check("property", error.0 as u32)?;
                    Ok(value)
                })
            }
        }
    };
}
property!(bool, GetBoolTrackedDeviceProperty);
property!(f32, GetFloatTrackedDeviceProperty);
property!(i32, GetInt32TrackedDeviceProperty);
property!(u64, GetUint64TrackedDeviceProperty);

impl private::Sealed for String {}
impl TrackedDeviceProperty for String {
    fn get(
        system: &System<'_>,
        index: TrackedDeviceIndex,
        property: raw::ETrackedDeviceProperty,
    ) -> Result<Self> {
        TrackedDeviceIndex::new(index.0)?;
        system.0.with(|s| {
            let table = s.system.as_ref().map_err(Clone::clone)?;
            let get = function!(table, GetStringTrackedDeviceProperty);
            let mut data = vec![0xff; 256];
            for _ in 0..4 {
                let mut error = raw::ETrackedPropertyError(0);
                let size = unsafe {
                    get(
                        index.0,
                        property,
                        data.as_mut_ptr().cast(),
                        data.len() as u32,
                        &mut error,
                    )
                } as usize;
                if size > raw::k_unMaxPropertyStringSize as usize {
                    return Err(Error::InvalidResponse("property exceeds SDK string limit"));
                }
                if error == raw::ETrackedPropertyError::TrackedProp_BufferTooSmall
                    && size > data.len()
                {
                    data.resize(size, 0xff);
                    continue;
                }
                check("property", error.0 as u32)?;
                if size == 0 || size > data.len() {
                    return Err(Error::InvalidResponse("invalid property string length"));
                }
                return crate::string_buffer(&data[..size]);
            }
            Err(Error::InvalidResponse("property string keeps growing"))
        })
    }
}

macro_rules! device_value {
    ($name:ident, $method:ident, $ty:ty) => {
        pub fn $name(&self, index: TrackedDeviceIndex) -> Result<$ty> {
            TrackedDeviceIndex::new(index.0)?;
            self.0.with(|s| {
                let table = s.system.as_ref().map_err(Clone::clone)?;
                Ok(unsafe { function!(table, $method)(index.0) })
            })
        }
    };
}
impl System<'_> {
    pub fn get_tracked_device_property<T: TrackedDeviceProperty>(
        &self,
        index: TrackedDeviceIndex,
        property: raw::ETrackedDeviceProperty,
    ) -> Result<T> {
        T::get(self, index, property)
    }
    device_value!(
        get_tracked_device_class,
        GetTrackedDeviceClass,
        raw::ETrackedDeviceClass
    );
    device_value!(
        get_controller_role_for_tracked_device_index,
        GetControllerRoleForTrackedDeviceIndex,
        raw::ETrackedControllerRole
    );
    device_value!(
        get_tracked_device_activity_level,
        GetTrackedDeviceActivityLevel,
        raw::EDeviceActivityLevel
    );

    pub fn get_device_to_absolute_tracking_pose(
        &self,
        origin: raw::ETrackingUniverseOrigin,
        prediction_seconds: f32,
    ) -> Result<[raw::TrackedDevicePose_t; raw::k_unMaxTrackedDeviceCount as usize]> {
        if !prediction_seconds.is_finite() {
            return Err(Error::InvalidInput("pose prediction must be finite"));
        }
        self.0.with(|s| {
            let table = s.system.as_ref().map_err(Clone::clone)?;
            let mut poses =
                [raw::TrackedDevicePose_t::default(); raw::k_unMaxTrackedDeviceCount as usize];
            unsafe {
                function!(table, GetDeviceToAbsoluteTrackingPose)(
                    origin,
                    prediction_seconds,
                    poses.as_mut_ptr(),
                    poses.len() as u32,
                )
            };
            Ok(poses)
        })
    }
    pub fn poll_next_event(&self) -> Result<Option<VREvent>> {
        self.0.with(|s| {
            let table = s.system.as_ref().map_err(Clone::clone)?;
            let mut event = raw::VREvent_t::default();
            let available = unsafe {
                function!(table, PollNextEvent)(&mut event, std::mem::size_of_val(&event) as u32)
            };
            Ok(available.then_some(VREvent(event)))
        })
    }
}

/// Unknown event IDs remain available through event_type().
pub struct VREvent(raw::VREvent_t);
impl VREvent {
    pub fn event_type(&self) -> raw::EVREventType {
        raw::EVREventType(self.0.eventType as _)
    }
    pub fn is(&self, event_type: raw::EVREventType) -> bool {
        self.event_type() == event_type
    }
    pub fn tracked_device_index(&self) -> TrackedDeviceIndex {
        TrackedDeviceIndex(self.0.trackedDeviceIndex)
    }
    pub fn age_seconds(&self) -> f32 {
        self.0.eventAgeSeconds
    }
    pub fn changed_property(&self) -> Option<raw::ETrackedDeviceProperty> {
        if !self.is(raw::EVREventType::VREvent_PropertyChanged) {
            return None;
        }
        Some(unsafe { self.0.data.property.prop })
    }
}
