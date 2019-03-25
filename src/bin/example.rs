use school_clock::SchoolClock;
use std::thread::sleep;
use time::Duration;

const CLOCK_FORMAT: &str = "%H:%M:%S";
const DELAY: std::time::Duration = std::time::Duration::from_millis(200);

fn main() {
    let clock = SchoolClock::new(
        Duration::hours(1),
        Duration::minutes(54),
        Duration::minutes(58),
        time::Tm {
            tm_hour: 8,
            tm_min: 30,
            ..time::now()
        },
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
