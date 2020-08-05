fn update(frame_num: u32) {
    optick::event!();
    optick::tag!("frame", frame_num);

    use std::{thread, time};
    let duration = time::Duration::from_millis(33);
    thread::sleep(duration);
}

pub fn main() {
    let mut frame = 0;
    loop {
        optick::next_frame();
        update(frame);
        frame = frame + 1;
    }
}
