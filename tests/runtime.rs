use raphii_openvr_rs::{
    Context, Error, TrackedDeviceIndex,
    input::{ActiveActionSet, InputString, InputValueHandle},
    pose::Matrix3x4,
    raw,
};
use std::path::{Path, PathBuf};

#[test]
#[ignore = "run maintainer/verify.py to build the fake runtime first"]
fn simulated_runtime() {
    let missing = PathBuf::from(
        std::env::var_os("RAPHII_OPENVR_TEST_MISSING_SYMBOL").expect("missing-symbol library path"),
    );
    assert!(matches!(
        unsafe {
            Context::init_from_path(&missing, raw::EVRApplicationType::VRApplication_Background)
        },
        Err(Error::Load(_))
    ));
    let path =
        PathBuf::from(std::env::var_os("RAPHII_OPENVR_TEST_RUNTIME").expect("fake runtime path"));
    let control = unsafe { libloading::Library::new(&path) }.unwrap();
    let mode = unsafe {
        *control
            .get::<unsafe extern "C" fn(i32)>(b"TestMode\0")
            .unwrap()
    };
    let count = unsafe {
        *control
            .get::<unsafe extern "C" fn(i32) -> i32>(b"TestCount\0")
            .unwrap()
    };
    let set_mode = |value| unsafe { mode(value) };
    let count = |kind| unsafe { count(kind) };
    let init = || unsafe {
        Context::init_from_path(&path, raw::EVRApplicationType::VRApplication_Background)
    };

    assert!(matches!(
        unsafe {
            Context::init_from_path(
                Path::new("/not/a/runtime/library"),
                raw::EVRApplicationType::VRApplication_Background,
            )
        },
        Err(Error::Load(_))
    ));
    set_mode(1);
    assert!(matches!(init(), Err(Error::Runtime { code: 9999, .. })));
    assert_eq!(count(1), 0);
    set_mode(2);
    assert!(matches!(init(), Err(Error::InterfaceUnavailable { .. })));
    assert_eq!(count(1), 1);
    set_mode(3);
    let context = init().unwrap();
    assert!(!context.settings_interface_available());
    assert!(matches!(
        context.settings().get_float(c"s", c"k"),
        Err(Error::InterfaceUnavailable { .. })
    ));
    context.shutdown();
    set_mode(6);
    let context = init().unwrap();
    assert!(matches!(
        context.settings().get_float(c"s", c"k"),
        Err(Error::MissingFunction("GetFloat"))
    ));
    context.shutdown();
    set_mode(12);
    let context = init().unwrap();
    assert!(!context.overlay_interface_available());
    assert!(!context.input_interface_available());
    assert!(matches!(
        context.overlays().is_dashboard_visible(),
        Err(Error::InterfaceUnavailable { .. })
    ));
    assert!(matches!(
        context.input().get_action_handle("a"),
        Err(Error::InterfaceUnavailable { .. })
    ));
    assert!(matches!(
        context.applications().is_application_installed("a"),
        Err(Error::InterfaceUnavailable { .. })
    ));
    context.shutdown();

    set_mode(0);
    let context = init().unwrap();
    let before = count(0);
    assert!(matches!(init(), Err(Error::AlreadyInitialized)));
    assert_eq!(count(0), before);
    let clone = context.clone();
    let settings = clone.settings();
    settings.set_float(c"steamvr", c"analogGain", 0.7).unwrap();
    assert_eq!(settings.get_float(c"steamvr", c"analogGain").unwrap(), 0.7);
    settings.set_bool(c"s", c"k", true).unwrap();
    assert!(settings.get_bool(c"s", c"k").unwrap());
    settings.set_int32(c"s", c"k", -3).unwrap();
    assert_eq!(settings.get_int32(c"s", c"k").unwrap(), -3);
    settings.set_string(c"s", c"k", c"hello").unwrap();
    assert_eq!(settings.get_string(c"s", c"k").unwrap(), "hello");
    settings.remove_key_in_section(c"s", c"k").unwrap();
    settings.remove_section(c"s").unwrap();
    set_mode(4);
    assert!(matches!(
        settings.get_float(c"s", c"k"),
        Err(Error::Runtime { code: 9999, .. })
    ));

    let system = context.system();
    let property = raw::ETrackedDeviceProperty::Prop_SerialNumber_String;
    assert!(matches!(
        system.get_tracked_device_property::<f32>(TrackedDeviceIndex(0), property),
        Err(Error::Runtime { code: 9999, .. })
    ));
    set_mode(0);
    assert_eq!(
        system
            .get_tracked_device_property::<String>(TrackedDeviceIndex(0), property)
            .unwrap(),
        "serial"
    );
    assert_eq!(
        system
            .get_tracked_device_property::<f32>(TrackedDeviceIndex(0), property)
            .unwrap(),
        0.75
    );
    assert!(
        system
            .get_tracked_device_property::<bool>(TrackedDeviceIndex(0), property)
            .unwrap()
    );
    assert_eq!(
        system
            .get_tracked_device_property::<i32>(TrackedDeviceIndex(0), property)
            .unwrap(),
        -42
    );
    assert_eq!(
        system
            .get_tracked_device_property::<u64>(TrackedDeviceIndex(0), property)
            .unwrap(),
        0xfedcba9876543210
    );
    set_mode(8);
    assert_eq!(
        system
            .get_tracked_device_property::<String>(TrackedDeviceIndex(0), property)
            .unwrap()
            .len(),
        599
    );
    for value in [9, 10] {
        set_mode(value);
        assert!(matches!(
            system.get_tracked_device_property::<String>(TrackedDeviceIndex(0), property),
            Err(Error::InvalidResponse(_))
        ));
    }
    set_mode(9);
    assert!(matches!(
        settings.get_string(c"s", c"k"),
        Err(Error::InvalidResponse(_))
    ));
    set_mode(7);
    assert_eq!(
        system
            .get_tracked_device_class(TrackedDeviceIndex(0))
            .unwrap()
            .0,
        9999
    );
    assert_eq!(
        system
            .get_controller_role_for_tracked_device_index(TrackedDeviceIndex(0))
            .unwrap()
            .0,
        9999
    );
    assert_eq!(
        system
            .get_tracked_device_activity_level(TrackedDeviceIndex(0))
            .unwrap()
            .0,
        9999
    );
    assert!(
        system
            .get_tracked_device_class(TrackedDeviceIndex(64))
            .is_err()
    );
    let poses = system
        .get_device_to_absolute_tracking_pose(
            raw::ETrackingUniverseOrigin::TrackingUniverseStanding,
            0.0,
        )
        .unwrap();
    assert_eq!(poses.len(), 64);
    assert!(poses[0].bDeviceIsConnected && poses[0].bPoseIsValid);
    assert_eq!(poses[0].mDeviceToAbsoluteTracking.m[0][3], 1.25);
    set_mode(0);
    let event = system.poll_next_event().unwrap().unwrap();
    assert!(event.is(raw::EVREventType::VREvent_PropertyChanged));
    assert_eq!(event.tracked_device_index().0, 3);
    assert_eq!(event.age_seconds(), 0.25);
    assert_eq!(
        event.changed_property().unwrap(),
        raw::ETrackedDeviceProperty::Prop_DeviceBatteryPercentage_Float
    );
    let event = system.poll_next_event().unwrap().unwrap();
    assert_eq!(event.event_type().0, 0x7ffffffe);
    assert!(event.changed_property().is_none());
    assert!(
        system
            .poll_next_event()
            .unwrap()
            .unwrap()
            .is(raw::EVREventType::VREvent_Quit)
    );
    assert!(system.poll_next_event().unwrap().is_none());

    let overlay = context.overlays();
    assert!(overlay.create_overlay("bad\0key", "name").is_err());
    let handle = overlay.create_overlay("key", "name").unwrap();
    assert!(overlay.set_raw_data(handle, &[0; 3], 1, 1, 4).is_err());
    assert!(
        overlay
            .set_raw_data(handle, &[], u32::MAX, u32::MAX, 4)
            .is_err()
    );
    overlay
        .set_raw_data(handle, &[0, 0, 0, 255], 1, 1, 4)
        .unwrap();
    overlay.set_opacity(handle, 0.8).unwrap();
    overlay.set_width(handle, 1.0).unwrap();
    overlay.set_sort_order(handle, 200).unwrap();
    overlay.set_visibility(handle, true).unwrap();
    assert!(overlay.is_dashboard_visible().unwrap());
    overlay.set_visibility(handle, false).unwrap();
    assert!(!overlay.is_dashboard_visible().unwrap());
    overlay
        .set_transform_tracked_device_relative(
            handle,
            TrackedDeviceIndex::HMD,
            &Matrix3x4([[1., 0., 0., 0.], [0., 1., 0., 0.], [0., 0., 1., -0.15]]),
        )
        .unwrap();

    let input = context.input();
    input
        .set_action_manifest(Path::new("manifest.json"))
        .unwrap();
    let action = input.get_action_handle("/actions/main/in/a").unwrap();
    let set = input.get_action_set_handle("/actions/main").unwrap();
    let source = input.get_input_source_handle("/user/hand/left").unwrap();
    let sets = [ActiveActionSet::new(set)];
    input.update_actions(&sets).unwrap();
    let data = input
        .get_digital_action_data(action, InputValueHandle(0))
        .unwrap()
        .0;
    assert!(data.bActive && data.bChanged && data.bState);
    assert_eq!(data.activeOrigin, source.0);
    assert_eq!(data.fUpdateTime, -0.125);
    assert_eq!(input.get_action_origins(set, action).unwrap()[0], 42);
    assert_eq!(
        input
            .get_origin_tracked_device_info(source)
            .unwrap()
            .0
            .trackedDeviceIndex,
        3
    );
    for (section, expected) in [
        (InputString::Hand, "1"),
        (InputString::ControllerType, "2"),
        (InputString::InputSource, "4"),
    ] {
        assert_eq!(
            input.get_origin_localized_name(source, &[section]).unwrap(),
            expected
        );
    }
    input.open_binding_ui(None, None, source, true).unwrap();
    input
        .open_binding_ui(Some("test.app"), Some(set), source, false)
        .unwrap();
    assert!(
        input
            .open_binding_ui(Some("bad\0key"), None, source, false)
            .is_err()
    );
    assert_eq!(input.get_action_binding_info(action).unwrap().len(), 1);
    set_mode(8);
    assert_eq!(input.get_action_binding_info(action).unwrap().len(), 20);
    assert_eq!(
        input
            .get_origin_localized_name(source, &[InputString::Hand])
            .unwrap(),
        "1"
    );
    set_mode(9);
    assert!(matches!(
        input.get_origin_localized_name(source, &[InputString::Hand]),
        Err(Error::InvalidResponse(_))
    ));
    set_mode(11);
    assert!(matches!(
        input.get_action_binding_info(action),
        Err(Error::InvalidResponse(_))
    ));
    set_mode(0);
    let applications = context.applications();
    applications
        .add_application_manifest(Path::new("manifest.json"), false)
        .unwrap();
    assert!(applications.is_application_installed("test.app").unwrap());
    applications
        .remove_application_manifest(Path::new("manifest.json"))
        .unwrap();
    assert!(!applications.is_application_installed("test.app").unwrap());
    overlay.destroy_overlay(handle).unwrap();

    let mut workers = vec![];
    for _ in 0..4 {
        let ctx = context.clone();
        workers.push(std::thread::spawn(move || {
            for _ in 0..100 {
                let result = ctx.settings().get_float(c"s", c"k");
                assert!(result.is_ok() || result == Err(Error::NotInitialized));
            }
        }));
    }
    let shutdowns = count(1);
    context.shutdown();
    for worker in workers {
        worker.join().unwrap();
    }
    assert_eq!(count(1), shutdowns + 1);
    assert_eq!(settings.get_float(c"s", c"k"), Err(Error::NotInitialized));
    let calls = count(2);
    clone.shutdown();
    assert_eq!(count(2), calls);
    let new_context = init().unwrap();
    assert!(matches!(
        new_context.overlays().set_opacity(handle, 0.5),
        Err(Error::InvalidInput(_))
    ));
    assert!(matches!(
        new_context.input().get_digital_action_data(action, source),
        Err(Error::InvalidInput(_))
    ));
    assert!(matches!(
        new_context.input().update_actions(&sets),
        Err(Error::InvalidInput(_))
    ));
    clone.shutdown();
    assert!(new_context.settings().get_float(c"s", c"k").is_ok());
    drop(new_context);
    assert_eq!(count(1), shutdowns + 2);
    drop(init().unwrap());
    assert_eq!(count(1), shutdowns + 3);

    let barrier = std::sync::Arc::new(std::sync::Barrier::new(8));
    let mut attempts = vec![];
    for _ in 0..8 {
        let path = path.clone();
        let barrier = barrier.clone();
        attempts.push(std::thread::spawn(move || {
            barrier.wait();
            unsafe {
                Context::init_from_path(&path, raw::EVRApplicationType::VRApplication_Background)
            }
        }));
    }
    let results: Vec<_> = attempts
        .into_iter()
        .map(|attempt| attempt.join().unwrap())
        .collect();
    assert_eq!(results.iter().filter(|result| result.is_ok()).count(), 1);
    assert_eq!(
        results
            .iter()
            .filter(|result| matches!(result, Err(Error::AlreadyInitialized)))
            .count(),
        7
    );
    drop(results);

    let context = init().unwrap();
    drop(control);
    assert!(context.settings().get_float(c"s", c"k").is_ok());
    context.shutdown();
}
