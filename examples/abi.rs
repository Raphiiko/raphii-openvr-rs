use raphii_openvr_rs::raw::*;
fn main() {
    println!(
        "HmdMatrix34_t.size={}\nHmdMatrix34_t.align={}",
        size_of::<HmdMatrix34_t>(),
        align_of::<HmdMatrix34_t>()
    );
    println!("HmdMatrix34_t.m={}", std::mem::offset_of!(HmdMatrix34_t, m));
    println!(
        "HmdVector3_t.size={}\nHmdVector3_t.align={}",
        size_of::<HmdVector3_t>(),
        align_of::<HmdVector3_t>()
    );
    println!("HmdVector3_t.v={}", std::mem::offset_of!(HmdVector3_t, v));
    println!(
        "HmdQuaternion_t.size={}\nHmdQuaternion_t.align={}",
        size_of::<HmdQuaternion_t>(),
        align_of::<HmdQuaternion_t>()
    );
    println!(
        "HmdQuaternion_t.w={}",
        std::mem::offset_of!(HmdQuaternion_t, w)
    );
    println!(
        "HmdQuaternion_t.x={}",
        std::mem::offset_of!(HmdQuaternion_t, x)
    );
    println!(
        "HmdQuaternion_t.y={}",
        std::mem::offset_of!(HmdQuaternion_t, y)
    );
    println!(
        "HmdQuaternion_t.z={}",
        std::mem::offset_of!(HmdQuaternion_t, z)
    );
    println!(
        "TrackedDevicePose_t.size={}\nTrackedDevicePose_t.align={}",
        size_of::<TrackedDevicePose_t>(),
        align_of::<TrackedDevicePose_t>()
    );
    println!(
        "TrackedDevicePose_t.mDeviceToAbsoluteTracking={}",
        std::mem::offset_of!(TrackedDevicePose_t, mDeviceToAbsoluteTracking)
    );
    println!(
        "TrackedDevicePose_t.vVelocity={}",
        std::mem::offset_of!(TrackedDevicePose_t, vVelocity)
    );
    println!(
        "TrackedDevicePose_t.vAngularVelocity={}",
        std::mem::offset_of!(TrackedDevicePose_t, vAngularVelocity)
    );
    println!(
        "TrackedDevicePose_t.eTrackingResult={}",
        std::mem::offset_of!(TrackedDevicePose_t, eTrackingResult)
    );
    println!(
        "TrackedDevicePose_t.bPoseIsValid={}",
        std::mem::offset_of!(TrackedDevicePose_t, bPoseIsValid)
    );
    println!(
        "TrackedDevicePose_t.bDeviceIsConnected={}",
        std::mem::offset_of!(TrackedDevicePose_t, bDeviceIsConnected)
    );
    println!(
        "VREvent_t.size={}\nVREvent_t.align={}",
        size_of::<VREvent_t>(),
        align_of::<VREvent_t>()
    );
    println!(
        "VREvent_t.eventType={}",
        std::mem::offset_of!(VREvent_t, eventType)
    );
    println!(
        "VREvent_t.trackedDeviceIndex={}",
        std::mem::offset_of!(VREvent_t, trackedDeviceIndex)
    );
    println!(
        "VREvent_t.eventAgeSeconds={}",
        std::mem::offset_of!(VREvent_t, eventAgeSeconds)
    );
    println!("VREvent_t.data={}", std::mem::offset_of!(VREvent_t, data));
    println!(
        "VREvent_Data_t.size={}\nVREvent_Data_t.align={}",
        size_of::<VREvent_Data_t>(),
        align_of::<VREvent_Data_t>()
    );
    println!(
        "VREvent_Data_t.property={}",
        std::mem::offset_of!(VREvent_Data_t, property)
    );
    println!(
        "VREvent_Data_t.controller={}",
        std::mem::offset_of!(VREvent_Data_t, controller)
    );
    println!(
        "VREvent_Data_t.reserved={}",
        std::mem::offset_of!(VREvent_Data_t, reserved)
    );
    println!(
        "VREvent_Property_t.size={}\nVREvent_Property_t.align={}",
        size_of::<VREvent_Property_t>(),
        align_of::<VREvent_Property_t>()
    );
    println!(
        "VREvent_Property_t.container={}",
        std::mem::offset_of!(VREvent_Property_t, container)
    );
    println!(
        "VREvent_Property_t.prop={}",
        std::mem::offset_of!(VREvent_Property_t, prop)
    );
    println!(
        "InputDigitalActionData_t.size={}\nInputDigitalActionData_t.align={}",
        size_of::<InputDigitalActionData_t>(),
        align_of::<InputDigitalActionData_t>()
    );
    println!(
        "InputDigitalActionData_t.bActive={}",
        std::mem::offset_of!(InputDigitalActionData_t, bActive)
    );
    println!(
        "InputDigitalActionData_t.activeOrigin={}",
        std::mem::offset_of!(InputDigitalActionData_t, activeOrigin)
    );
    println!(
        "InputDigitalActionData_t.bState={}",
        std::mem::offset_of!(InputDigitalActionData_t, bState)
    );
    println!(
        "InputDigitalActionData_t.bChanged={}",
        std::mem::offset_of!(InputDigitalActionData_t, bChanged)
    );
    println!(
        "InputDigitalActionData_t.fUpdateTime={}",
        std::mem::offset_of!(InputDigitalActionData_t, fUpdateTime)
    );
    println!(
        "InputOriginInfo_t.size={}\nInputOriginInfo_t.align={}",
        size_of::<InputOriginInfo_t>(),
        align_of::<InputOriginInfo_t>()
    );
    println!(
        "InputOriginInfo_t.devicePath={}",
        std::mem::offset_of!(InputOriginInfo_t, devicePath)
    );
    println!(
        "InputOriginInfo_t.trackedDeviceIndex={}",
        std::mem::offset_of!(InputOriginInfo_t, trackedDeviceIndex)
    );
    println!(
        "InputOriginInfo_t.rchRenderModelComponentName={}",
        std::mem::offset_of!(InputOriginInfo_t, rchRenderModelComponentName)
    );
    println!(
        "InputBindingInfo_t.size={}\nInputBindingInfo_t.align={}",
        size_of::<InputBindingInfo_t>(),
        align_of::<InputBindingInfo_t>()
    );
    println!(
        "InputBindingInfo_t.rchDevicePathName={}",
        std::mem::offset_of!(InputBindingInfo_t, rchDevicePathName)
    );
    println!(
        "InputBindingInfo_t.rchInputPathName={}",
        std::mem::offset_of!(InputBindingInfo_t, rchInputPathName)
    );
    println!(
        "InputBindingInfo_t.rchModeName={}",
        std::mem::offset_of!(InputBindingInfo_t, rchModeName)
    );
    println!(
        "InputBindingInfo_t.rchSlotName={}",
        std::mem::offset_of!(InputBindingInfo_t, rchSlotName)
    );
    println!(
        "InputBindingInfo_t.rchInputSourceType={}",
        std::mem::offset_of!(InputBindingInfo_t, rchInputSourceType)
    );
    println!(
        "VRActiveActionSet_t.size={}\nVRActiveActionSet_t.align={}",
        size_of::<VRActiveActionSet_t>(),
        align_of::<VRActiveActionSet_t>()
    );
    println!(
        "VRActiveActionSet_t.ulActionSet={}",
        std::mem::offset_of!(VRActiveActionSet_t, ulActionSet)
    );
    println!(
        "VRActiveActionSet_t.ulRestrictedToDevice={}",
        std::mem::offset_of!(VRActiveActionSet_t, ulRestrictedToDevice)
    );
    println!(
        "VRActiveActionSet_t.ulSecondaryActionSet={}",
        std::mem::offset_of!(VRActiveActionSet_t, ulSecondaryActionSet)
    );
    println!(
        "VRActiveActionSet_t.unPadding={}",
        std::mem::offset_of!(VRActiveActionSet_t, unPadding)
    );
    println!(
        "VRActiveActionSet_t.nPriority={}",
        std::mem::offset_of!(VRActiveActionSet_t, nPriority)
    );
    println!(
        "VRControllerState_t.size={}\nVRControllerState_t.align={}",
        size_of::<VRControllerState_t>(),
        align_of::<VRControllerState_t>()
    );
    println!(
        "VRControllerState_t.unPacketNum={}",
        std::mem::offset_of!(VRControllerState_t, unPacketNum)
    );
    println!(
        "VRControllerState_t.ulButtonPressed={}",
        std::mem::offset_of!(VRControllerState_t, ulButtonPressed)
    );
    println!(
        "VRControllerState_t.ulButtonTouched={}",
        std::mem::offset_of!(VRControllerState_t, ulButtonTouched)
    );
    println!(
        "VRControllerState_t.rAxis={}",
        std::mem::offset_of!(VRControllerState_t, rAxis)
    );
    println!(
        "VR_IVRSystem_FnTable.size={}\nVR_IVRSystem_FnTable.align={}",
        size_of::<VR_IVRSystem_FnTable>(),
        align_of::<VR_IVRSystem_FnTable>()
    );
    println!(
        "VR_IVRSystem_FnTable.GetRecommendedRenderTargetSize={}",
        std::mem::offset_of!(VR_IVRSystem_FnTable, GetRecommendedRenderTargetSize)
    );
    println!(
        "VR_IVRSystem_FnTable.GetProjectionMatrix={}",
        std::mem::offset_of!(VR_IVRSystem_FnTable, GetProjectionMatrix)
    );
    println!(
        "VR_IVRSystem_FnTable.GetProjectionRaw={}",
        std::mem::offset_of!(VR_IVRSystem_FnTable, GetProjectionRaw)
    );
    println!(
        "VR_IVRSystem_FnTable.ComputeDistortion={}",
        std::mem::offset_of!(VR_IVRSystem_FnTable, ComputeDistortion)
    );
    println!(
        "VR_IVRSystem_FnTable.ComputeDistortionSet={}",
        std::mem::offset_of!(VR_IVRSystem_FnTable, ComputeDistortionSet)
    );
    println!(
        "VR_IVRSystem_FnTable.GetEyeToHeadTransform={}",
        std::mem::offset_of!(VR_IVRSystem_FnTable, GetEyeToHeadTransform)
    );
    println!(
        "VR_IVRSystem_FnTable.GetTimeSinceLastVsync={}",
        std::mem::offset_of!(VR_IVRSystem_FnTable, GetTimeSinceLastVsync)
    );
    println!(
        "VR_IVRSystem_FnTable.GetD3D9AdapterIndex={}",
        std::mem::offset_of!(VR_IVRSystem_FnTable, GetD3D9AdapterIndex)
    );
    println!(
        "VR_IVRSystem_FnTable.GetDXGIOutputInfo={}",
        std::mem::offset_of!(VR_IVRSystem_FnTable, GetDXGIOutputInfo)
    );
    println!(
        "VR_IVRSystem_FnTable.GetOutputDevice={}",
        std::mem::offset_of!(VR_IVRSystem_FnTable, GetOutputDevice)
    );
    println!(
        "VR_IVRSystem_FnTable.IsDisplayOnDesktop={}",
        std::mem::offset_of!(VR_IVRSystem_FnTable, IsDisplayOnDesktop)
    );
    println!(
        "VR_IVRSystem_FnTable.SetDisplayVisibility={}",
        std::mem::offset_of!(VR_IVRSystem_FnTable, SetDisplayVisibility)
    );
    println!(
        "VR_IVRSystem_FnTable.GetDeviceToAbsoluteTrackingPose={}",
        std::mem::offset_of!(VR_IVRSystem_FnTable, GetDeviceToAbsoluteTrackingPose)
    );
    println!(
        "VR_IVRSystem_FnTable.GetSeatedZeroPoseToStandingAbsoluteTrackingPose={}",
        std::mem::offset_of!(
            VR_IVRSystem_FnTable,
            GetSeatedZeroPoseToStandingAbsoluteTrackingPose
        )
    );
    println!(
        "VR_IVRSystem_FnTable.GetRawZeroPoseToStandingAbsoluteTrackingPose={}",
        std::mem::offset_of!(
            VR_IVRSystem_FnTable,
            GetRawZeroPoseToStandingAbsoluteTrackingPose
        )
    );
    println!(
        "VR_IVRSystem_FnTable.GetSortedTrackedDeviceIndicesOfClass={}",
        std::mem::offset_of!(VR_IVRSystem_FnTable, GetSortedTrackedDeviceIndicesOfClass)
    );
    println!(
        "VR_IVRSystem_FnTable.GetTrackedDeviceActivityLevel={}",
        std::mem::offset_of!(VR_IVRSystem_FnTable, GetTrackedDeviceActivityLevel)
    );
    println!(
        "VR_IVRSystem_FnTable.ApplyTransform={}",
        std::mem::offset_of!(VR_IVRSystem_FnTable, ApplyTransform)
    );
    println!(
        "VR_IVRSystem_FnTable.GetTrackedDeviceIndexForControllerRole={}",
        std::mem::offset_of!(VR_IVRSystem_FnTable, GetTrackedDeviceIndexForControllerRole)
    );
    println!(
        "VR_IVRSystem_FnTable.GetControllerRoleForTrackedDeviceIndex={}",
        std::mem::offset_of!(VR_IVRSystem_FnTable, GetControllerRoleForTrackedDeviceIndex)
    );
    println!(
        "VR_IVRSystem_FnTable.GetTrackedDeviceClass={}",
        std::mem::offset_of!(VR_IVRSystem_FnTable, GetTrackedDeviceClass)
    );
    println!(
        "VR_IVRSystem_FnTable.IsTrackedDeviceConnected={}",
        std::mem::offset_of!(VR_IVRSystem_FnTable, IsTrackedDeviceConnected)
    );
    println!(
        "VR_IVRSystem_FnTable.GetBoolTrackedDeviceProperty={}",
        std::mem::offset_of!(VR_IVRSystem_FnTable, GetBoolTrackedDeviceProperty)
    );
    println!(
        "VR_IVRSystem_FnTable.GetFloatTrackedDeviceProperty={}",
        std::mem::offset_of!(VR_IVRSystem_FnTable, GetFloatTrackedDeviceProperty)
    );
    println!(
        "VR_IVRSystem_FnTable.GetInt32TrackedDeviceProperty={}",
        std::mem::offset_of!(VR_IVRSystem_FnTable, GetInt32TrackedDeviceProperty)
    );
    println!(
        "VR_IVRSystem_FnTable.GetUint64TrackedDeviceProperty={}",
        std::mem::offset_of!(VR_IVRSystem_FnTable, GetUint64TrackedDeviceProperty)
    );
    println!(
        "VR_IVRSystem_FnTable.GetMatrix34TrackedDeviceProperty={}",
        std::mem::offset_of!(VR_IVRSystem_FnTable, GetMatrix34TrackedDeviceProperty)
    );
    println!(
        "VR_IVRSystem_FnTable.GetArrayTrackedDeviceProperty={}",
        std::mem::offset_of!(VR_IVRSystem_FnTable, GetArrayTrackedDeviceProperty)
    );
    println!(
        "VR_IVRSystem_FnTable.GetStringTrackedDeviceProperty={}",
        std::mem::offset_of!(VR_IVRSystem_FnTable, GetStringTrackedDeviceProperty)
    );
    println!(
        "VR_IVRSystem_FnTable.GetPropErrorNameFromEnum={}",
        std::mem::offset_of!(VR_IVRSystem_FnTable, GetPropErrorNameFromEnum)
    );
    println!(
        "VR_IVRSystem_FnTable.PollNextEvent={}",
        std::mem::offset_of!(VR_IVRSystem_FnTable, PollNextEvent)
    );
    println!(
        "VR_IVRSystem_FnTable.PollNextEventWithPose={}",
        std::mem::offset_of!(VR_IVRSystem_FnTable, PollNextEventWithPose)
    );
    println!(
        "VR_IVRSystem_FnTable.PollNextEventWithPoseAndOverlays={}",
        std::mem::offset_of!(VR_IVRSystem_FnTable, PollNextEventWithPoseAndOverlays)
    );
    println!(
        "VR_IVRSystem_FnTable.GetEventTypeNameFromEnum={}",
        std::mem::offset_of!(VR_IVRSystem_FnTable, GetEventTypeNameFromEnum)
    );
    println!(
        "VR_IVRSystem_FnTable.GetHiddenAreaMesh={}",
        std::mem::offset_of!(VR_IVRSystem_FnTable, GetHiddenAreaMesh)
    );
    println!(
        "VR_IVRSystem_FnTable.GetEyeTrackedFoveationCenter={}",
        std::mem::offset_of!(VR_IVRSystem_FnTable, GetEyeTrackedFoveationCenter)
    );
    println!(
        "VR_IVRSystem_FnTable.GetEyeTrackedFoveationCenterForProjection={}",
        std::mem::offset_of!(
            VR_IVRSystem_FnTable,
            GetEyeTrackedFoveationCenterForProjection
        )
    );
    println!(
        "VR_IVRSystem_FnTable.GetControllerState={}",
        std::mem::offset_of!(VR_IVRSystem_FnTable, GetControllerState)
    );
    println!(
        "VR_IVRSystem_FnTable.GetControllerStateWithPose={}",
        std::mem::offset_of!(VR_IVRSystem_FnTable, GetControllerStateWithPose)
    );
    println!(
        "VR_IVRSystem_FnTable.TriggerHapticPulse={}",
        std::mem::offset_of!(VR_IVRSystem_FnTable, TriggerHapticPulse)
    );
    println!(
        "VR_IVRSystem_FnTable.GetButtonIdNameFromEnum={}",
        std::mem::offset_of!(VR_IVRSystem_FnTable, GetButtonIdNameFromEnum)
    );
    println!(
        "VR_IVRSystem_FnTable.GetControllerAxisTypeNameFromEnum={}",
        std::mem::offset_of!(VR_IVRSystem_FnTable, GetControllerAxisTypeNameFromEnum)
    );
    println!(
        "VR_IVRSystem_FnTable.IsInputAvailable={}",
        std::mem::offset_of!(VR_IVRSystem_FnTable, IsInputAvailable)
    );
    println!(
        "VR_IVRSystem_FnTable.IsSteamVRDrawingControllers={}",
        std::mem::offset_of!(VR_IVRSystem_FnTable, IsSteamVRDrawingControllers)
    );
    println!(
        "VR_IVRSystem_FnTable.ShouldApplicationPause={}",
        std::mem::offset_of!(VR_IVRSystem_FnTable, ShouldApplicationPause)
    );
    println!(
        "VR_IVRSystem_FnTable.ShouldApplicationReduceRenderingWork={}",
        std::mem::offset_of!(VR_IVRSystem_FnTable, ShouldApplicationReduceRenderingWork)
    );
    println!(
        "VR_IVRSystem_FnTable.PerformFirmwareUpdate={}",
        std::mem::offset_of!(VR_IVRSystem_FnTable, PerformFirmwareUpdate)
    );
    println!(
        "VR_IVRSystem_FnTable.AcknowledgeQuit_Exiting={}",
        std::mem::offset_of!(VR_IVRSystem_FnTable, AcknowledgeQuit_Exiting)
    );
    println!(
        "VR_IVRSystem_FnTable.GetAppContainerFilePaths={}",
        std::mem::offset_of!(VR_IVRSystem_FnTable, GetAppContainerFilePaths)
    );
    println!(
        "VR_IVRSystem_FnTable.GetRuntimeVersion={}",
        std::mem::offset_of!(VR_IVRSystem_FnTable, GetRuntimeVersion)
    );
    println!(
        "VR_IVRSystem_FnTable.SetSDKVersion={}",
        std::mem::offset_of!(VR_IVRSystem_FnTable, SetSDKVersion)
    );
    println!(
        "VR_IVRExtendedDisplay_FnTable.size={}\nVR_IVRExtendedDisplay_FnTable.align={}",
        size_of::<VR_IVRExtendedDisplay_FnTable>(),
        align_of::<VR_IVRExtendedDisplay_FnTable>()
    );
    println!(
        "VR_IVRExtendedDisplay_FnTable.GetWindowBounds={}",
        std::mem::offset_of!(VR_IVRExtendedDisplay_FnTable, GetWindowBounds)
    );
    println!(
        "VR_IVRExtendedDisplay_FnTable.GetEyeOutputViewport={}",
        std::mem::offset_of!(VR_IVRExtendedDisplay_FnTable, GetEyeOutputViewport)
    );
    println!(
        "VR_IVRExtendedDisplay_FnTable.GetDXGIOutputInfo={}",
        std::mem::offset_of!(VR_IVRExtendedDisplay_FnTable, GetDXGIOutputInfo)
    );
    println!(
        "VR_IVRTrackedCamera_FnTable.size={}\nVR_IVRTrackedCamera_FnTable.align={}",
        size_of::<VR_IVRTrackedCamera_FnTable>(),
        align_of::<VR_IVRTrackedCamera_FnTable>()
    );
    println!(
        "VR_IVRTrackedCamera_FnTable.GetCameraErrorNameFromEnum={}",
        std::mem::offset_of!(VR_IVRTrackedCamera_FnTable, GetCameraErrorNameFromEnum)
    );
    println!(
        "VR_IVRTrackedCamera_FnTable.HasCamera={}",
        std::mem::offset_of!(VR_IVRTrackedCamera_FnTable, HasCamera)
    );
    println!(
        "VR_IVRTrackedCamera_FnTable.GetCameraFrameSize={}",
        std::mem::offset_of!(VR_IVRTrackedCamera_FnTable, GetCameraFrameSize)
    );
    println!(
        "VR_IVRTrackedCamera_FnTable.GetCameraIntrinsics={}",
        std::mem::offset_of!(VR_IVRTrackedCamera_FnTable, GetCameraIntrinsics)
    );
    println!(
        "VR_IVRTrackedCamera_FnTable.GetCameraProjection={}",
        std::mem::offset_of!(VR_IVRTrackedCamera_FnTable, GetCameraProjection)
    );
    println!(
        "VR_IVRTrackedCamera_FnTable.AcquireVideoStreamingService={}",
        std::mem::offset_of!(VR_IVRTrackedCamera_FnTable, AcquireVideoStreamingService)
    );
    println!(
        "VR_IVRTrackedCamera_FnTable.ReleaseVideoStreamingService={}",
        std::mem::offset_of!(VR_IVRTrackedCamera_FnTable, ReleaseVideoStreamingService)
    );
    println!(
        "VR_IVRTrackedCamera_FnTable.GetVideoStreamFrameBuffer={}",
        std::mem::offset_of!(VR_IVRTrackedCamera_FnTable, GetVideoStreamFrameBuffer)
    );
    println!(
        "VR_IVRTrackedCamera_FnTable.GetVideoStreamTextureSize={}",
        std::mem::offset_of!(VR_IVRTrackedCamera_FnTable, GetVideoStreamTextureSize)
    );
    println!(
        "VR_IVRTrackedCamera_FnTable.GetVideoStreamTextureD3D11={}",
        std::mem::offset_of!(VR_IVRTrackedCamera_FnTable, GetVideoStreamTextureD3D11)
    );
    println!(
        "VR_IVRTrackedCamera_FnTable.GetVideoStreamTextureGL={}",
        std::mem::offset_of!(VR_IVRTrackedCamera_FnTable, GetVideoStreamTextureGL)
    );
    println!(
        "VR_IVRTrackedCamera_FnTable.ReleaseVideoStreamTextureGL={}",
        std::mem::offset_of!(VR_IVRTrackedCamera_FnTable, ReleaseVideoStreamTextureGL)
    );
    println!(
        "VR_IVRTrackedCamera_FnTable.SetCameraTrackingSpace={}",
        std::mem::offset_of!(VR_IVRTrackedCamera_FnTable, SetCameraTrackingSpace)
    );
    println!(
        "VR_IVRTrackedCamera_FnTable.GetCameraTrackingSpace={}",
        std::mem::offset_of!(VR_IVRTrackedCamera_FnTable, GetCameraTrackingSpace)
    );
    println!(
        "VR_IVRApplications_FnTable.size={}\nVR_IVRApplications_FnTable.align={}",
        size_of::<VR_IVRApplications_FnTable>(),
        align_of::<VR_IVRApplications_FnTable>()
    );
    println!(
        "VR_IVRApplications_FnTable.AddApplicationManifest={}",
        std::mem::offset_of!(VR_IVRApplications_FnTable, AddApplicationManifest)
    );
    println!(
        "VR_IVRApplications_FnTable.RemoveApplicationManifest={}",
        std::mem::offset_of!(VR_IVRApplications_FnTable, RemoveApplicationManifest)
    );
    println!(
        "VR_IVRApplications_FnTable.IsApplicationInstalled={}",
        std::mem::offset_of!(VR_IVRApplications_FnTable, IsApplicationInstalled)
    );
    println!(
        "VR_IVRApplications_FnTable.GetApplicationCount={}",
        std::mem::offset_of!(VR_IVRApplications_FnTable, GetApplicationCount)
    );
    println!(
        "VR_IVRApplications_FnTable.GetApplicationKeyByIndex={}",
        std::mem::offset_of!(VR_IVRApplications_FnTable, GetApplicationKeyByIndex)
    );
    println!(
        "VR_IVRApplications_FnTable.GetApplicationKeyByProcessId={}",
        std::mem::offset_of!(VR_IVRApplications_FnTable, GetApplicationKeyByProcessId)
    );
    println!(
        "VR_IVRApplications_FnTable.LaunchApplication={}",
        std::mem::offset_of!(VR_IVRApplications_FnTable, LaunchApplication)
    );
    println!(
        "VR_IVRApplications_FnTable.LaunchTemplateApplication={}",
        std::mem::offset_of!(VR_IVRApplications_FnTable, LaunchTemplateApplication)
    );
    println!(
        "VR_IVRApplications_FnTable.LaunchApplicationFromMimeType={}",
        std::mem::offset_of!(VR_IVRApplications_FnTable, LaunchApplicationFromMimeType)
    );
    println!(
        "VR_IVRApplications_FnTable.LaunchDashboardOverlay={}",
        std::mem::offset_of!(VR_IVRApplications_FnTable, LaunchDashboardOverlay)
    );
    println!(
        "VR_IVRApplications_FnTable.CancelApplicationLaunch={}",
        std::mem::offset_of!(VR_IVRApplications_FnTable, CancelApplicationLaunch)
    );
    println!(
        "VR_IVRApplications_FnTable.IdentifyApplication={}",
        std::mem::offset_of!(VR_IVRApplications_FnTable, IdentifyApplication)
    );
    println!(
        "VR_IVRApplications_FnTable.GetApplicationProcessId={}",
        std::mem::offset_of!(VR_IVRApplications_FnTable, GetApplicationProcessId)
    );
    println!(
        "VR_IVRApplications_FnTable.GetApplicationsErrorNameFromEnum={}",
        std::mem::offset_of!(VR_IVRApplications_FnTable, GetApplicationsErrorNameFromEnum)
    );
    println!(
        "VR_IVRApplications_FnTable.GetApplicationPropertyString={}",
        std::mem::offset_of!(VR_IVRApplications_FnTable, GetApplicationPropertyString)
    );
    println!(
        "VR_IVRApplications_FnTable.GetApplicationPropertyBool={}",
        std::mem::offset_of!(VR_IVRApplications_FnTable, GetApplicationPropertyBool)
    );
    println!(
        "VR_IVRApplications_FnTable.GetApplicationPropertyUint64={}",
        std::mem::offset_of!(VR_IVRApplications_FnTable, GetApplicationPropertyUint64)
    );
    println!(
        "VR_IVRApplications_FnTable.SetApplicationAutoLaunch={}",
        std::mem::offset_of!(VR_IVRApplications_FnTable, SetApplicationAutoLaunch)
    );
    println!(
        "VR_IVRApplications_FnTable.GetApplicationAutoLaunch={}",
        std::mem::offset_of!(VR_IVRApplications_FnTable, GetApplicationAutoLaunch)
    );
    println!(
        "VR_IVRApplications_FnTable.SetDefaultApplicationForMimeType={}",
        std::mem::offset_of!(VR_IVRApplications_FnTable, SetDefaultApplicationForMimeType)
    );
    println!(
        "VR_IVRApplications_FnTable.GetDefaultApplicationForMimeType={}",
        std::mem::offset_of!(VR_IVRApplications_FnTable, GetDefaultApplicationForMimeType)
    );
    println!(
        "VR_IVRApplications_FnTable.GetApplicationSupportedMimeTypes={}",
        std::mem::offset_of!(VR_IVRApplications_FnTable, GetApplicationSupportedMimeTypes)
    );
    println!(
        "VR_IVRApplications_FnTable.GetApplicationsThatSupportMimeType={}",
        std::mem::offset_of!(
            VR_IVRApplications_FnTable,
            GetApplicationsThatSupportMimeType
        )
    );
    println!(
        "VR_IVRApplications_FnTable.GetApplicationLaunchArguments={}",
        std::mem::offset_of!(VR_IVRApplications_FnTable, GetApplicationLaunchArguments)
    );
    println!(
        "VR_IVRApplications_FnTable.GetStartingApplication={}",
        std::mem::offset_of!(VR_IVRApplications_FnTable, GetStartingApplication)
    );
    println!(
        "VR_IVRApplications_FnTable.GetSceneApplicationState={}",
        std::mem::offset_of!(VR_IVRApplications_FnTable, GetSceneApplicationState)
    );
    println!(
        "VR_IVRApplications_FnTable.PerformApplicationPrelaunchCheck={}",
        std::mem::offset_of!(VR_IVRApplications_FnTable, PerformApplicationPrelaunchCheck)
    );
    println!(
        "VR_IVRApplications_FnTable.GetSceneApplicationStateNameFromEnum={}",
        std::mem::offset_of!(
            VR_IVRApplications_FnTable,
            GetSceneApplicationStateNameFromEnum
        )
    );
    println!(
        "VR_IVRApplications_FnTable.LaunchInternalProcess={}",
        std::mem::offset_of!(VR_IVRApplications_FnTable, LaunchInternalProcess)
    );
    println!(
        "VR_IVRApplications_FnTable.RegisterSubprocess={}",
        std::mem::offset_of!(VR_IVRApplications_FnTable, RegisterSubprocess)
    );
    println!(
        "VR_IVRApplications_FnTable.GetCurrentSceneProcessId={}",
        std::mem::offset_of!(VR_IVRApplications_FnTable, GetCurrentSceneProcessId)
    );
    println!(
        "VR_IVRChaperone_FnTable.size={}\nVR_IVRChaperone_FnTable.align={}",
        size_of::<VR_IVRChaperone_FnTable>(),
        align_of::<VR_IVRChaperone_FnTable>()
    );
    println!(
        "VR_IVRChaperone_FnTable.GetCalibrationState={}",
        std::mem::offset_of!(VR_IVRChaperone_FnTable, GetCalibrationState)
    );
    println!(
        "VR_IVRChaperone_FnTable.GetPlayAreaSize={}",
        std::mem::offset_of!(VR_IVRChaperone_FnTable, GetPlayAreaSize)
    );
    println!(
        "VR_IVRChaperone_FnTable.GetPlayAreaRect={}",
        std::mem::offset_of!(VR_IVRChaperone_FnTable, GetPlayAreaRect)
    );
    println!(
        "VR_IVRChaperone_FnTable.ReloadInfo={}",
        std::mem::offset_of!(VR_IVRChaperone_FnTable, ReloadInfo)
    );
    println!(
        "VR_IVRChaperone_FnTable.SetSceneColor={}",
        std::mem::offset_of!(VR_IVRChaperone_FnTable, SetSceneColor)
    );
    println!(
        "VR_IVRChaperone_FnTable.GetBoundsColor={}",
        std::mem::offset_of!(VR_IVRChaperone_FnTable, GetBoundsColor)
    );
    println!(
        "VR_IVRChaperone_FnTable.AreBoundsVisible={}",
        std::mem::offset_of!(VR_IVRChaperone_FnTable, AreBoundsVisible)
    );
    println!(
        "VR_IVRChaperone_FnTable.ForceBoundsVisible={}",
        std::mem::offset_of!(VR_IVRChaperone_FnTable, ForceBoundsVisible)
    );
    println!(
        "VR_IVRChaperone_FnTable.ResetZeroPose={}",
        std::mem::offset_of!(VR_IVRChaperone_FnTable, ResetZeroPose)
    );
    println!(
        "VR_IVRChaperoneSetup_FnTable.size={}\nVR_IVRChaperoneSetup_FnTable.align={}",
        size_of::<VR_IVRChaperoneSetup_FnTable>(),
        align_of::<VR_IVRChaperoneSetup_FnTable>()
    );
    println!(
        "VR_IVRChaperoneSetup_FnTable.CommitWorkingCopy={}",
        std::mem::offset_of!(VR_IVRChaperoneSetup_FnTable, CommitWorkingCopy)
    );
    println!(
        "VR_IVRChaperoneSetup_FnTable.RevertWorkingCopy={}",
        std::mem::offset_of!(VR_IVRChaperoneSetup_FnTable, RevertWorkingCopy)
    );
    println!(
        "VR_IVRChaperoneSetup_FnTable.GetWorkingPlayAreaSize={}",
        std::mem::offset_of!(VR_IVRChaperoneSetup_FnTable, GetWorkingPlayAreaSize)
    );
    println!(
        "VR_IVRChaperoneSetup_FnTable.GetWorkingPlayAreaRect={}",
        std::mem::offset_of!(VR_IVRChaperoneSetup_FnTable, GetWorkingPlayAreaRect)
    );
    println!(
        "VR_IVRChaperoneSetup_FnTable.GetWorkingCollisionBoundsInfo={}",
        std::mem::offset_of!(VR_IVRChaperoneSetup_FnTable, GetWorkingCollisionBoundsInfo)
    );
    println!(
        "VR_IVRChaperoneSetup_FnTable.GetLiveCollisionBoundsInfo={}",
        std::mem::offset_of!(VR_IVRChaperoneSetup_FnTable, GetLiveCollisionBoundsInfo)
    );
    println!(
        "VR_IVRChaperoneSetup_FnTable.GetWorkingSeatedZeroPoseToRawTrackingPose={}",
        std::mem::offset_of!(
            VR_IVRChaperoneSetup_FnTable,
            GetWorkingSeatedZeroPoseToRawTrackingPose
        )
    );
    println!(
        "VR_IVRChaperoneSetup_FnTable.GetWorkingStandingZeroPoseToRawTrackingPose={}",
        std::mem::offset_of!(
            VR_IVRChaperoneSetup_FnTable,
            GetWorkingStandingZeroPoseToRawTrackingPose
        )
    );
    println!(
        "VR_IVRChaperoneSetup_FnTable.SetWorkingPlayAreaSize={}",
        std::mem::offset_of!(VR_IVRChaperoneSetup_FnTable, SetWorkingPlayAreaSize)
    );
    println!(
        "VR_IVRChaperoneSetup_FnTable.SetWorkingCollisionBoundsInfo={}",
        std::mem::offset_of!(VR_IVRChaperoneSetup_FnTable, SetWorkingCollisionBoundsInfo)
    );
    println!(
        "VR_IVRChaperoneSetup_FnTable.SetWorkingPerimeter={}",
        std::mem::offset_of!(VR_IVRChaperoneSetup_FnTable, SetWorkingPerimeter)
    );
    println!(
        "VR_IVRChaperoneSetup_FnTable.SetWorkingSeatedZeroPoseToRawTrackingPose={}",
        std::mem::offset_of!(
            VR_IVRChaperoneSetup_FnTable,
            SetWorkingSeatedZeroPoseToRawTrackingPose
        )
    );
    println!(
        "VR_IVRChaperoneSetup_FnTable.SetWorkingStandingZeroPoseToRawTrackingPose={}",
        std::mem::offset_of!(
            VR_IVRChaperoneSetup_FnTable,
            SetWorkingStandingZeroPoseToRawTrackingPose
        )
    );
    println!(
        "VR_IVRChaperoneSetup_FnTable.ReloadFromDisk={}",
        std::mem::offset_of!(VR_IVRChaperoneSetup_FnTable, ReloadFromDisk)
    );
    println!(
        "VR_IVRChaperoneSetup_FnTable.GetLiveSeatedZeroPoseToRawTrackingPose={}",
        std::mem::offset_of!(
            VR_IVRChaperoneSetup_FnTable,
            GetLiveSeatedZeroPoseToRawTrackingPose
        )
    );
    println!(
        "VR_IVRChaperoneSetup_FnTable.ExportLiveToBuffer={}",
        std::mem::offset_of!(VR_IVRChaperoneSetup_FnTable, ExportLiveToBuffer)
    );
    println!(
        "VR_IVRChaperoneSetup_FnTable.ImportFromBufferToWorking={}",
        std::mem::offset_of!(VR_IVRChaperoneSetup_FnTable, ImportFromBufferToWorking)
    );
    println!(
        "VR_IVRChaperoneSetup_FnTable.ShowWorkingSetPreview={}",
        std::mem::offset_of!(VR_IVRChaperoneSetup_FnTable, ShowWorkingSetPreview)
    );
    println!(
        "VR_IVRChaperoneSetup_FnTable.HideWorkingSetPreview={}",
        std::mem::offset_of!(VR_IVRChaperoneSetup_FnTable, HideWorkingSetPreview)
    );
    println!(
        "VR_IVRChaperoneSetup_FnTable.RoomSetupStarting={}",
        std::mem::offset_of!(VR_IVRChaperoneSetup_FnTable, RoomSetupStarting)
    );
    println!(
        "VR_IVRCompositor_FnTable.size={}\nVR_IVRCompositor_FnTable.align={}",
        size_of::<VR_IVRCompositor_FnTable>(),
        align_of::<VR_IVRCompositor_FnTable>()
    );
    println!(
        "VR_IVRCompositor_FnTable.SetTrackingSpace={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, SetTrackingSpace)
    );
    println!(
        "VR_IVRCompositor_FnTable.GetTrackingSpace={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, GetTrackingSpace)
    );
    println!(
        "VR_IVRCompositor_FnTable.WaitGetPoses={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, WaitGetPoses)
    );
    println!(
        "VR_IVRCompositor_FnTable.GetLastPoses={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, GetLastPoses)
    );
    println!(
        "VR_IVRCompositor_FnTable.GetLastPoseForTrackedDeviceIndex={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, GetLastPoseForTrackedDeviceIndex)
    );
    println!(
        "VR_IVRCompositor_FnTable.GetSubmitTexture={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, GetSubmitTexture)
    );
    println!(
        "VR_IVRCompositor_FnTable.Submit={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, Submit)
    );
    println!(
        "VR_IVRCompositor_FnTable.SubmitWithArrayIndex={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, SubmitWithArrayIndex)
    );
    println!(
        "VR_IVRCompositor_FnTable.ClearLastSubmittedFrame={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, ClearLastSubmittedFrame)
    );
    println!(
        "VR_IVRCompositor_FnTable.PostPresentHandoff={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, PostPresentHandoff)
    );
    println!(
        "VR_IVRCompositor_FnTable.GetFrameTiming={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, GetFrameTiming)
    );
    println!(
        "VR_IVRCompositor_FnTable.GetFrameTimings={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, GetFrameTimings)
    );
    println!(
        "VR_IVRCompositor_FnTable.GetFrameTimeRemaining={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, GetFrameTimeRemaining)
    );
    println!(
        "VR_IVRCompositor_FnTable.GetCumulativeStats={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, GetCumulativeStats)
    );
    println!(
        "VR_IVRCompositor_FnTable.FadeToColor={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, FadeToColor)
    );
    println!(
        "VR_IVRCompositor_FnTable.GetCurrentFadeColor={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, GetCurrentFadeColor)
    );
    println!(
        "VR_IVRCompositor_FnTable.FadeGrid={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, FadeGrid)
    );
    println!(
        "VR_IVRCompositor_FnTable.GetCurrentGridAlpha={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, GetCurrentGridAlpha)
    );
    println!(
        "VR_IVRCompositor_FnTable.SetSkyboxOverride={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, SetSkyboxOverride)
    );
    println!(
        "VR_IVRCompositor_FnTable.ClearSkyboxOverride={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, ClearSkyboxOverride)
    );
    println!(
        "VR_IVRCompositor_FnTable.CompositorBringToFront={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, CompositorBringToFront)
    );
    println!(
        "VR_IVRCompositor_FnTable.CompositorGoToBack={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, CompositorGoToBack)
    );
    println!(
        "VR_IVRCompositor_FnTable.CompositorQuit={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, CompositorQuit)
    );
    println!(
        "VR_IVRCompositor_FnTable.IsFullscreen={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, IsFullscreen)
    );
    println!(
        "VR_IVRCompositor_FnTable.GetCurrentSceneFocusProcess={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, GetCurrentSceneFocusProcess)
    );
    println!(
        "VR_IVRCompositor_FnTable.GetLastFrameRenderer={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, GetLastFrameRenderer)
    );
    println!(
        "VR_IVRCompositor_FnTable.CanRenderScene={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, CanRenderScene)
    );
    println!(
        "VR_IVRCompositor_FnTable.ShowMirrorWindow={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, ShowMirrorWindow)
    );
    println!(
        "VR_IVRCompositor_FnTable.HideMirrorWindow={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, HideMirrorWindow)
    );
    println!(
        "VR_IVRCompositor_FnTable.IsMirrorWindowVisible={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, IsMirrorWindowVisible)
    );
    println!(
        "VR_IVRCompositor_FnTable.CompositorDumpImages={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, CompositorDumpImages)
    );
    println!(
        "VR_IVRCompositor_FnTable.ShouldAppRenderWithLowResources={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, ShouldAppRenderWithLowResources)
    );
    println!(
        "VR_IVRCompositor_FnTable.ForceInterleavedReprojectionOn={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, ForceInterleavedReprojectionOn)
    );
    println!(
        "VR_IVRCompositor_FnTable.ForceReconnectProcess={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, ForceReconnectProcess)
    );
    println!(
        "VR_IVRCompositor_FnTable.SuspendRendering={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, SuspendRendering)
    );
    println!(
        "VR_IVRCompositor_FnTable.GetMirrorTextureD3D11={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, GetMirrorTextureD3D11)
    );
    println!(
        "VR_IVRCompositor_FnTable.ReleaseMirrorTextureD3D11={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, ReleaseMirrorTextureD3D11)
    );
    println!(
        "VR_IVRCompositor_FnTable.GetMirrorTextureGL={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, GetMirrorTextureGL)
    );
    println!(
        "VR_IVRCompositor_FnTable.ReleaseSharedGLTexture={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, ReleaseSharedGLTexture)
    );
    println!(
        "VR_IVRCompositor_FnTable.LockGLSharedTextureForAccess={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, LockGLSharedTextureForAccess)
    );
    println!(
        "VR_IVRCompositor_FnTable.UnlockGLSharedTextureForAccess={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, UnlockGLSharedTextureForAccess)
    );
    println!(
        "VR_IVRCompositor_FnTable.GetVulkanInstanceExtensionsRequired={}",
        std::mem::offset_of!(
            VR_IVRCompositor_FnTable,
            GetVulkanInstanceExtensionsRequired
        )
    );
    println!(
        "VR_IVRCompositor_FnTable.GetVulkanDeviceExtensionsRequired={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, GetVulkanDeviceExtensionsRequired)
    );
    println!(
        "VR_IVRCompositor_FnTable.SetExplicitTimingMode={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, SetExplicitTimingMode)
    );
    println!(
        "VR_IVRCompositor_FnTable.SubmitExplicitTimingData={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, SubmitExplicitTimingData)
    );
    println!(
        "VR_IVRCompositor_FnTable.IsMotionSmoothingEnabled={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, IsMotionSmoothingEnabled)
    );
    println!(
        "VR_IVRCompositor_FnTable.IsMotionSmoothingSupported={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, IsMotionSmoothingSupported)
    );
    println!(
        "VR_IVRCompositor_FnTable.IsCurrentSceneFocusAppLoading={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, IsCurrentSceneFocusAppLoading)
    );
    println!(
        "VR_IVRCompositor_FnTable.SetStageOverride_Async={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, SetStageOverride_Async)
    );
    println!(
        "VR_IVRCompositor_FnTable.ClearStageOverride={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, ClearStageOverride)
    );
    println!(
        "VR_IVRCompositor_FnTable.GetCompositorBenchmarkResults={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, GetCompositorBenchmarkResults)
    );
    println!(
        "VR_IVRCompositor_FnTable.GetLastPosePredictionIDs={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, GetLastPosePredictionIDs)
    );
    println!(
        "VR_IVRCompositor_FnTable.GetPosesForFrame={}",
        std::mem::offset_of!(VR_IVRCompositor_FnTable, GetPosesForFrame)
    );
    println!(
        "VR_IVROverlay_FnTable.size={}\nVR_IVROverlay_FnTable.align={}",
        size_of::<VR_IVROverlay_FnTable>(),
        align_of::<VR_IVROverlay_FnTable>()
    );
    println!(
        "VR_IVROverlay_FnTable.FindOverlay={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, FindOverlay)
    );
    println!(
        "VR_IVROverlay_FnTable.CreateOverlay={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, CreateOverlay)
    );
    println!(
        "VR_IVROverlay_FnTable.CreateSubviewOverlay={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, CreateSubviewOverlay)
    );
    println!(
        "VR_IVROverlay_FnTable.DestroyOverlay={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, DestroyOverlay)
    );
    println!(
        "VR_IVROverlay_FnTable.GetOverlayKey={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, GetOverlayKey)
    );
    println!(
        "VR_IVROverlay_FnTable.GetOverlayName={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, GetOverlayName)
    );
    println!(
        "VR_IVROverlay_FnTable.SetOverlayName={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, SetOverlayName)
    );
    println!(
        "VR_IVROverlay_FnTable.GetOverlayImageData={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, GetOverlayImageData)
    );
    println!(
        "VR_IVROverlay_FnTable.GetOverlayErrorNameFromEnum={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, GetOverlayErrorNameFromEnum)
    );
    println!(
        "VR_IVROverlay_FnTable.SetOverlayRenderingPid={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, SetOverlayRenderingPid)
    );
    println!(
        "VR_IVROverlay_FnTable.GetOverlayRenderingPid={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, GetOverlayRenderingPid)
    );
    println!(
        "VR_IVROverlay_FnTable.SetOverlayFlag={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, SetOverlayFlag)
    );
    println!(
        "VR_IVROverlay_FnTable.GetOverlayFlag={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, GetOverlayFlag)
    );
    println!(
        "VR_IVROverlay_FnTable.GetOverlayFlags={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, GetOverlayFlags)
    );
    println!(
        "VR_IVROverlay_FnTable.SetOverlayColor={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, SetOverlayColor)
    );
    println!(
        "VR_IVROverlay_FnTable.GetOverlayColor={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, GetOverlayColor)
    );
    println!(
        "VR_IVROverlay_FnTable.SetOverlayAlpha={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, SetOverlayAlpha)
    );
    println!(
        "VR_IVROverlay_FnTable.GetOverlayAlpha={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, GetOverlayAlpha)
    );
    println!(
        "VR_IVROverlay_FnTable.SetOverlayTexelAspect={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, SetOverlayTexelAspect)
    );
    println!(
        "VR_IVROverlay_FnTable.GetOverlayTexelAspect={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, GetOverlayTexelAspect)
    );
    println!(
        "VR_IVROverlay_FnTable.SetOverlaySortOrder={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, SetOverlaySortOrder)
    );
    println!(
        "VR_IVROverlay_FnTable.GetOverlaySortOrder={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, GetOverlaySortOrder)
    );
    println!(
        "VR_IVROverlay_FnTable.SetOverlayWidthInMeters={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, SetOverlayWidthInMeters)
    );
    println!(
        "VR_IVROverlay_FnTable.GetOverlayWidthInMeters={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, GetOverlayWidthInMeters)
    );
    println!(
        "VR_IVROverlay_FnTable.SetOverlayCurvature={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, SetOverlayCurvature)
    );
    println!(
        "VR_IVROverlay_FnTable.GetOverlayCurvature={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, GetOverlayCurvature)
    );
    println!(
        "VR_IVROverlay_FnTable.SetOverlayPreCurvePitch={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, SetOverlayPreCurvePitch)
    );
    println!(
        "VR_IVROverlay_FnTable.GetOverlayPreCurvePitch={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, GetOverlayPreCurvePitch)
    );
    println!(
        "VR_IVROverlay_FnTable.SetOverlayTextureColorSpace={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, SetOverlayTextureColorSpace)
    );
    println!(
        "VR_IVROverlay_FnTable.GetOverlayTextureColorSpace={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, GetOverlayTextureColorSpace)
    );
    println!(
        "VR_IVROverlay_FnTable.SetOverlayTextureBounds={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, SetOverlayTextureBounds)
    );
    println!(
        "VR_IVROverlay_FnTable.GetOverlayTextureBounds={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, GetOverlayTextureBounds)
    );
    println!(
        "VR_IVROverlay_FnTable.GetOverlayTransformType={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, GetOverlayTransformType)
    );
    println!(
        "VR_IVROverlay_FnTable.SetOverlayTransformAbsolute={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, SetOverlayTransformAbsolute)
    );
    println!(
        "VR_IVROverlay_FnTable.GetOverlayTransformAbsolute={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, GetOverlayTransformAbsolute)
    );
    println!(
        "VR_IVROverlay_FnTable.SetOverlayTransformTrackedDeviceRelative={}",
        std::mem::offset_of!(
            VR_IVROverlay_FnTable,
            SetOverlayTransformTrackedDeviceRelative
        )
    );
    println!(
        "VR_IVROverlay_FnTable.GetOverlayTransformTrackedDeviceRelative={}",
        std::mem::offset_of!(
            VR_IVROverlay_FnTable,
            GetOverlayTransformTrackedDeviceRelative
        )
    );
    println!(
        "VR_IVROverlay_FnTable.SetOverlayTransformTrackedDeviceComponent={}",
        std::mem::offset_of!(
            VR_IVROverlay_FnTable,
            SetOverlayTransformTrackedDeviceComponent
        )
    );
    println!(
        "VR_IVROverlay_FnTable.GetOverlayTransformTrackedDeviceComponent={}",
        std::mem::offset_of!(
            VR_IVROverlay_FnTable,
            GetOverlayTransformTrackedDeviceComponent
        )
    );
    println!(
        "VR_IVROverlay_FnTable.SetOverlayTransformCursor={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, SetOverlayTransformCursor)
    );
    println!(
        "VR_IVROverlay_FnTable.GetOverlayTransformCursor={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, GetOverlayTransformCursor)
    );
    println!(
        "VR_IVROverlay_FnTable.SetOverlayTransformProjection={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, SetOverlayTransformProjection)
    );
    println!(
        "VR_IVROverlay_FnTable.SetSubviewPosition={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, SetSubviewPosition)
    );
    println!(
        "VR_IVROverlay_FnTable.ShowOverlay={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, ShowOverlay)
    );
    println!(
        "VR_IVROverlay_FnTable.HideOverlay={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, HideOverlay)
    );
    println!(
        "VR_IVROverlay_FnTable.IsOverlayVisible={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, IsOverlayVisible)
    );
    println!(
        "VR_IVROverlay_FnTable.GetTransformForOverlayCoordinates={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, GetTransformForOverlayCoordinates)
    );
    println!(
        "VR_IVROverlay_FnTable.WaitFrameSync={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, WaitFrameSync)
    );
    println!(
        "VR_IVROverlay_FnTable.PollNextOverlayEvent={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, PollNextOverlayEvent)
    );
    println!(
        "VR_IVROverlay_FnTable.GetOverlayInputMethod={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, GetOverlayInputMethod)
    );
    println!(
        "VR_IVROverlay_FnTable.SetOverlayInputMethod={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, SetOverlayInputMethod)
    );
    println!(
        "VR_IVROverlay_FnTable.GetOverlayMouseScale={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, GetOverlayMouseScale)
    );
    println!(
        "VR_IVROverlay_FnTable.SetOverlayMouseScale={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, SetOverlayMouseScale)
    );
    println!(
        "VR_IVROverlay_FnTable.ComputeOverlayIntersection={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, ComputeOverlayIntersection)
    );
    println!(
        "VR_IVROverlay_FnTable.IsHoverTargetOverlay={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, IsHoverTargetOverlay)
    );
    println!(
        "VR_IVROverlay_FnTable.SetOverlayIntersectionMask={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, SetOverlayIntersectionMask)
    );
    println!(
        "VR_IVROverlay_FnTable.TriggerLaserMouseHapticVibration={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, TriggerLaserMouseHapticVibration)
    );
    println!(
        "VR_IVROverlay_FnTable.SetOverlayCursor={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, SetOverlayCursor)
    );
    println!(
        "VR_IVROverlay_FnTable.SetOverlayCursorPositionOverride={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, SetOverlayCursorPositionOverride)
    );
    println!(
        "VR_IVROverlay_FnTable.ClearOverlayCursorPositionOverride={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, ClearOverlayCursorPositionOverride)
    );
    println!(
        "VR_IVROverlay_FnTable.SetOverlayTexture={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, SetOverlayTexture)
    );
    println!(
        "VR_IVROverlay_FnTable.ClearOverlayTexture={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, ClearOverlayTexture)
    );
    println!(
        "VR_IVROverlay_FnTable.SetOverlayRaw={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, SetOverlayRaw)
    );
    println!(
        "VR_IVROverlay_FnTable.SetOverlayFromFile={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, SetOverlayFromFile)
    );
    println!(
        "VR_IVROverlay_FnTable.GetOverlayTexture={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, GetOverlayTexture)
    );
    println!(
        "VR_IVROverlay_FnTable.ReleaseNativeOverlayHandle={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, ReleaseNativeOverlayHandle)
    );
    println!(
        "VR_IVROverlay_FnTable.GetOverlayTextureSize={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, GetOverlayTextureSize)
    );
    println!(
        "VR_IVROverlay_FnTable.CreateDashboardOverlay={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, CreateDashboardOverlay)
    );
    println!(
        "VR_IVROverlay_FnTable.IsDashboardVisible={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, IsDashboardVisible)
    );
    println!(
        "VR_IVROverlay_FnTable.IsActiveDashboardOverlay={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, IsActiveDashboardOverlay)
    );
    println!(
        "VR_IVROverlay_FnTable.SetDashboardOverlaySceneProcess={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, SetDashboardOverlaySceneProcess)
    );
    println!(
        "VR_IVROverlay_FnTable.GetDashboardOverlaySceneProcess={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, GetDashboardOverlaySceneProcess)
    );
    println!(
        "VR_IVROverlay_FnTable.ShowDashboard={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, ShowDashboard)
    );
    println!(
        "VR_IVROverlay_FnTable.GetPrimaryDashboardDevice={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, GetPrimaryDashboardDevice)
    );
    println!(
        "VR_IVROverlay_FnTable.ShowKeyboard={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, ShowKeyboard)
    );
    println!(
        "VR_IVROverlay_FnTable.ShowKeyboardForOverlay={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, ShowKeyboardForOverlay)
    );
    println!(
        "VR_IVROverlay_FnTable.GetKeyboardText={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, GetKeyboardText)
    );
    println!(
        "VR_IVROverlay_FnTable.HideKeyboard={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, HideKeyboard)
    );
    println!(
        "VR_IVROverlay_FnTable.SetKeyboardTransformAbsolute={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, SetKeyboardTransformAbsolute)
    );
    println!(
        "VR_IVROverlay_FnTable.SetKeyboardPositionForOverlay={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, SetKeyboardPositionForOverlay)
    );
    println!(
        "VR_IVROverlay_FnTable.ShowMessageOverlay={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, ShowMessageOverlay)
    );
    println!(
        "VR_IVROverlay_FnTable.CloseMessageOverlay={}",
        std::mem::offset_of!(VR_IVROverlay_FnTable, CloseMessageOverlay)
    );
    println!(
        "VR_IVROverlayView_FnTable.size={}\nVR_IVROverlayView_FnTable.align={}",
        size_of::<VR_IVROverlayView_FnTable>(),
        align_of::<VR_IVROverlayView_FnTable>()
    );
    println!(
        "VR_IVROverlayView_FnTable.AcquireOverlayView={}",
        std::mem::offset_of!(VR_IVROverlayView_FnTable, AcquireOverlayView)
    );
    println!(
        "VR_IVROverlayView_FnTable.ReleaseOverlayView={}",
        std::mem::offset_of!(VR_IVROverlayView_FnTable, ReleaseOverlayView)
    );
    println!(
        "VR_IVROverlayView_FnTable.PostOverlayEvent={}",
        std::mem::offset_of!(VR_IVROverlayView_FnTable, PostOverlayEvent)
    );
    println!(
        "VR_IVROverlayView_FnTable.IsViewingPermitted={}",
        std::mem::offset_of!(VR_IVROverlayView_FnTable, IsViewingPermitted)
    );
    println!(
        "VR_IVRHeadsetView_FnTable.size={}\nVR_IVRHeadsetView_FnTable.align={}",
        size_of::<VR_IVRHeadsetView_FnTable>(),
        align_of::<VR_IVRHeadsetView_FnTable>()
    );
    println!(
        "VR_IVRHeadsetView_FnTable.SetHeadsetViewSize={}",
        std::mem::offset_of!(VR_IVRHeadsetView_FnTable, SetHeadsetViewSize)
    );
    println!(
        "VR_IVRHeadsetView_FnTable.GetHeadsetViewSize={}",
        std::mem::offset_of!(VR_IVRHeadsetView_FnTable, GetHeadsetViewSize)
    );
    println!(
        "VR_IVRHeadsetView_FnTable.SetHeadsetViewMode={}",
        std::mem::offset_of!(VR_IVRHeadsetView_FnTable, SetHeadsetViewMode)
    );
    println!(
        "VR_IVRHeadsetView_FnTable.GetHeadsetViewMode={}",
        std::mem::offset_of!(VR_IVRHeadsetView_FnTable, GetHeadsetViewMode)
    );
    println!(
        "VR_IVRHeadsetView_FnTable.SetHeadsetViewCropped={}",
        std::mem::offset_of!(VR_IVRHeadsetView_FnTable, SetHeadsetViewCropped)
    );
    println!(
        "VR_IVRHeadsetView_FnTable.GetHeadsetViewCropped={}",
        std::mem::offset_of!(VR_IVRHeadsetView_FnTable, GetHeadsetViewCropped)
    );
    println!(
        "VR_IVRHeadsetView_FnTable.GetHeadsetViewAspectRatio={}",
        std::mem::offset_of!(VR_IVRHeadsetView_FnTable, GetHeadsetViewAspectRatio)
    );
    println!(
        "VR_IVRHeadsetView_FnTable.SetHeadsetViewBlendRange={}",
        std::mem::offset_of!(VR_IVRHeadsetView_FnTable, SetHeadsetViewBlendRange)
    );
    println!(
        "VR_IVRHeadsetView_FnTable.GetHeadsetViewBlendRange={}",
        std::mem::offset_of!(VR_IVRHeadsetView_FnTable, GetHeadsetViewBlendRange)
    );
    println!(
        "VR_IVRRenderModels_FnTable.size={}\nVR_IVRRenderModels_FnTable.align={}",
        size_of::<VR_IVRRenderModels_FnTable>(),
        align_of::<VR_IVRRenderModels_FnTable>()
    );
    println!(
        "VR_IVRRenderModels_FnTable.LoadRenderModel_Async={}",
        std::mem::offset_of!(VR_IVRRenderModels_FnTable, LoadRenderModel_Async)
    );
    println!(
        "VR_IVRRenderModels_FnTable.FreeRenderModel={}",
        std::mem::offset_of!(VR_IVRRenderModels_FnTable, FreeRenderModel)
    );
    println!(
        "VR_IVRRenderModels_FnTable.LoadTexture_Async={}",
        std::mem::offset_of!(VR_IVRRenderModels_FnTable, LoadTexture_Async)
    );
    println!(
        "VR_IVRRenderModels_FnTable.FreeTexture={}",
        std::mem::offset_of!(VR_IVRRenderModels_FnTable, FreeTexture)
    );
    println!(
        "VR_IVRRenderModels_FnTable.LoadTextureD3D11_Async={}",
        std::mem::offset_of!(VR_IVRRenderModels_FnTable, LoadTextureD3D11_Async)
    );
    println!(
        "VR_IVRRenderModels_FnTable.LoadIntoTextureD3D11_Async={}",
        std::mem::offset_of!(VR_IVRRenderModels_FnTable, LoadIntoTextureD3D11_Async)
    );
    println!(
        "VR_IVRRenderModels_FnTable.FreeTextureD3D11={}",
        std::mem::offset_of!(VR_IVRRenderModels_FnTable, FreeTextureD3D11)
    );
    println!(
        "VR_IVRRenderModels_FnTable.GetRenderModelName={}",
        std::mem::offset_of!(VR_IVRRenderModels_FnTable, GetRenderModelName)
    );
    println!(
        "VR_IVRRenderModels_FnTable.GetRenderModelCount={}",
        std::mem::offset_of!(VR_IVRRenderModels_FnTable, GetRenderModelCount)
    );
    println!(
        "VR_IVRRenderModels_FnTable.GetComponentCount={}",
        std::mem::offset_of!(VR_IVRRenderModels_FnTable, GetComponentCount)
    );
    println!(
        "VR_IVRRenderModels_FnTable.GetComponentName={}",
        std::mem::offset_of!(VR_IVRRenderModels_FnTable, GetComponentName)
    );
    println!(
        "VR_IVRRenderModels_FnTable.GetComponentButtonMask={}",
        std::mem::offset_of!(VR_IVRRenderModels_FnTable, GetComponentButtonMask)
    );
    println!(
        "VR_IVRRenderModels_FnTable.GetComponentRenderModelName={}",
        std::mem::offset_of!(VR_IVRRenderModels_FnTable, GetComponentRenderModelName)
    );
    println!(
        "VR_IVRRenderModels_FnTable.GetComponentStateForDevicePath={}",
        std::mem::offset_of!(VR_IVRRenderModels_FnTable, GetComponentStateForDevicePath)
    );
    println!(
        "VR_IVRRenderModels_FnTable.GetComponentState={}",
        std::mem::offset_of!(VR_IVRRenderModels_FnTable, GetComponentState)
    );
    println!(
        "VR_IVRRenderModels_FnTable.RenderModelHasComponent={}",
        std::mem::offset_of!(VR_IVRRenderModels_FnTable, RenderModelHasComponent)
    );
    println!(
        "VR_IVRRenderModels_FnTable.GetRenderModelThumbnailURL={}",
        std::mem::offset_of!(VR_IVRRenderModels_FnTable, GetRenderModelThumbnailURL)
    );
    println!(
        "VR_IVRRenderModels_FnTable.GetRenderModelOriginalPath={}",
        std::mem::offset_of!(VR_IVRRenderModels_FnTable, GetRenderModelOriginalPath)
    );
    println!(
        "VR_IVRRenderModels_FnTable.GetRenderModelErrorNameFromEnum={}",
        std::mem::offset_of!(VR_IVRRenderModels_FnTable, GetRenderModelErrorNameFromEnum)
    );
    println!(
        "VR_IVRNotifications_FnTable.size={}\nVR_IVRNotifications_FnTable.align={}",
        size_of::<VR_IVRNotifications_FnTable>(),
        align_of::<VR_IVRNotifications_FnTable>()
    );
    println!(
        "VR_IVRNotifications_FnTable.CreateNotification={}",
        std::mem::offset_of!(VR_IVRNotifications_FnTable, CreateNotification)
    );
    println!(
        "VR_IVRNotifications_FnTable.RemoveNotification={}",
        std::mem::offset_of!(VR_IVRNotifications_FnTable, RemoveNotification)
    );
    println!(
        "VR_IVRSettings_FnTable.size={}\nVR_IVRSettings_FnTable.align={}",
        size_of::<VR_IVRSettings_FnTable>(),
        align_of::<VR_IVRSettings_FnTable>()
    );
    println!(
        "VR_IVRSettings_FnTable.GetSettingsErrorNameFromEnum={}",
        std::mem::offset_of!(VR_IVRSettings_FnTable, GetSettingsErrorNameFromEnum)
    );
    println!(
        "VR_IVRSettings_FnTable.SetBool={}",
        std::mem::offset_of!(VR_IVRSettings_FnTable, SetBool)
    );
    println!(
        "VR_IVRSettings_FnTable.SetInt32={}",
        std::mem::offset_of!(VR_IVRSettings_FnTable, SetInt32)
    );
    println!(
        "VR_IVRSettings_FnTable.SetFloat={}",
        std::mem::offset_of!(VR_IVRSettings_FnTable, SetFloat)
    );
    println!(
        "VR_IVRSettings_FnTable.SetString={}",
        std::mem::offset_of!(VR_IVRSettings_FnTable, SetString)
    );
    println!(
        "VR_IVRSettings_FnTable.GetBool={}",
        std::mem::offset_of!(VR_IVRSettings_FnTable, GetBool)
    );
    println!(
        "VR_IVRSettings_FnTable.GetInt32={}",
        std::mem::offset_of!(VR_IVRSettings_FnTable, GetInt32)
    );
    println!(
        "VR_IVRSettings_FnTable.GetFloat={}",
        std::mem::offset_of!(VR_IVRSettings_FnTable, GetFloat)
    );
    println!(
        "VR_IVRSettings_FnTable.GetString={}",
        std::mem::offset_of!(VR_IVRSettings_FnTable, GetString)
    );
    println!(
        "VR_IVRSettings_FnTable.RemoveSection={}",
        std::mem::offset_of!(VR_IVRSettings_FnTable, RemoveSection)
    );
    println!(
        "VR_IVRSettings_FnTable.RemoveKeyInSection={}",
        std::mem::offset_of!(VR_IVRSettings_FnTable, RemoveKeyInSection)
    );
    println!(
        "VR_IVRScreenshots_FnTable.size={}\nVR_IVRScreenshots_FnTable.align={}",
        size_of::<VR_IVRScreenshots_FnTable>(),
        align_of::<VR_IVRScreenshots_FnTable>()
    );
    println!(
        "VR_IVRScreenshots_FnTable.RequestScreenshot={}",
        std::mem::offset_of!(VR_IVRScreenshots_FnTable, RequestScreenshot)
    );
    println!(
        "VR_IVRScreenshots_FnTable.HookScreenshot={}",
        std::mem::offset_of!(VR_IVRScreenshots_FnTable, HookScreenshot)
    );
    println!(
        "VR_IVRScreenshots_FnTable.GetScreenshotPropertyType={}",
        std::mem::offset_of!(VR_IVRScreenshots_FnTable, GetScreenshotPropertyType)
    );
    println!(
        "VR_IVRScreenshots_FnTable.GetScreenshotPropertyFilename={}",
        std::mem::offset_of!(VR_IVRScreenshots_FnTable, GetScreenshotPropertyFilename)
    );
    println!(
        "VR_IVRScreenshots_FnTable.UpdateScreenshotProgress={}",
        std::mem::offset_of!(VR_IVRScreenshots_FnTable, UpdateScreenshotProgress)
    );
    println!(
        "VR_IVRScreenshots_FnTable.TakeStereoScreenshot={}",
        std::mem::offset_of!(VR_IVRScreenshots_FnTable, TakeStereoScreenshot)
    );
    println!(
        "VR_IVRScreenshots_FnTable.SubmitScreenshot={}",
        std::mem::offset_of!(VR_IVRScreenshots_FnTable, SubmitScreenshot)
    );
    println!(
        "VR_IVRResources_FnTable.size={}\nVR_IVRResources_FnTable.align={}",
        size_of::<VR_IVRResources_FnTable>(),
        align_of::<VR_IVRResources_FnTable>()
    );
    println!(
        "VR_IVRResources_FnTable.LoadSharedResource={}",
        std::mem::offset_of!(VR_IVRResources_FnTable, LoadSharedResource)
    );
    println!(
        "VR_IVRResources_FnTable.GetResourceFullPath={}",
        std::mem::offset_of!(VR_IVRResources_FnTable, GetResourceFullPath)
    );
    println!(
        "VR_IVRDriverManager_FnTable.size={}\nVR_IVRDriverManager_FnTable.align={}",
        size_of::<VR_IVRDriverManager_FnTable>(),
        align_of::<VR_IVRDriverManager_FnTable>()
    );
    println!(
        "VR_IVRDriverManager_FnTable.GetDriverCount={}",
        std::mem::offset_of!(VR_IVRDriverManager_FnTable, GetDriverCount)
    );
    println!(
        "VR_IVRDriverManager_FnTable.GetDriverName={}",
        std::mem::offset_of!(VR_IVRDriverManager_FnTable, GetDriverName)
    );
    println!(
        "VR_IVRDriverManager_FnTable.GetDriverHandle={}",
        std::mem::offset_of!(VR_IVRDriverManager_FnTable, GetDriverHandle)
    );
    println!(
        "VR_IVRDriverManager_FnTable.IsEnabled={}",
        std::mem::offset_of!(VR_IVRDriverManager_FnTable, IsEnabled)
    );
    println!(
        "VR_IVRInput_FnTable.size={}\nVR_IVRInput_FnTable.align={}",
        size_of::<VR_IVRInput_FnTable>(),
        align_of::<VR_IVRInput_FnTable>()
    );
    println!(
        "VR_IVRInput_FnTable.SetActionManifestPath={}",
        std::mem::offset_of!(VR_IVRInput_FnTable, SetActionManifestPath)
    );
    println!(
        "VR_IVRInput_FnTable.GetActionSetHandle={}",
        std::mem::offset_of!(VR_IVRInput_FnTable, GetActionSetHandle)
    );
    println!(
        "VR_IVRInput_FnTable.GetActionHandle={}",
        std::mem::offset_of!(VR_IVRInput_FnTable, GetActionHandle)
    );
    println!(
        "VR_IVRInput_FnTable.GetInputSourceHandle={}",
        std::mem::offset_of!(VR_IVRInput_FnTable, GetInputSourceHandle)
    );
    println!(
        "VR_IVRInput_FnTable.UpdateActionState={}",
        std::mem::offset_of!(VR_IVRInput_FnTable, UpdateActionState)
    );
    println!(
        "VR_IVRInput_FnTable.GetDigitalActionData={}",
        std::mem::offset_of!(VR_IVRInput_FnTable, GetDigitalActionData)
    );
    println!(
        "VR_IVRInput_FnTable.GetAnalogActionData={}",
        std::mem::offset_of!(VR_IVRInput_FnTable, GetAnalogActionData)
    );
    println!(
        "VR_IVRInput_FnTable.GetPoseActionDataRelativeToNow={}",
        std::mem::offset_of!(VR_IVRInput_FnTable, GetPoseActionDataRelativeToNow)
    );
    println!(
        "VR_IVRInput_FnTable.GetPoseActionDataForNextFrame={}",
        std::mem::offset_of!(VR_IVRInput_FnTable, GetPoseActionDataForNextFrame)
    );
    println!(
        "VR_IVRInput_FnTable.GetSkeletalActionData={}",
        std::mem::offset_of!(VR_IVRInput_FnTable, GetSkeletalActionData)
    );
    println!(
        "VR_IVRInput_FnTable.GetDominantHand={}",
        std::mem::offset_of!(VR_IVRInput_FnTable, GetDominantHand)
    );
    println!(
        "VR_IVRInput_FnTable.SetDominantHand={}",
        std::mem::offset_of!(VR_IVRInput_FnTable, SetDominantHand)
    );
    println!(
        "VR_IVRInput_FnTable.GetEyeTrackingDataRelativeToNow={}",
        std::mem::offset_of!(VR_IVRInput_FnTable, GetEyeTrackingDataRelativeToNow)
    );
    println!(
        "VR_IVRInput_FnTable.GetEyeTrackingDataForNextFrame={}",
        std::mem::offset_of!(VR_IVRInput_FnTable, GetEyeTrackingDataForNextFrame)
    );
    println!(
        "VR_IVRInput_FnTable.GetBoneCount={}",
        std::mem::offset_of!(VR_IVRInput_FnTable, GetBoneCount)
    );
    println!(
        "VR_IVRInput_FnTable.GetBoneHierarchy={}",
        std::mem::offset_of!(VR_IVRInput_FnTable, GetBoneHierarchy)
    );
    println!(
        "VR_IVRInput_FnTable.GetBoneName={}",
        std::mem::offset_of!(VR_IVRInput_FnTable, GetBoneName)
    );
    println!(
        "VR_IVRInput_FnTable.GetSkeletalReferenceTransforms={}",
        std::mem::offset_of!(VR_IVRInput_FnTable, GetSkeletalReferenceTransforms)
    );
    println!(
        "VR_IVRInput_FnTable.GetSkeletalTrackingLevel={}",
        std::mem::offset_of!(VR_IVRInput_FnTable, GetSkeletalTrackingLevel)
    );
    println!(
        "VR_IVRInput_FnTable.GetSkeletalBoneData={}",
        std::mem::offset_of!(VR_IVRInput_FnTable, GetSkeletalBoneData)
    );
    println!(
        "VR_IVRInput_FnTable.GetSkeletalSummaryData={}",
        std::mem::offset_of!(VR_IVRInput_FnTable, GetSkeletalSummaryData)
    );
    println!(
        "VR_IVRInput_FnTable.GetSkeletalBoneDataCompressed={}",
        std::mem::offset_of!(VR_IVRInput_FnTable, GetSkeletalBoneDataCompressed)
    );
    println!(
        "VR_IVRInput_FnTable.DecompressSkeletalBoneData={}",
        std::mem::offset_of!(VR_IVRInput_FnTable, DecompressSkeletalBoneData)
    );
    println!(
        "VR_IVRInput_FnTable.TriggerHapticVibrationAction={}",
        std::mem::offset_of!(VR_IVRInput_FnTable, TriggerHapticVibrationAction)
    );
    println!(
        "VR_IVRInput_FnTable.GetActionOrigins={}",
        std::mem::offset_of!(VR_IVRInput_FnTable, GetActionOrigins)
    );
    println!(
        "VR_IVRInput_FnTable.GetOriginLocalizedName={}",
        std::mem::offset_of!(VR_IVRInput_FnTable, GetOriginLocalizedName)
    );
    println!(
        "VR_IVRInput_FnTable.GetOriginTrackedDeviceInfo={}",
        std::mem::offset_of!(VR_IVRInput_FnTable, GetOriginTrackedDeviceInfo)
    );
    println!(
        "VR_IVRInput_FnTable.GetActionBindingInfo={}",
        std::mem::offset_of!(VR_IVRInput_FnTable, GetActionBindingInfo)
    );
    println!(
        "VR_IVRInput_FnTable.ShowActionOrigins={}",
        std::mem::offset_of!(VR_IVRInput_FnTable, ShowActionOrigins)
    );
    println!(
        "VR_IVRInput_FnTable.ShowBindingsForActionSet={}",
        std::mem::offset_of!(VR_IVRInput_FnTable, ShowBindingsForActionSet)
    );
    println!(
        "VR_IVRInput_FnTable.GetComponentStateForBinding={}",
        std::mem::offset_of!(VR_IVRInput_FnTable, GetComponentStateForBinding)
    );
    println!(
        "VR_IVRInput_FnTable.IsUsingLegacyInput={}",
        std::mem::offset_of!(VR_IVRInput_FnTable, IsUsingLegacyInput)
    );
    println!(
        "VR_IVRInput_FnTable.OpenBindingUI={}",
        std::mem::offset_of!(VR_IVRInput_FnTable, OpenBindingUI)
    );
    println!(
        "VR_IVRInput_FnTable.GetBindingVariant={}",
        std::mem::offset_of!(VR_IVRInput_FnTable, GetBindingVariant)
    );
    println!(
        "VR_IVRIOBuffer_FnTable.size={}\nVR_IVRIOBuffer_FnTable.align={}",
        size_of::<VR_IVRIOBuffer_FnTable>(),
        align_of::<VR_IVRIOBuffer_FnTable>()
    );
    println!(
        "VR_IVRIOBuffer_FnTable.Open={}",
        std::mem::offset_of!(VR_IVRIOBuffer_FnTable, Open)
    );
    println!(
        "VR_IVRIOBuffer_FnTable.Close={}",
        std::mem::offset_of!(VR_IVRIOBuffer_FnTable, Close)
    );
    println!(
        "VR_IVRIOBuffer_FnTable.Read={}",
        std::mem::offset_of!(VR_IVRIOBuffer_FnTable, Read)
    );
    println!(
        "VR_IVRIOBuffer_FnTable.Write={}",
        std::mem::offset_of!(VR_IVRIOBuffer_FnTable, Write)
    );
    println!(
        "VR_IVRIOBuffer_FnTable.PropertyContainer={}",
        std::mem::offset_of!(VR_IVRIOBuffer_FnTable, PropertyContainer)
    );
    println!(
        "VR_IVRIOBuffer_FnTable.HasReaders={}",
        std::mem::offset_of!(VR_IVRIOBuffer_FnTable, HasReaders)
    );
    println!(
        "VR_IVRSpatialAnchors_FnTable.size={}\nVR_IVRSpatialAnchors_FnTable.align={}",
        size_of::<VR_IVRSpatialAnchors_FnTable>(),
        align_of::<VR_IVRSpatialAnchors_FnTable>()
    );
    println!(
        "VR_IVRSpatialAnchors_FnTable.CreateSpatialAnchorFromDescriptor={}",
        std::mem::offset_of!(
            VR_IVRSpatialAnchors_FnTable,
            CreateSpatialAnchorFromDescriptor
        )
    );
    println!(
        "VR_IVRSpatialAnchors_FnTable.CreateSpatialAnchorFromPose={}",
        std::mem::offset_of!(VR_IVRSpatialAnchors_FnTable, CreateSpatialAnchorFromPose)
    );
    println!(
        "VR_IVRSpatialAnchors_FnTable.GetSpatialAnchorPose={}",
        std::mem::offset_of!(VR_IVRSpatialAnchors_FnTable, GetSpatialAnchorPose)
    );
    println!(
        "VR_IVRSpatialAnchors_FnTable.GetSpatialAnchorDescriptor={}",
        std::mem::offset_of!(VR_IVRSpatialAnchors_FnTable, GetSpatialAnchorDescriptor)
    );
    println!(
        "VR_IVRDebug_FnTable.size={}\nVR_IVRDebug_FnTable.align={}",
        size_of::<VR_IVRDebug_FnTable>(),
        align_of::<VR_IVRDebug_FnTable>()
    );
    println!(
        "VR_IVRDebug_FnTable.EmitVrProfilerEvent={}",
        std::mem::offset_of!(VR_IVRDebug_FnTable, EmitVrProfilerEvent)
    );
    println!(
        "VR_IVRDebug_FnTable.BeginVrProfilerEvent={}",
        std::mem::offset_of!(VR_IVRDebug_FnTable, BeginVrProfilerEvent)
    );
    println!(
        "VR_IVRDebug_FnTable.FinishVrProfilerEvent={}",
        std::mem::offset_of!(VR_IVRDebug_FnTable, FinishVrProfilerEvent)
    );
    println!(
        "VR_IVRDebug_FnTable.DriverDebugRequest={}",
        std::mem::offset_of!(VR_IVRDebug_FnTable, DriverDebugRequest)
    );
    println!(
        "VR_IVRIPCResourceManagerClient_FnTable.size={}\nVR_IVRIPCResourceManagerClient_FnTable.align={}",
        size_of::<VR_IVRIPCResourceManagerClient_FnTable>(),
        align_of::<VR_IVRIPCResourceManagerClient_FnTable>()
    );
    println!(
        "VR_IVRIPCResourceManagerClient_FnTable.NewSharedVulkanImage={}",
        std::mem::offset_of!(VR_IVRIPCResourceManagerClient_FnTable, NewSharedVulkanImage)
    );
    println!(
        "VR_IVRIPCResourceManagerClient_FnTable.NewSharedVulkanBuffer={}",
        std::mem::offset_of!(
            VR_IVRIPCResourceManagerClient_FnTable,
            NewSharedVulkanBuffer
        )
    );
    println!(
        "VR_IVRIPCResourceManagerClient_FnTable.NewSharedVulkanSemaphore={}",
        std::mem::offset_of!(
            VR_IVRIPCResourceManagerClient_FnTable,
            NewSharedVulkanSemaphore
        )
    );
    println!(
        "VR_IVRIPCResourceManagerClient_FnTable.RefResource={}",
        std::mem::offset_of!(VR_IVRIPCResourceManagerClient_FnTable, RefResource)
    );
    println!(
        "VR_IVRIPCResourceManagerClient_FnTable.UnrefResource={}",
        std::mem::offset_of!(VR_IVRIPCResourceManagerClient_FnTable, UnrefResource)
    );
    println!(
        "VR_IVRIPCResourceManagerClient_FnTable.GetDmabufFormats={}",
        std::mem::offset_of!(VR_IVRIPCResourceManagerClient_FnTable, GetDmabufFormats)
    );
    println!(
        "VR_IVRIPCResourceManagerClient_FnTable.GetDmabufModifiers={}",
        std::mem::offset_of!(VR_IVRIPCResourceManagerClient_FnTable, GetDmabufModifiers)
    );
    println!(
        "VR_IVRIPCResourceManagerClient_FnTable.ImportDmabuf={}",
        std::mem::offset_of!(VR_IVRIPCResourceManagerClient_FnTable, ImportDmabuf)
    );
    println!(
        "VR_IVRIPCResourceManagerClient_FnTable.ReceiveSharedFd={}",
        std::mem::offset_of!(VR_IVRIPCResourceManagerClient_FnTable, ReceiveSharedFd)
    );
    println!(
        "VR_IVRIPCResourceManagerClient_FnTable.DestructIVRIPCResourceManagerClient={}",
        std::mem::offset_of!(
            VR_IVRIPCResourceManagerClient_FnTable,
            DestructIVRIPCResourceManagerClient
        )
    );
    println!(
        "VR_IVRProperties_FnTable.size={}\nVR_IVRProperties_FnTable.align={}",
        size_of::<VR_IVRProperties_FnTable>(),
        align_of::<VR_IVRProperties_FnTable>()
    );
    println!(
        "VR_IVRProperties_FnTable.ReadPropertyBatch={}",
        std::mem::offset_of!(VR_IVRProperties_FnTable, ReadPropertyBatch)
    );
    println!(
        "VR_IVRProperties_FnTable.WritePropertyBatch={}",
        std::mem::offset_of!(VR_IVRProperties_FnTable, WritePropertyBatch)
    );
    println!(
        "VR_IVRProperties_FnTable.GetPropErrorNameFromEnum={}",
        std::mem::offset_of!(VR_IVRProperties_FnTable, GetPropErrorNameFromEnum)
    );
    println!(
        "VR_IVRProperties_FnTable.TrackedDeviceToPropertyContainer={}",
        std::mem::offset_of!(VR_IVRProperties_FnTable, TrackedDeviceToPropertyContainer)
    );
    println!(
        "VR_IVRPaths_FnTable.size={}\nVR_IVRPaths_FnTable.align={}",
        size_of::<VR_IVRPaths_FnTable>(),
        align_of::<VR_IVRPaths_FnTable>()
    );
    println!(
        "VR_IVRPaths_FnTable.ReadPathBatch={}",
        std::mem::offset_of!(VR_IVRPaths_FnTable, ReadPathBatch)
    );
    println!(
        "VR_IVRPaths_FnTable.WritePathBatch={}",
        std::mem::offset_of!(VR_IVRPaths_FnTable, WritePathBatch)
    );
    println!(
        "VR_IVRPaths_FnTable.StringToHandle={}",
        std::mem::offset_of!(VR_IVRPaths_FnTable, StringToHandle)
    );
    println!(
        "VR_IVRPaths_FnTable.HandleToString={}",
        std::mem::offset_of!(VR_IVRPaths_FnTable, HandleToString)
    );
    println!(
        "VR_IVRBlockQueue_FnTable.size={}\nVR_IVRBlockQueue_FnTable.align={}",
        size_of::<VR_IVRBlockQueue_FnTable>(),
        align_of::<VR_IVRBlockQueue_FnTable>()
    );
    println!(
        "VR_IVRBlockQueue_FnTable.Create={}",
        std::mem::offset_of!(VR_IVRBlockQueue_FnTable, Create)
    );
    println!(
        "VR_IVRBlockQueue_FnTable.Connect={}",
        std::mem::offset_of!(VR_IVRBlockQueue_FnTable, Connect)
    );
    println!(
        "VR_IVRBlockQueue_FnTable.Destroy={}",
        std::mem::offset_of!(VR_IVRBlockQueue_FnTable, Destroy)
    );
    println!(
        "VR_IVRBlockQueue_FnTable.AcquireWriteOnlyBlock={}",
        std::mem::offset_of!(VR_IVRBlockQueue_FnTable, AcquireWriteOnlyBlock)
    );
    println!(
        "VR_IVRBlockQueue_FnTable.ReleaseWriteOnlyBlock={}",
        std::mem::offset_of!(VR_IVRBlockQueue_FnTable, ReleaseWriteOnlyBlock)
    );
    println!(
        "VR_IVRBlockQueue_FnTable.WaitAndAcquireReadOnlyBlock={}",
        std::mem::offset_of!(VR_IVRBlockQueue_FnTable, WaitAndAcquireReadOnlyBlock)
    );
    println!(
        "VR_IVRBlockQueue_FnTable.AcquireReadOnlyBlock={}",
        std::mem::offset_of!(VR_IVRBlockQueue_FnTable, AcquireReadOnlyBlock)
    );
    println!(
        "VR_IVRBlockQueue_FnTable.ReleaseReadOnlyBlock={}",
        std::mem::offset_of!(VR_IVRBlockQueue_FnTable, ReleaseReadOnlyBlock)
    );
    println!(
        "VR_IVRBlockQueue_FnTable.QueueHasReader={}",
        std::mem::offset_of!(VR_IVRBlockQueue_FnTable, QueueHasReader)
    );
}
