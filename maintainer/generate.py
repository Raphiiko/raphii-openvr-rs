"""Run inside the maintainer image, never during a consumer build."""
from pathlib import Path
import hashlib
import re
import subprocess
import tempfile

root = Path(__file__).resolve().parents[1]
assert (root / "vendor/REVISION").read_text().strip() == "0924064316de3effbcd1acf1e309182a2deb1c05"
for line in (root / "vendor/SHA256SUMS").read_text().splitlines():
    expected, name = line.split("  ", 1)
    assert hashlib.sha256((root / "vendor" / name).read_bytes()).hexdigest() == expected, f"vendor checksum differs: {name}"
assert subprocess.check_output(["bindgen", "--version"], text=True).strip() == "bindgen 0.72.1"
assert subprocess.check_output(["clang", "--version"], text=True).splitlines()[0] == "clang version 22.1.8"
targets = ["x86_64-pc-windows-msvc", "x86_64-unknown-linux-gnu", "aarch64-unknown-linux-gnu"]
header = root / "vendor/openvr_capi.h"
enums = re.findall(r"typedef enum (\w+)", header.read_text())
out = root / "src/generated"
out.mkdir(exist_ok=True)
for target in targets:
    common = ["--use-core", "--no-doc-comments", "--with-derive-default", "--with-derive-partialeq", "--default-enum-style", "newtype", "--no-prepend-enum-name", "--no-layout-tests"]
    clang = ["--target=" + target, "-DOPENVR_API_NODLL", "-DOPENVR_NO_STL"]
    if target.startswith("aarch64"):
        clang += ["--sysroot=/usr/aarch64-linux-gnu"]
    source = header.read_text()
    if "linux" in target:
        source, count = re.subn(r"(typedef struct VRControllerState_t\s*\{.*?\} VRControllerState_t;)", r"#pragma pack(push, 4)\n\1\n#pragma pack(pop)", source, flags=re.S)
        assert count == 1, "controller state declaration changed"
    with tempfile.TemporaryDirectory() as temporary:
        prepared = Path(temporary) / "openvr_capi.h"
        prepared.write_text(source)
        raw = subprocess.check_output(["bindgen", str(prepared), *common, "--blocklist-function", ".*", "--", *clang], text=True)
    for enum in enums:
        raw = raw.replace("pub const " + enum + "_", "pub const ")
    raw = "// Copyright (c) Valve Corporation. See vendor/LICENSE-Valve.\n" + raw
    (out / (target + ".rs")).write_text(raw)
    exports = subprocess.check_output(["bindgen", str(root / "maintainer/exports.hpp"), *common, "--allowlist-type", "Openvr.*", "--", "-x", "c++", "-std=c++11", *clang], text=True)
    exports = "// Copyright (c) Valve Corporation. See vendor/LICENSE-Valve.\n" + exports
    (out / (target + "-exports.rs")).write_text(exports)
subprocess.run(["python", str(root / "maintainer/abi.py")], check=True)
