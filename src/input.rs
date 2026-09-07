use crate::{Context, Error, Result, check, cstring, function, path_string, raw};
use std::path::Path;

pub struct Input<'a>(pub(crate) &'a Context);
#[derive(Clone, Copy, Debug)]
pub struct ActionHandle(pub u64, u64);
#[derive(Clone, Copy, Debug)]
pub struct ActionSetHandle(pub u64, u64);
/// A runtime input-source or origin ID; zero means unrestricted where supported.
#[derive(Clone, Copy, Debug)]
pub struct InputValueHandle(pub u64);
#[derive(Clone, Copy)]
pub struct ActiveActionSet {
    raw: raw::VRActiveActionSet_t,
    session: u64,
}
impl ActiveActionSet {
    pub fn new(set: ActionSetHandle) -> Self {
        Self {
            raw: raw::VRActiveActionSet_t {
                ulActionSet: set.0,
                ..Default::default()
            },
            session: set.1,
        }
    }
    pub fn restrict_to(mut self, device: InputValueHandle) -> Self {
        self.raw.ulRestrictedToDevice = device.0;
        self
    }
    pub fn priority(mut self, priority: i32) -> Self {
        self.raw.nPriority = priority;
        self
    }
}
pub struct DigitalActionData(pub raw::InputDigitalActionData_t);
pub struct OriginInfo(pub raw::InputOriginInfo_t);

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum InputString {
    Hand = 1,
    ControllerType = 2,
    InputSource = 4,
}

fn validate(handle_session: u64, session: u64) -> Result<()> {
    if handle_session == session {
        Ok(())
    } else {
        Err(Error::InvalidInput("action belongs to a different session"))
    }
}
macro_rules! handle {
    ($name:ident, $method:ident, $ty:ident) => {
        pub fn $name(&self, name: &str) -> Result<$ty> {
            let name = cstring(name)?;
            self.0.with(|s| {
                let table = s.input.as_ref().map_err(Clone::clone)?;
                let mut value = 0;
                check(
                    "input",
                    unsafe { function!(table, $method)(name.as_ptr().cast_mut(), &mut value) }.0
                        as u32,
                )?;
                if value == 0 {
                    return Err(Error::InvalidResponse(
                        "runtime returned an invalid action handle",
                    ));
                }
                Ok($ty(value, s.id))
            })
        }
    };
}
impl Input<'_> {
    pub fn set_action_manifest(&self, path: &Path) -> Result<()> {
        let path = path_string(path)?;
        self.0.with(|s| {
            let table = s.input.as_ref().map_err(Clone::clone)?;
            check(
                "input",
                unsafe { function!(table, SetActionManifestPath)(path.as_ptr().cast_mut()) }.0
                    as u32,
            )
        })
    }
    handle!(get_action_handle, GetActionHandle, ActionHandle);
    handle!(get_action_set_handle, GetActionSetHandle, ActionSetHandle);
    pub fn get_input_source_handle(&self, name: &str) -> Result<InputValueHandle> {
        let name = cstring(name)?;
        self.0.with(|s| {
            let table = s.input.as_ref().map_err(Clone::clone)?;
            let mut value = 0;
            check(
                "input",
                unsafe {
                    function!(table, GetInputSourceHandle)(name.as_ptr().cast_mut(), &mut value)
                }
                .0 as u32,
            )?;
            Ok(InputValueHandle(value))
        })
    }
    pub fn update_actions(&self, sets: &[ActiveActionSet]) -> Result<()> {
        let count =
            u32::try_from(sets.len()).map_err(|_| Error::InvalidInput("too many action sets"))?;
        self.0.with(|s| {
            for set in sets {
                validate(set.session, s.id)?;
            }
            let mut sets: Vec<_> = sets.iter().map(|set| set.raw).collect();
            let table = s.input.as_ref().map_err(Clone::clone)?;
            check(
                "input",
                unsafe {
                    function!(table, UpdateActionState)(
                        sets.as_mut_ptr(),
                        std::mem::size_of::<raw::VRActiveActionSet_t>() as u32,
                        count,
                    )
                }
                .0 as u32,
            )
        })
    }
    pub fn get_digital_action_data(
        &self,
        action: ActionHandle,
        restrict: InputValueHandle,
    ) -> Result<DigitalActionData> {
        self.0.with(|s| {
            validate(action.1, s.id)?;
            let table = s.input.as_ref().map_err(Clone::clone)?;
            let mut data = raw::InputDigitalActionData_t::default();
            check(
                "input",
                unsafe {
                    function!(table, GetDigitalActionData)(
                        action.0,
                        &mut data,
                        std::mem::size_of_val(&data) as u32,
                        restrict.0,
                    )
                }
                .0 as u32,
            )?;
            Ok(DigitalActionData(data))
        })
    }
    pub fn get_action_origins(
        &self,
        set: ActionSetHandle,
        action: ActionHandle,
    ) -> Result<[u64; raw::k_unMaxActionOriginCount as usize]> {
        self.0.with(|s| {
            validate(set.1, s.id)?;
            validate(action.1, s.id)?;
            let table = s.input.as_ref().map_err(Clone::clone)?;
            let mut data = [0; raw::k_unMaxActionOriginCount as usize];
            check(
                "input",
                unsafe {
                    function!(table, GetActionOrigins)(
                        set.0,
                        action.0,
                        data.as_mut_ptr(),
                        data.len() as u32,
                    )
                }
                .0 as u32,
            )?;
            Ok(data)
        })
    }
    pub fn get_origin_tracked_device_info(&self, origin: InputValueHandle) -> Result<OriginInfo> {
        self.0.with(|s| {
            let table = s.input.as_ref().map_err(Clone::clone)?;
            let mut data = raw::InputOriginInfo_t::default();
            check(
                "input",
                unsafe {
                    function!(table, GetOriginTrackedDeviceInfo)(
                        origin.0,
                        &mut data,
                        std::mem::size_of_val(&data) as u32,
                    )
                }
                .0 as u32,
            )?;
            Ok(OriginInfo(data))
        })
    }
    pub fn get_origin_localized_name(
        &self,
        origin: InputValueHandle,
        sections: &[InputString],
    ) -> Result<String> {
        let bits = sections
            .iter()
            .fold(0, |bits, section| bits | *section as i32);
        self.0.with(|s| {
            let table = s.input.as_ref().map_err(Clone::clone)?;
            for size in [128, 512, 2048, 32768] {
                let mut bytes = vec![0xff; size];
                let error = unsafe {
                    function!(table, GetOriginLocalizedName)(
                        origin.0,
                        bytes.as_mut_ptr().cast(),
                        size as u32,
                        bits,
                    )
                };
                if error == raw::EVRInputError::VRInputError_BufferTooSmall {
                    continue;
                }
                check("input", error.0 as u32)?;
                return crate::string_buffer(&bytes);
            }
            Err(Error::InvalidResponse("origin name exceeds 32768 bytes"))
        })
    }
    pub fn get_action_binding_info(
        &self,
        action: ActionHandle,
    ) -> Result<Vec<raw::InputBindingInfo_t>> {
        self.0.with(|s| {
            validate(action.1, s.id)?;
            let table = s.input.as_ref().map_err(Clone::clone)?;
            let mut data = vec![raw::InputBindingInfo_t::default(); 16];
            for _ in 0..4 {
                let mut count = 0;
                let error = unsafe {
                    function!(table, GetActionBindingInfo)(
                        action.0,
                        data.as_mut_ptr(),
                        std::mem::size_of::<raw::InputBindingInfo_t>() as u32,
                        data.len() as u32,
                        &mut count,
                    )
                };
                if count > 4096 {
                    return Err(Error::InvalidResponse("binding count exceeds 4096"));
                }
                if error == raw::EVRInputError::VRInputError_BufferTooSmall
                    && count as usize > data.len()
                {
                    data.resize(count as usize, raw::InputBindingInfo_t::default());
                    continue;
                }
                check("input", error.0 as u32)?;
                if count as usize > data.len() {
                    return Err(Error::InvalidResponse("binding count exceeds buffer"));
                }
                data.truncate(count as usize);
                return Ok(data);
            }
            Err(Error::InvalidResponse("binding count keeps growing"))
        })
    }
    pub fn open_binding_ui(
        &self,
        app_key: Option<&str>,
        set: Option<ActionSetHandle>,
        device: InputValueHandle,
        desktop: bool,
    ) -> Result<()> {
        let key = app_key.map(cstring).transpose()?;
        self.0.with(|s| {
            if let Some(set) = set {
                validate(set.1, s.id)?;
            }
            let table = s.input.as_ref().map_err(Clone::clone)?;
            check(
                "input",
                unsafe {
                    function!(table, OpenBindingUI)(
                        key.as_ref()
                            .map_or(std::ptr::null_mut(), |s| s.as_ptr().cast_mut()),
                        set.map_or(0, |s| s.0),
                        device.0,
                        desktop,
                    )
                }
                .0 as u32,
            )
        })
    }
}

impl From<raw::EVRInputError> for Error {
    fn from(error: raw::EVRInputError) -> Self {
        Self::Runtime {
            interface: "input",
            code: error.0 as u32,
        }
    }
}
