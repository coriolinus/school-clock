use school_clock::SchoolClock;
use std::thread::sleep;
use time::Duration;

const CLOCK_FORMAT: &str = "%H:%M:%S";
const DELAY: std::time::Duration = std::time::Duration::from_millis(100);

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
        Duration::seconds(59),
        now,
    )
    .expect("must have successfully created clock");

    loop {
        print!(
            "\r{}",
            clock
                .at(time::now())
                .to_local()
                .strftime(CLOCK_FORMAT)
                .unwrap()
        );
        sleep(DELAY);
    }
}
