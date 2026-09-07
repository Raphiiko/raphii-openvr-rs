#define OPENVR_API_NODLL
#define VR_InitInternal Capi_InitInternal
#define VR_GetGenericInterface Capi_GetGenericInterface
#define VR_ShutdownInternal Capi_ShutdownInternal
#include "../vendor/openvr_capi.h"
#undef VR_InitInternal
#undef VR_GetGenericInterface
#undef VR_ShutdownInternal
#include <cstdio>
#include <cstdlib>
#include <cstring>

#ifdef _WIN32
#define EXPORT extern "C" __declspec(dllexport)
#else
#define EXPORT extern "C" __attribute__((visibility("default")))
#endif
#define CALL OPENVR_FNTABLE_CALLTYPE

static int mode, init_count, shutdown_count, calls, active, event_index;
static float float_setting = 1.0f;
static bool bool_setting;
static int32_t int_setting;
static char string_setting[32768] = "setting";
static bool installed;
static bool overlay_visible;
static VR_IVRSystem_FnTable system_table;
static VR_IVRSettings_FnTable settings_table;
static VR_IVROverlay_FnTable overlay_table;
static VR_IVRInput_FnTable input_table;
static VR_IVRApplications_FnTable applications_table;

static void called() {
  if (!active)
    std::abort();
  ++calls;
}
EXPORT void TestMode(int value) {
  mode = value;
  event_index = 0;
}
EXPORT int TestCount(int kind) {
  return kind == 0 ? init_count : kind == 1 ? shutdown_count : calls;
}

static ETrackedDeviceClass CALL device_class(uint32_t) {
  called();
  return static_cast<ETrackedDeviceClass>(mode == 7 ? 9999 : 2);
}
static ETrackedControllerRole CALL device_role(uint32_t) {
  called();
  return static_cast<ETrackedControllerRole>(mode == 7 ? 9999 : 1);
}
static EDeviceActivityLevel CALL activity(uint32_t) {
  called();
  return static_cast<EDeviceActivityLevel>(mode == 7 ? 9999 : 1);
}
static float CALL float_property(uint32_t, ETrackedDeviceProperty,
                                 ETrackedPropertyError *error) {
  called();
  *error = static_cast<ETrackedPropertyError>(mode == 4 ? 9999 : 0);
  return 0.75f;
}
static bool CALL bool_property(uint32_t, ETrackedDeviceProperty,
                               ETrackedPropertyError *error) {
  called();
  *error = static_cast<ETrackedPropertyError>(0);
  return true;
}
static int32_t CALL int_property(uint32_t, ETrackedDeviceProperty,
                                 ETrackedPropertyError *error) {
  called();
  *error = static_cast<ETrackedPropertyError>(0);
  return -42;
}
static uint64_t CALL uint_property(uint32_t, ETrackedDeviceProperty,
                                   ETrackedPropertyError *error) {
  called();
  *error = static_cast<ETrackedPropertyError>(0);
  return 0xfedcba9876543210ULL;
}
static uint32_t CALL string_property(uint32_t, ETrackedDeviceProperty,
                                     char *buffer, uint32_t size,
                                     ETrackedPropertyError *error) {
  called();
  const uint32_t required = mode == 8 ? 600 : 7;
  if (size < required) {
    *error = ETrackedPropertyError_TrackedProp_BufferTooSmall;
    return required;
  }
  *error = static_cast<ETrackedPropertyError>(0);
  if (mode == 9) {
    std::memset(buffer, 'x', size);
    return size;
  }
  if (mode == 10) {
    buffer[0] = static_cast<char>(0xff);
    buffer[1] = 0;
    return 2;
  }
  if (mode == 8) {
    std::memset(buffer, 'a', required - 1);
    buffer[required - 1] = 0;
  } else
    std::memcpy(buffer, "serial", required);
  return required;
}
static void CALL poses(ETrackingUniverseOrigin, float prediction,
                       TrackedDevicePose_t *data, uint32_t count) {
  called();
  if (count != 64 || prediction != 0)
    std::abort();
  std::memset(data, 0, sizeof(*data) * count);
  data[0].bPoseIsValid = true;
  data[0].bDeviceIsConnected = true;
  data[0].mDeviceToAbsoluteTracking.m[0][3] = 1.25f;
}
static bool CALL event(VREvent_t *data, uint32_t size) {
  called();
  if (size != sizeof(*data))
    std::abort();
  if (event_index >= 3)
    return false;
  std::memset(data, 0, size);
  data->trackedDeviceIndex = 3;
  data->eventAgeSeconds = 0.25f;
  data->eventType = event_index == 0   ? EVREventType_VREvent_PropertyChanged
                    : event_index == 1 ? 0x7ffffffe
                                       : EVREventType_VREvent_Quit;
  data->data.property.container = 0xfedcba9876543210ULL;
  data->data.property.prop =
      ETrackedDeviceProperty_Prop_DeviceBatteryPercentage_Float;
  ++event_index;
  return true;
}
static float CALL get_float(char *, char *, EVRSettingsError *error) {
  called();
  *error = static_cast<EVRSettingsError>(mode == 4 ? 9999 : 0);
  return float_setting;
}
static void CALL set_float(char *, char *, float value,
                           EVRSettingsError *error) {
  called();
  *error = static_cast<EVRSettingsError>(0);
  float_setting = value;
}
static bool CALL get_bool(char *, char *, EVRSettingsError *error) {
  called();
  *error = static_cast<EVRSettingsError>(0);
  return bool_setting;
}
static void CALL set_bool(char *, char *, bool value, EVRSettingsError *error) {
  called();
  *error = static_cast<EVRSettingsError>(0);
  bool_setting = value;
}
static int32_t CALL get_int(char *, char *, EVRSettingsError *error) {
  called();
  *error = static_cast<EVRSettingsError>(0);
  return int_setting;
}
static void CALL set_int(char *, char *, int32_t value,
                         EVRSettingsError *error) {
  called();
  *error = static_cast<EVRSettingsError>(0);
  int_setting = value;
}
static void CALL get_string(char *, char *, char *buffer, uint32_t size,
                            EVRSettingsError *error) {
  called();
  *error = static_cast<EVRSettingsError>(0);
  if (mode == 9)
    std::memset(buffer, 'x', size);
  else
    std::snprintf(buffer, size, "%s", string_setting);
}
static void CALL set_string(char *, char *, char *value,
                            EVRSettingsError *error) {
  called();
  *error = static_cast<EVRSettingsError>(0);
  std::snprintf(string_setting, sizeof(string_setting), "%s", value);
}
static void CALL remove_key(char *, char *, EVRSettingsError *error) {
  called();
  *error = static_cast<EVRSettingsError>(0);
}
static void CALL remove_section(char *, EVRSettingsError *error) {
  called();
  *error = static_cast<EVRSettingsError>(0);
}
static EVROverlayError CALL create_overlay(char *, char *,
                                           VROverlayHandle_t *value) {
  called();
  *value = 0x123456789abcdef0ULL;
  return static_cast<EVROverlayError>(0);
}
static EVROverlayError CALL destroy_overlay(VROverlayHandle_t value) {
  called();
  if (value != 0x123456789abcdef0ULL)
    std::abort();
  return static_cast<EVROverlayError>(0);
}
static EVROverlayError CALL overlay_float(VROverlayHandle_t value, float) {
  return destroy_overlay(value);
}
static EVROverlayError CALL overlay_sort(VROverlayHandle_t value, uint32_t) {
  return destroy_overlay(value);
}
static EVROverlayError CALL overlay_show(VROverlayHandle_t value) {
  overlay_visible = true;
  return destroy_overlay(value);
}
static EVROverlayError CALL overlay_hide(VROverlayHandle_t value) {
  overlay_visible = false;
  return destroy_overlay(value);
}
static bool CALL dashboard() {
  called();
  return overlay_visible;
}
static EVROverlayError CALL overlay_transform(VROverlayHandle_t value,
                                              uint32_t device,
                                              HmdMatrix34_t *matrix) {
  if (device != 0 || matrix->m[2][3] != -0.15f)
    std::abort();
  return destroy_overlay(value);
}
static EVROverlayError CALL overlay_raw(VROverlayHandle_t value, void *bytes,
                                        uint32_t w, uint32_t h, uint32_t bpp) {
  if (w != 1 || h != 1 || bpp != 4 ||
      static_cast<unsigned char *>(bytes)[3] != 255)
    std::abort();
  return destroy_overlay(value);
}
static EVRInputError CALL manifest(char *path) {
  called();
  if (!path || !path[0])
    std::abort();
  return static_cast<EVRInputError>(0);
}
static EVRInputError CALL handle(char *, uint64_t *value) {
  called();
  *value = 0xfedcba9876543210ULL;
  return static_cast<EVRInputError>(0);
}
static EVRInputError CALL update(VRActiveActionSet_t *sets, uint32_t size,
                                 uint32_t count) {
  called();
  if (size != sizeof(*sets) || count != 1 ||
      sets[0].ulActionSet != 0xfedcba9876543210ULL)
    std::abort();
  return static_cast<EVRInputError>(0);
}
static EVRInputError CALL digital(VRActionHandle_t,
                                  InputDigitalActionData_t *data, uint32_t size,
                                  VRInputValueHandle_t) {
  called();
  if (size != sizeof(*data))
    std::abort();
  std::memset(data, 0, size);
  data->bActive = true;
  data->bChanged = true;
  data->bState = true;
  data->activeOrigin = 0xfedcba9876543210ULL;
  data->fUpdateTime = -0.125f;
  return static_cast<EVRInputError>(0);
}
static EVRInputError CALL origins(VRActionSetHandle_t, VRActionHandle_t,
                                  VRInputValueHandle_t *data, uint32_t count) {
  called();
  if (count != 16)
    std::abort();
  data[0] = 42;
  return static_cast<EVRInputError>(0);
}
static EVRInputError CALL localized(VRInputValueHandle_t, char *data,
                                    uint32_t size, int32_t bits) {
  called();
  if (mode == 8 && size < 512)
    return EVRInputError_VRInputError_BufferTooSmall;
  if (mode == 9)
    std::memset(data, 'x', size);
  else
    std::snprintf(data, size, "%d", bits);
  return static_cast<EVRInputError>(0);
}
static EVRInputError CALL origin_info(VRInputValueHandle_t,
                                      InputOriginInfo_t *data, uint32_t size) {
  called();
  if (size != sizeof(*data))
    std::abort();
  std::memset(data, 0, size);
  data->trackedDeviceIndex = 3;
  return static_cast<EVRInputError>(0);
}
static EVRInputError CALL bindings(VRActionHandle_t, InputBindingInfo_t *data,
                                   uint32_t size, uint32_t count,
                                   uint32_t *returned) {
  called();
  if (size != sizeof(*data))
    std::abort();
  *returned = mode == 11 ? count + 1 : mode == 8 ? 20 : 1;
  if (mode == 8 && count < *returned)
    return EVRInputError_VRInputError_BufferTooSmall;
  std::memset(data, 0, size * count);
  std::snprintf(data[0].rchDevicePathName, 128, "/user/hand/left");
  return static_cast<EVRInputError>(0);
}
static EVRInputError CALL binding_ui(char *key, VRActionSetHandle_t,
                                     VRInputValueHandle_t, bool) {
  called();
  if (key && std::strcmp(key, "test.app"))
    std::abort();
  return static_cast<EVRInputError>(0);
}
static EVRApplicationError CALL add_manifest(char *, bool) {
  called();
  installed = true;
  return static_cast<EVRApplicationError>(0);
}
static EVRApplicationError CALL remove_manifest(char *) {
  called();
  installed = false;
  return static_cast<EVRApplicationError>(0);
}
static bool CALL is_installed(char *) {
  called();
  return installed;
}

EXPORT uint32_t VR_InitInternal2(EVRInitError *error, EVRApplicationType type,
                                 const char *) {
  ++init_count;
  if (active)
    std::abort();
  if (type != EVRApplicationType_VRApplication_Background)
    std::abort();
  *error = static_cast<EVRInitError>(mode == 1 ? 9999 : 0);
  if (mode == 1)
    return 0;
  active = 1;
  system_table = {};
  settings_table = {};
  overlay_table = {};
  input_table = {};
  applications_table = {};
  system_table.GetTrackedDeviceClass = device_class;
  system_table.GetControllerRoleForTrackedDeviceIndex = device_role;
  system_table.GetTrackedDeviceActivityLevel = activity;
  system_table.GetFloatTrackedDeviceProperty = float_property;
  system_table.GetBoolTrackedDeviceProperty = bool_property;
  system_table.GetStringTrackedDeviceProperty = string_property;
  system_table.GetInt32TrackedDeviceProperty = int_property;
  system_table.GetUint64TrackedDeviceProperty = uint_property;
  system_table.GetDeviceToAbsoluteTrackingPose = poses;
  system_table.PollNextEvent = event;
  settings_table.GetFloat = mode == 6 ? nullptr : get_float;
  settings_table.SetFloat = set_float;
  settings_table.GetBool = get_bool;
  settings_table.SetBool = set_bool;
  settings_table.GetInt32 = get_int;
  settings_table.SetInt32 = set_int;
  settings_table.GetString = get_string;
  settings_table.SetString = set_string;
  settings_table.RemoveKeyInSection = remove_key;
  settings_table.RemoveSection = remove_section;
  overlay_table.CreateOverlay = create_overlay;
  overlay_table.DestroyOverlay = destroy_overlay;
  overlay_table.SetOverlayAlpha = overlay_float;
  overlay_table.SetOverlayWidthInMeters = overlay_float;
  overlay_table.SetOverlaySortOrder = overlay_sort;
  overlay_table.ShowOverlay = overlay_show;
  overlay_table.HideOverlay = overlay_hide;
  overlay_table.SetOverlayTransformTrackedDeviceRelative = overlay_transform;
  overlay_table.SetOverlayRaw = overlay_raw;
  overlay_table.IsDashboardVisible = dashboard;
  input_table.SetActionManifestPath = manifest;
  input_table.GetActionHandle = handle;
  input_table.GetActionSetHandle = handle;
  input_table.GetInputSourceHandle = handle;
  input_table.UpdateActionState = update;
  input_table.GetDigitalActionData = digital;
  input_table.GetActionOrigins = origins;
  input_table.GetOriginLocalizedName = localized;
  input_table.GetOriginTrackedDeviceInfo = origin_info;
  input_table.GetActionBindingInfo = bindings;
  input_table.OpenBindingUI = binding_ui;
  applications_table.AddApplicationManifest = add_manifest;
  applications_table.RemoveApplicationManifest = remove_manifest;
  applications_table.IsApplicationInstalled = is_installed;
  return 123;
}
EXPORT void VR_ShutdownInternal() {
  if (!active)
    std::abort();
  active = 0;
  ++shutdown_count;
}
EXPORT void *VR_GetGenericInterface(const char *version, EVRInitError *error) {
  called();
  *error = static_cast<EVRInitError>(0);
  if (!std::strcmp(version, "FnTable:IVRSystem_026") && mode != 2)
    return &system_table;
  if (!std::strcmp(version, "FnTable:IVRSettings_003") && mode != 3)
    return &settings_table;
  if (!std::strcmp(version, "FnTable:IVROverlay_028") && mode != 12)
    return &overlay_table;
  if (!std::strcmp(version, "FnTable:IVRInput_011") && mode != 12)
    return &input_table;
  if (!std::strcmp(version, "FnTable:IVRApplications_008") && mode != 12)
    return &applications_table;
  *error = static_cast<EVRInitError>(105);
  return nullptr;
}
