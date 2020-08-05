use std::os::raw::c_char;

extern "C" {
    pub fn OptickAPI_RegisterThread(in_thread_name: *const c_char, in_thread_name_length: u16);

    pub fn OptickAPI_CreateEventDescription(
        in_function_name: *const c_char,
        in_function_name_length: u16,
        in_file_name: *const c_char,
        in_file_name_length: u16,
        in_file_line: u32,
    ) -> u64;

    pub fn OptickAPI_NextFrame();

    pub fn OptickAPI_PushEvent(in_event_description_id: u64) -> u64;

    pub fn OptickAPI_PopEvent(in_event_data: u64);

    pub fn OptickAPI_StartCapture();

    pub fn OptickAPI_StopCapture(in_capture_name: *const c_char, in_capture_name_length: u16);

    pub fn OptickAPI_AttachTag_String(
        in_event_description_id: u64,
        in_value: *const c_char,
        in_value_length: u16,
    );

    pub fn OptickAPI_AttachTag_Int32(in_event_description_id: u64, in_value: i32);

    pub fn OptickAPI_AttachTag_UInt32(in_event_description_id: u64, in_value: u32);

    pub fn OptickAPI_AttachTag_Float(in_event_description_id: u64, in_value: f32);

    pub fn OptickAPI_AttachTag_UInt64(in_event_description_id: u64, in_value: u64);

    pub fn OptickAPI_AttachTag_Point(in_event_description_id: u64, in_x: f32, in_y: f32, in_z: f32);

}
