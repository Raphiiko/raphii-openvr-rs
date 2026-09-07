# Maintaining the wrapper

Run these commands from the repository root. Binding generation is separate from normal Rust builds.

## Run the tests

The native tests load a fake OpenVR library and compare Rust types with Valve's C and C++ headers. They do not use SteamVR or a headset.

On Windows, install Python, Rust with the MSVC target, and Visual Studio Build Tools with the C++ workload. The script finds the compiler through `vswhere`.

```powershell
python maintainer/verify.py
cargo test --doc --locked
```

`cargo test` alone skips the native test because it needs the fake library built first. `verify.py` builds that library and runs the test. It writes its files under `test-output/`.

For Linux, use the Docker image. These commands are for PowerShell and work from any checkout location:

```powershell
$repo = (Get-Location).Path
docker build -t raphii-openvr-dev -f maintainer/Dockerfile .
docker run --rm --mount "type=bind,source=$repo,target=/work" raphii-openvr-dev python maintainer/verify.py
docker run --rm --mount "type=bind,source=$repo,target=/work" raphii-openvr-dev python maintainer/verify.py --target aarch64-unknown-linux-gnu
```

The ARM64 run cross-compiles and uses QEMU. These tests check the wrapper and its binary layout, not compatibility with a running SteamVR installation.

## Regenerate bindings

Use the same Docker image and `$repo` value:

```powershell
docker run --rm --mount "type=bind,source=$repo,target=/work" raphii-openvr-dev python maintainer/generate.py
git diff -- src/generated examples/abi.rs tests/abi-capi.cpp tests/abi-cpp.cpp vendor
```

An unchanged SDK and generator should produce no diff. For an intentional update, review and commit the generated files, then run all three native test commands above.

The image pins Rust 1.93.0, bindgen 0.72.1 and Clang 22.1.8. `generate.py` checks the bindgen and Clang versions before writing files.

## Update the SDK

The current SDK is [OpenVR 2.15.6](https://github.com/ValveSoftware/openvr/tree/0924064316de3effbcd1acf1e309182a2deb1c05). Its revision and file hashes live in `vendor/REVISION` and `vendor/SHA256SUMS`.

Replace the vendor headers and license with the files from the chosen Valve revision. Update the revision, hashes and revision check in `generate.py`. Keep the vendor files unchanged; apply any binding corrections in the generator.

Two discrepancies in the current SDK need special treatment:

- The C header declares the initialization return value as `intptr_t`, but the C++ API uses `uint32_t`. `exports.hpp` takes the loader signatures from the C++ header.
- On Linux, the C++ controller-state structure is 60 bytes with alignment 4. The C header gives it 64 bytes with alignment 8. The generator applies packing 4 to its temporary C header. The tests check the difference against both original headers.

When updating the SDK, check whether Valve has corrected these declarations. Do not carry either workaround forward without checking.

## Constraints to keep in mind

The context keeps the native library loaded. Its lock serializes calls and shutdown. Cloned contexts share a session, and action and overlay handles belong to that session. Another wrapper must not initialize or shut down OpenVR in the same process.

The safe API assumes its calls can move between threads when serialized. The pinned headers give no thread-affinity requirement for these calls. Graphics submission needs its own threading review before adding a safe wrapper.

Enum values are integer newtypes so an unknown runtime value remains valid Rust data. Keep that property when changing the generator.

String reads check termination and UTF-8. Property strings and input names are capped at 32768 bytes; binding queries are capped at 4096 entries. Settings reads use a 32768-byte buffer. OpenVR does not report the required settings-string length, so a larger value may be truncated by the runtime.
