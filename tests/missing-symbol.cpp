#ifdef _WIN32
extern "C" __declspec(dllexport) void UnrelatedExport() {}
#else
extern "C" __attribute__((visibility("default"))) void UnrelatedExport() {}
#endif
