fn fibonacci(n: u32) -> u32 {
    optick::event!();
    let res = match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    };
    return res;
}

pub fn main() {
    optick::start_capture();
    fibonacci(30);
    optick::stop_capture("capture_rust.opt");
}