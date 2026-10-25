use std::{ops::Deref, time::Duration};

use chrono::{DateTime, FixedOffset, Utc};

// AoC uses EST timezone for puzzle releases
const EST_SECS: i32 = -5 * 60 * 60;
pub const EST_TZ: FixedOffset = FixedOffset::east_opt(EST_SECS).unwrap();

pub trait Clock {
    fn now_etc(&self) -> DateTime<FixedOffset>;

    async fn sleep(&mut self, duration: Duration);
}

pub struct SystemClock;

impl Clock for SystemClock {
    fn now_etc(&self) -> DateTime<FixedOffset> {
        Utc::now().with_timezone(&EST_TZ)
    }

    async fn sleep(&mut self, duration: Duration) {
        tokio::time::sleep(duration).await;
    }
}

pub struct FakeClock(DateTime<FixedOffset>);

impl Deref for FakeClock {
    type Target = DateTime<FixedOffset>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FakeClock {
    pub fn new(time: DateTime<FixedOffset>) -> Self {
        Self(time)
    }

    pub fn set(&mut self, time: DateTime<FixedOffset>) {
        self.0 = time;
    }
}

#[cfg(test)]
impl Clock for FakeClock {
    fn now_etc(&self) -> DateTime<FixedOffset> {
        self.0
    }

    async fn sleep(&mut self, duration: Duration) {
        self.0 += duration;
    }
}
