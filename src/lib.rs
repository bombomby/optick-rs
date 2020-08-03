//! [Optick Profiler](https://github.com/bombomby/optick) - Super Lightweight Performance Profiler
//!
//! ## How to use
//!
//! In `Cargo.toml` add:
//!
//! ```toml
//! [dependencies]
//! optick = "1.3.1"
//! ```
//!
//! Example usage:
//!
//! ```rust
//! fn calc(n: u32) {
//! 	// Profile current scope (automatically extracts current function name)
//!		// You could also specify a custom name if needed - e.g. optick::scope!("calc");
//!     optick::event!();
//! 
//! 	// Attach custom data tag to the capture (i32, u32, u64, f32, str, vec3)
//!     optick::tag!("number", n);
//!     optick::tag!("name", "Bob");
//!     optick::tag!("position", (10.0f32, -12.0f32, 14.0f32));
//! 
//!		...
//! }
//! 
//! pub fn main() {
//! 	// Start a new capture
//!     optick::start_capture();
//!     
//! 	calc(42);
//! 	
//! 	// Stop and save current capture 
//!     optick::stop_capture("capture_name"); // => Saves capture to {working_dir}/capture_name(date-time).opt
//! }
//! ```
//! 
//! ## GUI
//!
//! Use Optick GUI to open saved *.opt capture for further analysis:
//! https://github.com/bombomby/optick/releases
//!
//! ## Feature flags
//!
//! - `enable` - this flag is used by default and enables Optick instrumentation
//! 
//! ## Run as Administartor to collect ETW events
//! Optick uses ETW to collect hardware counters: switch-contexts, auto-sampling, CPU core utilization, etc.
//! Run your app as administrator to enable the collection of ETW events:
//! ```
//! Start-Process cargo run -Verb runAs
//! ```
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

mod optick_api;

/// Instrument current scope
/// 
/// Note: You could override the name of the function by passing a custom name
///
/// Example: 
///
/// ```rust
/// fn calc() {
///     optick::event!();
///		...
/// }
/// ```
#[macro_export]
#[cfg(all(feature = "enable"))]
macro_rules! event {
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


/// Attach custom data to the current scope
/// 
/// Note: You could override the name of the function by passing a custom name
///
/// Example: 
///
/// ```rust
/// fn calc(number: u32, name: &str) {
///     optick::event!();
///     optick::tag!("number", number);
///     optick::tag!("name", name);
///		...
/// }
/// ```
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

/// Extract current function name
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

/// Scoped event
#[cfg(all(feature = "enable"))]
pub struct OptickCounter {
    /// ID of the current event description
    pub event_data : u64,
}

/// Destructor for scoped event
#[cfg(all(feature = "enable"))]
impl Drop for OptickCounter {
    #[inline(always)]
    fn drop(&mut self) {
        if self.event_data != 0 {
            pop_event(self.event_data);
        }
    }
}

/// Check whether profiler is enabled
pub const fn enabled() -> bool {
    cfg!(all(feature = "enable"))
}

/// Create event description
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

/// Push profiling event
pub fn push_event(_description: u64) -> u64 {
    #[cfg(all(feature = "enable"))]
    unsafe {
        optick_api::OptickAPI_PushEvent(_description)
    }
}

/// Pop profiling event
pub fn pop_event(_event_data: u64) {
    #[cfg(all(feature = "enable"))]
    unsafe {
        optick_api::OptickAPI_PopEvent(_event_data); 
    }
}

/// Mark frame update
///
/// Example: 
///
/// ```rust
/// fn update() {
///     optick::next_frame();
///		...
/// }
/// ```
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

/// Register thread for profiling
///
/// Example: 
///
/// ```rust
/// optick::register_thread("Thread Name");
/// ```
pub fn register_thread(thread: &str) {
    #[cfg(all(feature = "enable"))]
    unsafe {
        optick_api::OptickAPI_RegisterThread(
            thread.as_ptr() as *const i8,
            thread.len() as u16,
        )
    }
}

/// Start a new capture
pub fn start_capture() {
    #[cfg(all(feature = "enable"))]
    {
        register_thread("MainThread");
        unsafe {
            optick_api::OptickAPI_StartCapture();
        }
    }
}

/// Stop and save current capture to the specified path
///
/// Example: 
///
/// ```rust
/// pub fn main() {
///     optick::start_capture();
/// 	calc(42);
///     optick::stop_capture("capture_name"); // => {working_dir}/capture_name(2020-07-24.02-33-19).opt
/// }
/// ```
pub fn stop_capture(path: &str) {
    #[cfg(all(feature = "enable"))]
    unsafe {
        optick_api::OptickAPI_StopCapture(path.as_ptr() as *const i8, path.len() as u16);
    }
}

/// Optick Tag
pub trait OptickTag {
    /// Attach tag
    fn attach(&self, _description: u64);
}

/// Optick Tag: &str
impl OptickTag for &str {
    fn attach(&self, description: u64) {
        unsafe {
            optick_api::OptickAPI_AttachTag_String(description, (*self).as_ptr() as *const i8, (*self).len() as u16);
        }     
    }
}

/// Optick Tag: String
impl OptickTag for String {
    fn attach(&self, description: u64) {
        unsafe {
            optick_api::OptickAPI_AttachTag_String(description, (*self).as_ptr() as *const i8, (*self).len() as u16);
        }     
    }
}

/// Optick Tag: i32
impl OptickTag for i32 {
    fn attach(&self, description: u64) {
        unsafe {
            optick_api::OptickAPI_AttachTag_Int32(description, *self);
        }     
    }
}

/// Optick Tag: u32
impl OptickTag for u32 {
    fn attach(&self, description: u64) {
        unsafe {
            optick_api::OptickAPI_AttachTag_UInt32(description, *self);
        }     
    }
}

/// Optick Tag: u64
impl OptickTag for u64 {
    fn attach(&self, description: u64) {
        unsafe {
            optick_api::OptickAPI_AttachTag_UInt64(description, *self);
        }     
    }
}

/// Optick Tag: f32
impl OptickTag for f32 {
    fn attach(&self, description: u64) {
        unsafe {
            optick_api::OptickAPI_AttachTag_Float(description, *self);
        }     
    }
}

/// Optick Tag: vec3 (f32, f32, f32)
impl OptickTag for (f32, f32, f32) {
    fn attach(&self, description: u64) {
        unsafe {
            optick_api::OptickAPI_AttachTag_Point(description, (*self).0, (*self).1, (*self).2);
        }     
    }
}

// fn attach(n: &str, description: u64)
// {
//     n.attach(description);
// }
