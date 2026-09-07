#define OPENVR_API_NODLL
#define OPENVR_NO_STL
#include "../vendor/openvr.h"
#include <cstdio>
#include <cstddef>
#include <type_traits>
static_assert(std::is_same<decltype(&vr::VR_InitInternal2), uint32_t (VR_CALLTYPE *)(vr::EVRInitError*, vr::EVRApplicationType, const char*)>::value, "init ABI");
static_assert(std::is_same<decltype(&vr::VR_GetGenericInterface), void* (VR_CALLTYPE *)(const char*, vr::EVRInitError*)>::value, "interface ABI");
static_assert(std::is_same<decltype(&vr::VR_ShutdownInternal), void (VR_CALLTYPE *)()>::value, "shutdown ABI");
int main() {
printf("HmdMatrix34_t.size=%zu\nHmdMatrix34_t.align=%zu\n", sizeof(vr::HmdMatrix34_t), alignof(vr::HmdMatrix34_t));
printf("HmdMatrix34_t.m=%zu\n", offsetof(vr::HmdMatrix34_t, m));
printf("HmdVector3_t.size=%zu\nHmdVector3_t.align=%zu\n", sizeof(vr::HmdVector3_t), alignof(vr::HmdVector3_t));
printf("HmdVector3_t.v=%zu\n", offsetof(vr::HmdVector3_t, v));
printf("HmdQuaternion_t.size=%zu\nHmdQuaternion_t.align=%zu\n", sizeof(vr::HmdQuaternion_t), alignof(vr::HmdQuaternion_t));
printf("HmdQuaternion_t.w=%zu\n", offsetof(vr::HmdQuaternion_t, w));
printf("HmdQuaternion_t.x=%zu\n", offsetof(vr::HmdQuaternion_t, x));
printf("HmdQuaternion_t.y=%zu\n", offsetof(vr::HmdQuaternion_t, y));
printf("HmdQuaternion_t.z=%zu\n", offsetof(vr::HmdQuaternion_t, z));
printf("TrackedDevicePose_t.size=%zu\nTrackedDevicePose_t.align=%zu\n", sizeof(vr::TrackedDevicePose_t), alignof(vr::TrackedDevicePose_t));
printf("TrackedDevicePose_t.mDeviceToAbsoluteTracking=%zu\n", offsetof(vr::TrackedDevicePose_t, mDeviceToAbsoluteTracking));
printf("TrackedDevicePose_t.vVelocity=%zu\n", offsetof(vr::TrackedDevicePose_t, vVelocity));
printf("TrackedDevicePose_t.vAngularVelocity=%zu\n", offsetof(vr::TrackedDevicePose_t, vAngularVelocity));
printf("TrackedDevicePose_t.eTrackingResult=%zu\n", offsetof(vr::TrackedDevicePose_t, eTrackingResult));
printf("TrackedDevicePose_t.bPoseIsValid=%zu\n", offsetof(vr::TrackedDevicePose_t, bPoseIsValid));
printf("TrackedDevicePose_t.bDeviceIsConnected=%zu\n", offsetof(vr::TrackedDevicePose_t, bDeviceIsConnected));
printf("VREvent_t.size=%zu\nVREvent_t.align=%zu\n", sizeof(vr::VREvent_t), alignof(vr::VREvent_t));
printf("VREvent_t.eventType=%zu\n", offsetof(vr::VREvent_t, eventType));
printf("VREvent_t.trackedDeviceIndex=%zu\n", offsetof(vr::VREvent_t, trackedDeviceIndex));
printf("VREvent_t.eventAgeSeconds=%zu\n", offsetof(vr::VREvent_t, eventAgeSeconds));
printf("VREvent_t.data=%zu\n", offsetof(vr::VREvent_t, data));
printf("VREvent_Data_t.size=%zu\nVREvent_Data_t.align=%zu\n", sizeof(vr::VREvent_Data_t), alignof(vr::VREvent_Data_t));
printf("VREvent_Data_t.property=%zu\n", offsetof(vr::VREvent_Data_t, property));
printf("VREvent_Data_t.controller=%zu\n", offsetof(vr::VREvent_Data_t, controller));
printf("VREvent_Data_t.reserved=%zu\n", offsetof(vr::VREvent_Data_t, reserved));
printf("VREvent_Property_t.size=%zu\nVREvent_Property_t.align=%zu\n", sizeof(vr::VREvent_Property_t), alignof(vr::VREvent_Property_t));
printf("VREvent_Property_t.container=%zu\n", offsetof(vr::VREvent_Property_t, container));
printf("VREvent_Property_t.prop=%zu\n", offsetof(vr::VREvent_Property_t, prop));
printf("InputDigitalActionData_t.size=%zu\nInputDigitalActionData_t.align=%zu\n", sizeof(vr::InputDigitalActionData_t), alignof(vr::InputDigitalActionData_t));
printf("InputDigitalActionData_t.bActive=%zu\n", offsetof(vr::InputDigitalActionData_t, bActive));
printf("InputDigitalActionData_t.activeOrigin=%zu\n", offsetof(vr::InputDigitalActionData_t, activeOrigin));
printf("InputDigitalActionData_t.bState=%zu\n", offsetof(vr::InputDigitalActionData_t, bState));
printf("InputDigitalActionData_t.bChanged=%zu\n", offsetof(vr::InputDigitalActionData_t, bChanged));
printf("InputDigitalActionData_t.fUpdateTime=%zu\n", offsetof(vr::InputDigitalActionData_t, fUpdateTime));
printf("InputOriginInfo_t.size=%zu\nInputOriginInfo_t.align=%zu\n", sizeof(vr::InputOriginInfo_t), alignof(vr::InputOriginInfo_t));
printf("InputOriginInfo_t.devicePath=%zu\n", offsetof(vr::InputOriginInfo_t, devicePath));
printf("InputOriginInfo_t.trackedDeviceIndex=%zu\n", offsetof(vr::InputOriginInfo_t, trackedDeviceIndex));
printf("InputOriginInfo_t.rchRenderModelComponentName=%zu\n", offsetof(vr::InputOriginInfo_t, rchRenderModelComponentName));
printf("InputBindingInfo_t.size=%zu\nInputBindingInfo_t.align=%zu\n", sizeof(vr::InputBindingInfo_t), alignof(vr::InputBindingInfo_t));
printf("InputBindingInfo_t.rchDevicePathName=%zu\n", offsetof(vr::InputBindingInfo_t, rchDevicePathName));
printf("InputBindingInfo_t.rchInputPathName=%zu\n", offsetof(vr::InputBindingInfo_t, rchInputPathName));
printf("InputBindingInfo_t.rchModeName=%zu\n", offsetof(vr::InputBindingInfo_t, rchModeName));
printf("InputBindingInfo_t.rchSlotName=%zu\n", offsetof(vr::InputBindingInfo_t, rchSlotName));
printf("InputBindingInfo_t.rchInputSourceType=%zu\n", offsetof(vr::InputBindingInfo_t, rchInputSourceType));
printf("VRActiveActionSet_t.size=%zu\nVRActiveActionSet_t.align=%zu\n", sizeof(vr::VRActiveActionSet_t), alignof(vr::VRActiveActionSet_t));
printf("VRActiveActionSet_t.ulActionSet=%zu\n", offsetof(vr::VRActiveActionSet_t, ulActionSet));
printf("VRActiveActionSet_t.ulRestrictedToDevice=%zu\n", offsetof(vr::VRActiveActionSet_t, ulRestrictedToDevice));
printf("VRActiveActionSet_t.ulSecondaryActionSet=%zu\n", offsetof(vr::VRActiveActionSet_t, ulSecondaryActionSet));
printf("VRActiveActionSet_t.unPadding=%zu\n", offsetof(vr::VRActiveActionSet_t, unPadding));
printf("VRActiveActionSet_t.nPriority=%zu\n", offsetof(vr::VRActiveActionSet_t, nPriority));
printf("VRControllerState_t.size=%zu\nVRControllerState_t.align=%zu\n", sizeof(vr::VRControllerState_t), alignof(vr::VRControllerState_t));
printf("VRControllerState_t.unPacketNum=%zu\n", offsetof(vr::VRControllerState_t, unPacketNum));
printf("VRControllerState_t.ulButtonPressed=%zu\n", offsetof(vr::VRControllerState_t, ulButtonPressed));
printf("VRControllerState_t.ulButtonTouched=%zu\n", offsetof(vr::VRControllerState_t, ulButtonTouched));
printf("VRControllerState_t.rAxis=%zu\n", offsetof(vr::VRControllerState_t, rAxis));
}
