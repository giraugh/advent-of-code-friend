use std::time::Duration;

use chrono::{DateTime, FixedOffset, Utc};

// AoC uses EST timezone for puzzle releases
const EST_SECS: i32 = -5 * 60 * 60;
pub const EST_TZ: FixedOffset = FixedOffset::east_opt(EST_SECS).unwrap();

/// Trait that abstracts getting current time and waiting
/// this way we can test our wait methods in unit tests
/// for normal behaviour, just use `SystemClock` which will call
/// tokio and chrono time methods
pub trait Clock {
    fn now_etc(&self) -> DateTime<FixedOffset>;

    async fn sleep(&mut self, duration: Duration);
}

/// Normal clock, uses system time
pub struct SystemClock;

impl Clock for SystemClock {
    fn now_etc(&self) -> DateTime<FixedOffset> {
        Utc::now().with_timezone(&EST_TZ)
    }

    async fn sleep(&mut self, duration: Duration) {
        tokio::time::sleep(duration).await;
    }
}

#[cfg(test)]
pub mod fake_clock {
    use super::*;
    use chrono::{DateTime, FixedOffset};

    /// Faked clock, can manually set and get the time
    pub struct FakeClock(DateTime<FixedOffset>);

    impl std::ops::Deref for FakeClock {
        type Target = DateTime<FixedOffset>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl FakeClock {
        pub fn new(time: DateTime<FixedOffset>) -> Self {
            Self(time)
        }
    }

    impl Clock for FakeClock {
        fn now_etc(&self) -> DateTime<FixedOffset> {
            self.0
        }

        async fn sleep(&mut self, duration: Duration) {
            self.0 += duration;
        }
    }
}
