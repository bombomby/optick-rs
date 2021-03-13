bindgen optick/src/optick_capi.h -o src/optick_api.rs \
  \
  --whitelist-function OptickAPI_RegisterThread \
  --whitelist-function OptickAPI_CreateEventDescription \
  --whitelist-function OptickAPI_PushEvent \
  --whitelist-function OptickAPI_PopEvent \
  --whitelist-function OptickAPI_NextFrame \
  --whitelist-function OptickAPI_StartCapture \
  --whitelist-function OptickAPI_StopCapture \
  --whitelist-function OptickAPI_AttachTag_String \
  --whitelist-function OptickAPI_AttachTag_Int32 \
  --whitelist-function OptickAPI_AttachTag_Float \
  --whitelist-function OptickAPI_AttachTag_UInt32 \
  --whitelist-function OptickAPI_AttachTag_UInt64 \
  --whitelist-function OptickAPI_AttachTag_Point \
  \
  -- -x c++ -std=c++11