## Rust API for [Optick Profiler](https://github.com/bombomby/optick)
[![Build Status](https://github.com/bombomby/optick-rs/workflows/Rust/badge.svg)](https://github.com/bombomby/optick-rs/actions?workflow=Rust)
[![Crates.io](https://img.shields.io/crates/v/optick.svg)](https://crates.io/crates/optick)
[![Docs](https://docs.rs/optick/badge.svg)](https://docs.rs/optick)

![](https://optick.dev/images/screenshots/optick/Optick.png)

## Supported Platforms
| Windows (x64) | Linux | MacOS |
| ------------- | ----- | ----- |
| :heavy_check_mark: | :hourglass_flowing_sand: soon | :hourglass_flowing_sand: soon |

## How to use

In `Cargo.toml` add:

```toml
[dependencies]
optick = "1.3.3"
```

Example 1 (generic app, automation, etc.):

```rust
fn calc(n: u32) {
	// Profile current scope (automatically extracts current function name)
	// You could also specify a custom name if needed - e.g. optick::scope!("calc");
	optick::event!();

	// Attach custom data tag to the capture (i32, u32, u64, f32, str, vec3)
	optick::tag!("number", n);
	optick::tag!("name", "Bob");
	optick::tag!("position", (10.0f32, -12.0f32, 14.0f32));

	...
}

pub fn main() {
	// Start a new capture
	optick::start_capture();

	calc(42);
	
	// Stop and save current capture to {working_dir}/capture_name(date-time).opt
	optick::stop_capture("capture_name"); 
}
```

Example 2 (game):

```rust
fn update(frame_num: u32) {
    optick::event!();
    optick::tag!("frame", frame_num);
    std::thread::sleep(std::time::Duration::from_millis(33));
}

pub fn main() {
    let mut frame = 0; 
    loop {
        optick::next_frame();
        update(frame);
        frame = frame + 1;
    }
}
```

## GUI

Use Optick GUI to open saved *.opt capture for further analysis:
https://github.com/bombomby/optick/releases <br/>
After grabbing the latest available Optick_vX.X.X.zip => launch Optick.exe.

## Video Tutorial (Features Overview)
[![Optick Video Tutorial](https://github.com/bombomby/brofiler/blob/gh-pages/images/VideoThumbnail.jpg)](https://www.youtube.com/watch?v=p57TV5342fo)

## Procedural Macros
Optick supports a set of procedural macros for simplifying the process of code markup:
https://crates.io/crates/optick-attr
```rust
// Instrument current function
#[optick_attr::profile]
fn calc() {
    // Do some stuff
}
//Generate performance capture for function to {working_dir}/capture_name(date-time).opt.
#[optick_attr::capture("capture_name")]
pub fn main() {
    calc();
}
```

## Feature flags

- `enable` - this flag is used by default and enables Optick instrumentation

## Run as Administartor to collect ETW events
Optick uses ETW to collect hardware counters: switch-contexts, auto-sampling, CPU core utilization, etc.
Run your app as administrator to enable the collection of ETW events:
```
Start-Process cargo run -Verb runAs
```
