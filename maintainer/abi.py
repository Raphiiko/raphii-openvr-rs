"""Compare Rust declarations with native C and C++ compilers parsing Valve headers."""
from pathlib import Path
import re
import subprocess

root = Path(__file__).resolve().parents[1]
fields = {
    "HmdMatrix34_t": ["m"], "HmdVector3_t": ["v"], "HmdQuaternion_t": ["w", "x", "y", "z"],
    "TrackedDevicePose_t": ["mDeviceToAbsoluteTracking", "vVelocity", "vAngularVelocity", "eTrackingResult", "bPoseIsValid", "bDeviceIsConnected"],
    "VREvent_t": ["eventType", "trackedDeviceIndex", "eventAgeSeconds", "data"],
    "VREvent_Data_t": ["property", "controller", "reserved"],
    "VREvent_Property_t": ["container", "prop"],
    "InputDigitalActionData_t": ["bActive", "activeOrigin", "bState", "bChanged", "fUpdateTime"],
    "InputOriginInfo_t": ["devicePath", "trackedDeviceIndex", "rchRenderModelComponentName"],
    "InputBindingInfo_t": ["rchDevicePathName", "rchInputPathName", "rchModeName", "rchSlotName", "rchInputSourceType"],
    "VRActiveActionSet_t": ["ulActionSet", "ulRestrictedToDevice", "ulSecondaryActionSet", "unPadding", "nPriority"],
    "VRControllerState_t": ["unPacketNum", "ulButtonPressed", "ulButtonTouched", "rAxis"],
}
header = (root / "vendor/openvr_capi.h").read_text()
tables = {}
for name, body in re.findall(r"struct (VR_\w+_FnTable)\s*\{(.*?)\n\};", header, re.S):
    tables[name] = re.findall(r"OPENVR_FNTABLE_CALLTYPE \*(\w+)\)", body)

def cpp_source(capi):
    prefix = "" if capi else "vr::"
    src = '#define OPENVR_API_NODLL\n#define OPENVR_NO_STL\n#include "../vendor/' + ("openvr_capi.h" if capi else "openvr.h") + '"\n#include <cstdio>\n#include <cstddef>\n'
    if not capi:
        src += '#include <type_traits>\nstatic_assert(std::is_same<decltype(&vr::VR_InitInternal2), uint32_t (VR_CALLTYPE *)(vr::EVRInitError*, vr::EVRApplicationType, const char*)>::value, "init ABI");\nstatic_assert(std::is_same<decltype(&vr::VR_GetGenericInterface), void* (VR_CALLTYPE *)(const char*, vr::EVRInitError*)>::value, "interface ABI");\nstatic_assert(std::is_same<decltype(&vr::VR_ShutdownInternal), void (VR_CALLTYPE *)()>::value, "shutdown ABI");\n'
    src += "int main() {\n"
    for ty, members in (fields | tables if capi else fields).items():
        src += f'printf("{ty}.size=%zu\\n{ty}.align=%zu\\n", sizeof({prefix}{ty}), alignof({prefix}{ty}));\n'
        for member in members:
            src += f'printf("{ty}.{member}=%zu\\n", offsetof({prefix}{ty}, {member}));\n'
    return src + "}\n"

(root / "tests/abi-capi.cpp").write_text(cpp_source(True))
(root / "tests/abi-cpp.cpp").write_text(cpp_source(False))
src = "use raphii_openvr_rs::raw::*;\nfn main() {\n"
for ty, members in (fields | tables).items():
    src += f'println!("{ty}.size={{}}\\n{ty}.align={{}}", size_of::<{ty}>(), align_of::<{ty}>());\n'
    for member in members:
        src += f'println!("{ty}.{member}={{}}", std::mem::offset_of!({ty}, {member}));\n'
src += "}\n"
(root / "examples").mkdir(exist_ok=True)
(root / "examples/abi.rs").write_text(src)
subprocess.run(["rustfmt", "--edition", "2024", str(root / "examples/abi.rs")], check=True)
