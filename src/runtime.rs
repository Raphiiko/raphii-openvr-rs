use crate::{Error, Result, raw};
use libloading::Library;
use std::{
    ffi::CStr,
    path::Path,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
};

static INITIALIZED: AtomicBool = AtomicBool::new(false);
static NEXT_SESSION: AtomicU64 = AtomicU64::new(1);
struct Reservation;
impl Drop for Reservation {
    fn drop(&mut self) {
        INITIALIZED.store(false, Ordering::Release);
    }
}

pub(crate) struct Session {
    pub id: u64,
    pub system: Result<raw::VR_IVRSystem_FnTable>,
    pub settings: Result<raw::VR_IVRSettings_FnTable>,
    pub overlay: Result<raw::VR_IVROverlay_FnTable>,
    pub input: Result<raw::VR_IVRInput_FnTable>,
    pub applications: Result<raw::VR_IVRApplications_FnTable>,
    shutdown: raw::exports::OpenvrShutdown,
    _library: Library,
    _reservation: Reservation,
}
impl Drop for Session {
    fn drop(&mut self) {
        if let Some(shutdown) = self.shutdown {
            unsafe { shutdown() };
        }
    }
}

/// Clones share one session. Shutdown invalidates all clones and managers atomically.
#[derive(Clone)]
pub struct Context(Arc<Mutex<Option<Session>>>);

impl Context {
    /// Loads openvr_api.dll beside the executable on Windows, or libopenvr_api.so on Linux.
    pub fn init(application_type: raw::EVRApplicationType) -> Result<Self> {
        #[cfg(target_os = "windows")]
        let path = std::env::current_exe()
            .map_err(|e| Error::Load(e.to_string()))?
            .with_file_name("openvr_api.dll");
        #[cfg(target_os = "linux")]
        let path = std::path::PathBuf::from("libopenvr_api.so");
        unsafe { Self::init_from_path(&path, application_type) }
    }

    /// # Safety
    /// The library must implement Valve's OpenVR ABI; no other code may initialize or shut it down.
    pub unsafe fn init_from_path(
        path: &Path,
        application_type: raw::EVRApplicationType,
    ) -> Result<Self> {
        INITIALIZED
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .map_err(|_| Error::AlreadyInitialized)?;
        let reservation = Reservation;
        let library = unsafe { Library::new(path) }.map_err(|e| Error::Load(e.to_string()))?;
        let init = *unsafe { library.get::<raw::exports::OpenvrInit>(b"VR_InitInternal2\0") }
            .map_err(|e| Error::Load(e.to_string()))?;
        let shutdown =
            *unsafe { library.get::<raw::exports::OpenvrShutdown>(b"VR_ShutdownInternal\0") }
                .map_err(|e| Error::Load(e.to_string()))?;
        let get = *unsafe {
            library.get::<raw::exports::OpenvrGetInterface>(b"VR_GetGenericInterface\0")
        }
        .map_err(|e| Error::Load(e.to_string()))?;
        let mut error = raw::exports::vr_EVRInitError(0);
        unsafe {
            init.ok_or(Error::MissingFunction("VR_InitInternal2"))?(
                &mut error,
                raw::exports::vr_EVRApplicationType(application_type.0 as _),
                std::ptr::null(),
            )
        };
        crate::check("initialization", error.0 as u32)?;
        let mut session = Session {
            id: NEXT_SESSION.fetch_add(1, Ordering::Relaxed),
            system: Err(Error::NotInitialized),
            settings: Err(Error::NotInitialized),
            overlay: Err(Error::NotInitialized),
            input: Err(Error::NotInitialized),
            applications: Err(Error::NotInitialized),
            shutdown,
            _library: library,
            _reservation: reservation,
        };
        macro_rules! acquire {
            ($field:ident, $ty:ty, $version:ident) => {
                session.$field =
                    unsafe { acquire::<$ty>(get, raw::$version, stringify!($version)) };
            };
        }
        acquire!(system, raw::VR_IVRSystem_FnTable, IVRSystem_Version);
        acquire!(settings, raw::VR_IVRSettings_FnTable, IVRSettings_Version);
        acquire!(overlay, raw::VR_IVROverlay_FnTable, IVROverlay_Version);
        acquire!(input, raw::VR_IVRInput_FnTable, IVRInput_Version);
        acquire!(
            applications,
            raw::VR_IVRApplications_FnTable,
            IVRApplications_Version
        );
        session.system.as_ref().map_err(Clone::clone)?;
        Ok(Self(Arc::new(Mutex::new(Some(session)))))
    }

    /// Stops the session once; managers from this session subsequently return NotInitialized.
    pub fn shutdown(&self) {
        self.0.lock().unwrap_or_else(|e| e.into_inner()).take();
    }

    pub(crate) fn with<T>(&self, f: impl FnOnce(&Session) -> Result<T>) -> Result<T> {
        let guard = self.0.lock().unwrap_or_else(|e| e.into_inner());
        f(guard.as_ref().ok_or(Error::NotInitialized)?)
    }
    pub fn overlay_interface_available(&self) -> bool {
        self.with(|s| s.overlay.as_ref().map(|_| ()).map_err(Clone::clone))
            .is_ok()
    }
    pub fn settings_interface_available(&self) -> bool {
        self.with(|s| s.settings.as_ref().map(|_| ()).map_err(Clone::clone))
            .is_ok()
    }
    pub fn input_interface_available(&self) -> bool {
        self.with(|s| s.input.as_ref().map(|_| ()).map_err(Clone::clone))
            .is_ok()
    }
    pub fn system(&self) -> crate::system::System<'_> {
        crate::system::System(self)
    }
    pub fn settings(&self) -> crate::settings::Settings<'_> {
        crate::settings::Settings(self)
    }
    pub fn overlays(&self) -> crate::overlay::Overlays<'_> {
        crate::overlay::Overlays(self)
    }
    pub fn input(&self) -> crate::input::Input<'_> {
        crate::input::Input(self)
    }
    pub fn applications(&self) -> crate::applications::Applications<'_> {
        crate::applications::Applications(self)
    }
}

unsafe fn acquire<T: Copy>(
    get: raw::exports::OpenvrGetInterface,
    version: &[u8],
    name: &'static str,
) -> Result<T> {
    let mut version_string = b"FnTable:".to_vec();
    version_string.extend_from_slice(version);
    let version = CStr::from_bytes_with_nul(&version_string)
        .map_err(|_| Error::InvalidInput("invalid interface version"))?;
    let mut error = raw::exports::vr_EVRInitError(0);
    let ptr = unsafe {
        get.ok_or(Error::MissingFunction("VR_GetGenericInterface"))?(version.as_ptr(), &mut error)
    };
    if error.0 != 0 || ptr.is_null() {
        return Err(Error::InterfaceUnavailable {
            interface: name,
            code: error.0 as u32,
        });
    }
    if (ptr as usize) % std::mem::align_of::<T>() != 0 {
        return Err(Error::InvalidResponse("unaligned interface table"));
    }
    Ok(unsafe { ptr.cast::<T>().read() })
}
