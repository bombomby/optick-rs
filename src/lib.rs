mod optick_api;

#[macro_export]
#[cfg(all(feature = "enable"))]
macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        &name[..name.len() - 3]
    }}
}

#[macro_export]
#[cfg(all(feature = "enable"))]
macro_rules! optick_event {
    () => {
        static mut _OPTICK_EVENT_DESCRIPTION : u64 = 0;
        let mut _optick_counter = $crate::OptickCounter { event_data: 0 };
        unsafe {
            if _OPTICK_EVENT_DESCRIPTION == 0 {
                _OPTICK_EVENT_DESCRIPTION = $crate::create_description($crate::function!(), file!(), line!());
            } 
            _optick_counter.event_data = $crate::push_event(_OPTICK_EVENT_DESCRIPTION);
        }
    };
    ($name:expr) => {
        static mut _OPTICK_EVENT_DESCRIPTION : u64 = 0;
        let mut _optick_counter = $crate::OptickCounter { event_data: 0 };
        unsafe {
            if _OPTICK_EVENT_DESCRIPTION == 0 {
                _OPTICK_EVENT_DESCRIPTION = $crate::create_description($name, file!(), line!());
            } 
            _optick_counter.event_data = $crate::push_event(_OPTICK_EVENT_DESCRIPTION);
        }
    };
}

#[macro_export]
#[cfg(all(feature = "enable"))]
macro_rules! tag {
    ($name:expr, $value:expr) => {{
        use $crate::OptickTag;
        static mut _OPTICK_EVENT_DESCRIPTION : u64 = 0;
        let mut _optick_counter = $crate::OptickCounter { event_data: 0 };
        unsafe {
            if _OPTICK_EVENT_DESCRIPTION == 0 {
                _OPTICK_EVENT_DESCRIPTION = $crate::create_description($name, file!(), line!());
            } 
            $value.attach(_OPTICK_EVENT_DESCRIPTION);
        }}
    };
}

#[cfg(all(feature = "enable"))]
pub struct OptickCounter {
    pub event_data : u64,
}

#[cfg(all(feature = "enable"))]
impl Drop for OptickCounter {
    #[inline(always)]
    fn drop(&mut self) {
        if self.event_data != 0 {
            pop_event(self.event_data);
        }
    }
}

pub const fn enabled() -> bool {
    cfg!(all(feature = "enable"))
}

pub fn create_description(name: &str, file: &str, line: u32) -> u64 {
    #[cfg(all(feature = "enable"))]
    unsafe {
        optick_api::OptickAPI_CreateEventDescription(
            name.as_ptr() as *const i8,
            name.len() as u16,
            file.as_ptr() as *const i8,
            file.len() as u16,
            line
        )
    }
}

pub fn push_event(_description: u64) -> u64 {
    #[cfg(all(feature = "enable"))]
    unsafe {
        optick_api::OptickAPI_PushEvent(_description)
    }
}

pub fn pop_event(_event_data: u64) {
    #[cfg(all(feature = "enable"))]
    unsafe {
        optick_api::OptickAPI_PopEvent(_event_data); 
    }
}

pub fn next_frame() {
    #[cfg(all(feature = "enable"))]
    unsafe {
        static mut _OPTICK_INIT_ONCE : bool = false;
        if _OPTICK_INIT_ONCE == false {
            register_thread("MainThread");
            _OPTICK_INIT_ONCE = true;
        }
        optick_api::OptickAPI_NextFrame();
    }
}

pub fn register_thread(thread: &str) {
    #[cfg(all(feature = "enable"))]
    unsafe {
        optick_api::OptickAPI_RegisterThread(
            thread.as_ptr() as *const i8,
            thread.len() as u16,
        )
    }
}

pub fn start_capture() {
    #[cfg(all(feature = "enable"))]
    {
        register_thread("MainThread");
        unsafe {
            optick_api::OptickAPI_StartCapture();
        }
    }
}

pub fn stop_capture(path: &str) {
    #[cfg(all(feature = "enable"))]
    unsafe {
        optick_api::OptickAPI_StopCapture(path.as_ptr() as *const i8, path.len() as u16);
    }
}

pub trait OptickTag {
    fn attach(&self, _description: u64);
}

impl OptickTag for &str {
    fn attach(&self, description: u64) {
        unsafe {
            optick_api::OptickAPI_AttachTag_String(description, (*self).as_ptr() as *const i8, (*self).len() as u16);
        }     
    }
}

impl OptickTag for String {
    fn attach(&self, description: u64) {
        unsafe {
            optick_api::OptickAPI_AttachTag_String(description, (*self).as_ptr() as *const i8, (*self).len() as u16);
        }     
    }
}

impl OptickTag for i32 {
    fn attach(&self, description: u64) {
        unsafe {
            optick_api::OptickAPI_AttachTag_Int32(description, *self);
        }     
    }
}

impl OptickTag for u32 {
    fn attach(&self, description: u64) {
        unsafe {
            optick_api::OptickAPI_AttachTag_UInt32(description, *self);
        }     
    }
}

impl OptickTag for u64 {
    fn attach(&self, description: u64) {
        unsafe {
            optick_api::OptickAPI_AttachTag_UInt64(description, *self);
        }     
    }
}

impl OptickTag for f32 {
    fn attach(&self, description: u64) {
        unsafe {
            optick_api::OptickAPI_AttachTag_Float(description, *self);
        }     
    }
}

// impl OptickTag for (f32, f32, f32) {
//     fn attach(&self, description: u64) {
//         unsafe {
//             optick_api::OptickAPI_AttachTag_Point(description, (*self).0, (*self).1, (*self).2);
//         }     
//     }
// }

pub fn attach(n: &str, description: u64)
{
    n.attach(description);
}
