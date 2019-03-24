use school_clock::SchoolClock;
use std::thread::sleep;
use time::Duration;

const CLOCK_FORMAT: &str = "%H:%M:%S";
const DELAY: std::time::Duration = std::time::Duration::from_millis(200);

fn main() {
    let now = {
        let mut now = time::now();
        now.tm_min = 0;
        now.tm_sec = 0;
        now.tm_nsec = 0;
        now
    };
    let clock = SchoolClock::new(
        Duration::minutes(1),
        Duration::seconds(54),
        Duration::seconds(58),
        now,
    )
    .expect("must have successfully created clock");

    let mut same_line;
    loop {
        same_line = false;
        if let Ok((_, y)) = term_cursor::get_pos() {
            if term_cursor::set_pos(0, y).is_ok() {
                same_line = true;
            }
        }
        if !same_line {
            println!(); // create a new line to not have two times on same line
        }

        print!(
            "{}",
            clock
                .at(time::now())
                .to_local()
                .strftime(CLOCK_FORMAT)
                .unwrap()
        );
        sleep(DELAY);
    }
}
