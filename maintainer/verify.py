"""Run native SDK ABI probes and fake-runtime tests without loading SteamVR."""
from pathlib import Path
import argparse
import os
import subprocess

root = Path(__file__).resolve().parents[1]
os.chdir(root)
parser = argparse.ArgumentParser()
parser.add_argument("--target", default="x86_64-pc-windows-msvc" if os.name == "nt" else "x86_64-unknown-linux-gnu")
args = parser.parse_args()
target = args.target
out = root / "test-output" / target
out.mkdir(parents=True, exist_ok=True)
env = dict(os.environ)
runner = []
if target.startswith("aarch64"):
    env["CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER"] = "aarch64-linux-gnu-gcc"
    env["CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUNNER"] = "qemu-aarch64-static -L /usr/aarch64-linux-gnu"
    runner = ["qemu-aarch64-static", "-L", "/usr/aarch64-linux-gnu"]

def run(command, capture=False):
    print("+", subprocess.list2cmdline([str(v) for v in command]), flush=True)
    result = subprocess.run(command, check=True, env=env, text=True, stdout=subprocess.PIPE if capture else None)
    return result.stdout

def compile(source, output, shared=False):
    if os.name == "nt":
        vswhere = Path(os.environ["ProgramFiles(x86)"]) / "Microsoft Visual Studio/Installer/vswhere.exe"
        vs = subprocess.check_output([vswhere, "-latest", "-products", "*", "-requires", "Microsoft.VisualStudio.Component.VC.Tools.x86.x64", "-property", "installationPath"], text=True).strip()
        vcvars = Path(vs) / "VC/Auxiliary/Build/vcvars64.bat"
        command = ["cl", "/nologo", "/std:c++17", "/EHsc", "/W4", str(source), "/Fe:" + str(output), "/Fo:" + str(output.with_suffix(".obj"))]
        if shared: command.append("/LD")
        batch = out / "compile.cmd"
        batch.write_text(f'@call "{vcvars}" >nul\n@if errorlevel 1 exit /b %errorlevel%\n@' + subprocess.list2cmdline(command) + '\n', encoding="utf-8")
        run(["cmd", "/d", "/c", str(batch)])
    else:
        compiler = "aarch64-linux-gnu-g++" if target.startswith("aarch64") else "g++"
        command = [compiler, "-std=c++17", "-Wall", "-Wextra", "-Werror", "-Wno-unused-variable", str(source), "-o", str(output)]
        if shared: command += ["-shared", "-fPIC"]
        run(command)

rust_output = run(["cargo", "run", "--quiet", "--target", target, "--example", "abi"], True)
def parse(text): return dict(line.split("=", 1) for line in text.splitlines() if "=" in line)
rust = parse(rust_output)
(out / "rust-abi.txt").write_text(rust_output)
for kind in ["capi", "cpp"]:
    executable = out / ("abi-" + kind + (".exe" if os.name == "nt" else ""))
    compile(root / ("tests/abi-" + kind + ".cpp"), executable)
    output = run([*runner, str(executable)], True)
    (out / (kind + "-abi.txt")).write_text(output)
    native = parse(output)
    differences = {key: (value, rust.get(key)) for key, value in native.items() if rust.get(key) != value}
    if kind == "capi" and "linux" in target:
        expected = {"VRControllerState_t.size": ("64", "60"), "VRControllerState_t.align": ("8", "4"), "VRControllerState_t.ulButtonPressed": ("8", "4"), "VRControllerState_t.ulButtonTouched": ("16", "12"), "VRControllerState_t.rAxis": ("24", "20")}
        for key, value in expected.items():
            assert differences.pop(key, None) == value, "Valve controller packing discrepancy changed"
        print("capi: verified 5 known controller packing differences; C++ ABI is authoritative")
    if differences: raise RuntimeError(f"{kind} ABI mismatch: {differences}")
    print(f"{kind}: {len(native)} ABI values checked", flush=True)
library = out / ("fake-openvr.dll" if os.name == "nt" else "libfake-openvr.so")
compile(root / "tests/fake-runtime.cpp", library, True)
missing = out / ("missing-symbol.dll" if os.name == "nt" else "libmissing-symbol.so")
compile(root / "tests/missing-symbol.cpp", missing, True)
env["RAPHII_OPENVR_TEST_RUNTIME"] = str(library)
env["RAPHII_OPENVR_TEST_MISSING_SYMBOL"] = str(missing)
run(["cargo", "test", "--target", target, "--test", "runtime", "--", "--ignored", "--nocapture"])
