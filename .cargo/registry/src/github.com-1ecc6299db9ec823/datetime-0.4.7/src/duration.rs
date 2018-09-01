//! Lengths of time on the timeline.

use std::ops::{Add, Sub, Mul};


/// A **duration** is a length of time on the timeline, irrespective of
/// time zone or calendar format, with millisecond precision.
#[derive(Clone, PartialEq, Eq, Debug, Copy)]
pub struct Duration {
    seconds: i64,
    milliseconds: i16,
}

impl Duration {

    /// Create a new zero-length duration.
    pub fn zero() -> Duration {
        Duration { seconds: 0, milliseconds: 0 }
    }

    /// Create a new duration that’s the given number of seconds long.
    pub fn of(seconds: i64) -> Duration {
        Duration { seconds: seconds, milliseconds: 0 }
    }

    /// Create a new duration that’s the given number of seconds and
    /// milliseconds long.
    pub fn of_ms(seconds: i64, milliseconds: i16) -> Duration {
        assert!(milliseconds >= 0 && milliseconds <= 999);  // TODO: replace assert with returning Result
        Duration { seconds: seconds, milliseconds: milliseconds }
    }

    /// Return the seconds and milliseconds portions of the duration as
    /// a 2-element tuple.
    pub fn lengths(&self) -> (i64, i16) {
        (self.seconds, self.milliseconds)
    }

    // I’ve done it like this instead of having separate seconds() and
    // milliseconds() functions, because I think there’s a danger that
    // people will think that milliseconds() returns the *total* length
    // in milliseconds, rather than just this particular portion. This
    // way, it’s clear that there are two separate values being returned.
}

impl Add<Duration> for Duration {
    type Output = Duration;

    fn add(self, rhs: Duration) -> Duration {
        let ms = self.milliseconds + rhs.milliseconds;
        if ms >= 1000 {
            Duration::of_ms(self.seconds + rhs.seconds + 1, ms - 1000)
        }
        else {
            Duration::of_ms(self.seconds + rhs.seconds, ms)
        }
    }
}

impl Sub<Duration> for Duration {
    type Output = Duration;

    fn sub(self, rhs: Duration) -> Duration {
        let ms = self.milliseconds - rhs.milliseconds;
        if ms < 0 {
            Duration::of_ms(self.seconds - rhs.seconds - 1, ms + 1000)
        }
        else {
            Duration::of_ms(self.seconds - rhs.seconds, ms)
        }
    }
}

impl Mul<i64> for Duration {
    type Output = Duration;

    fn mul(self, amount: i64) -> Duration {
        let ms = self.milliseconds as i64 * amount;
        Duration::of_ms(self.seconds * amount + ms / 1000, (ms % 1000) as i16)
    }
}
