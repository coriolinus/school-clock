use time::{Duration, Tm};

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct SchoolClock {
    beat: Duration,
    inflection: Duration,
    fade: Duration,
    initialize: Tm,
}

impl SchoolClock {
    pub fn new(
        beat: Duration,
        inflection: Duration,
        fade: Duration,
        initialize: Tm,
    ) -> Option<SchoolClock> {
        if inflection <= fade && fade <= beat {
            Some(SchoolClock {
                beat,
                inflection,
                fade,
                initialize,
            })
        } else {
            None
        }
    }

    /// calculate the reported time given the real time
    pub fn at(&self, real_time: Tm) -> Tm {
        // first, find the start of the current cycle
        let mut cycle = self.initialize;
        while cycle + self.beat < real_time {
            cycle = cycle + self.beat;
        }

        let inflexion = cycle + self.inflection;
        (if real_time < inflexion {
            // run fast
            let real = real_time - cycle;
            cycle
                + Duration::milliseconds(
                    (real.num_milliseconds() * self.fade.num_milliseconds())
                        / self.inflection.num_milliseconds(),
                )
        } else {
            // run slow
            let real = real_time - inflexion;
            cycle
                + self.fade
                + Duration::milliseconds(
                    (real.num_milliseconds())
                        * (self.beat.num_milliseconds() - self.fade.num_milliseconds())
                        / (self.beat.num_milliseconds() - self.inflection.num_milliseconds()),
                )
        })
        .to_utc()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn today_at(hour: i32, minute: i32, sec: i32) -> Tm {
        let mut t = time::now_utc();
        t.tm_hour = hour;
        t.tm_min = minute;
        t.tm_sec = sec;
        t.tm_nsec = 0;
        t
    }

    #[test]
    fn readme_example() {
        let clock = SchoolClock::new(
            Duration::hours(1),
            Duration::minutes(54),
            Duration::minutes(59),
            today_at(8, 30, 0),
        )
        .expect("must successfully create clock");

        // 08:30:00 -> 08:30:00
        // 08:57:00 -> 08:59:30
        // 09:24:00 -> 09:29:00
        // 09:27:00 -> 09:29:30
        // 09:30:00 -> 09:30:00
        let cases = &[
            ((8, 30, 0), (8, 30, 0)),
            ((8, 57, 0), (8, 59, 30)),
            ((9, 24, 0), (9, 29, 0)),
            ((9, 27, 0), (9, 29, 30)),
            ((9, 30, 0), (9, 30, 0)),
        ];
        for ((rh, rm, rs), (ch, cm, cs)) in cases {
            let real = today_at(*rh, *rm, *rs);
            let expect = today_at(*ch, *cm, *cs);
            let calc = clock.at(real);

            println!(
                "at({}) -> {} (expect {})",
                real.rfc3339(),
                calc.rfc3339(),
                expect.rfc3339()
            );
            assert_eq!(calc, expect);
        }
    }
}
