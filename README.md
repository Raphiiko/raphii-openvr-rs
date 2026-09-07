# raphii-openvr-rs

My personal Rust wrapper for OpenVR, for use in projects such as [OyasumiVR](https://github.com/Raphiiko/OyasumiVR) and VRTI.

It wraps devices, settings, overlays, input actions and application registration. The Windows x64, Linux x64 and Linux ARM64 bindings use Valve's C function tables. They are checked in, so building a project that uses this library needs no C++ compiler, LLVM or libclang.

## Usage

Requires Rust 1.85 or later. Add this to `Cargo.toml`:

```toml
[dependencies]
raphii-openvr-rs = { git = "https://github.com/Raphiiko/raphii-openvr-rs", branch = "main" }
```

Use `rev` instead of `branch` to pin a commit. `libloading` is the only direct dependency.

Install SteamVR separately. Supply the OpenVR native library for your target from the [pinned OpenVR SDK 2.15.6](https://github.com/ValveSoftware/openvr/tree/0924064316de3effbcd1acf1e309182a2deb1c05/bin):

- **Windows:** copy `win64/openvr_api.dll` beside your executable. For `cargo run`, that is normally `target/debug/openvr_api.dll`.
- **Linux:** use `linux64/libopenvr_api.so` for x64 or `linuxarm64/libopenvr_api.so` for ARM64. Make that directory available to the system loader. For example, on x64:

  ```sh
  LD_LIBRARY_PATH=/path/to/openvr/bin/linux64 cargo run
  ```

This crate does not bundle or download those files. `unsafe Context::init_from_path` accepts a custom path; the library you load must implement Valve's OpenVR ABI.

Start SteamVR, then run this example to read the headset model:

```rust,no_run
use raphii_openvr_rs::{Context, TrackedDeviceIndex, raw};

fn main() -> Result<(), raphii_openvr_rs::Error> {
    let context = Context::init(raw::EVRApplicationType::VRApplication_Background)?;
    let model: String = context.system().get_tracked_device_property(
        TrackedDeviceIndex::HMD,
        raw::ETrackedDeviceProperty::Prop_ModelNumber_String,
    )?;
    println!("Headset: {model}");
    Ok(())
}
```

Dropping the last context shuts down the OpenVR session. A missing native library or export returns `Error::Load`. Runtime failures retain their numeric OpenVR error code.

## API

| Accessor | Operations |
|---|---|
| `context.system()` | Device properties, classes, roles, activity, poses and events |
| `context.settings()` | Read and write float, boolean, integer and string settings; remove keys and sections |
| `context.overlays()` | Create and destroy overlays; set pixels, transforms, width, opacity, sort order and visibility |
| `context.input()` | Load action manifests, update actions, read digital input, inspect bindings and open the binding editor |
| `context.applications()` | Register and remove application manifests; check registration |

There is no safe compositor or graphics-submission API. The `raw` module exposes all 24 SDK function tables; calling them requires `unsafe`. Enum values use integer newtypes, so callers must handle values they do not recognize.

Build the API reference locally with `cargo doc --no-deps --open`.

## Sessions and compatibility

Initialize once per process and clone the context to share it. A second initialization returns `AlreadyInitialized`. Do not let another OpenVR wrapper initialize or shut down the same runtime.

Calls through shared contexts are serialized. Calling `context.shutdown()` invalidates every clone; later calls return `NotInitialized`. You can then initialize a new session. Discard old action and overlay handles. Destroy overlays explicitly when finished with them, or let session shutdown release them.

Your application handles SteamVR quit events and reconnect timing. This crate does not start or restart SteamVR for you.

Bindings use OpenVR SDK 2.15.6. SDK and SteamVR version numbers need not match, but the runtime must provide the requested interface versions. The system interface is required. Missing settings, overlay, input or applications interfaces return `InterfaceUnavailable` when used.

## Maintenance

See [the maintainer notes](maintainer/README.md) for regenerating bindings, running the native tests and updating the SDK.

## License

[MIT](LICENSE). Valve headers and derived bindings retain Valve's [BSD-3-Clause license](vendor/LICENSE-Valve).
