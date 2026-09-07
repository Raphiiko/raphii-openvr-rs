#include "../vendor/openvr.h"
typedef decltype(&vr::VR_InitInternal2) OpenvrInit;
typedef decltype(&vr::VR_ShutdownInternal) OpenvrShutdown;
typedef decltype(&vr::VR_GetGenericInterface) OpenvrGetInterface;
