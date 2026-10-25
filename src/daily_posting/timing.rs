use std::time::Duration;

use chrono::{DateTime, Datelike, FixedOffset, Timelike};

use crate::daily_posting::clock::Clock;

/// How many puzzles are in AoC
/// this used to be 24 but is now just 12
const NUM_PUZZLE_DAYS: u32 = 12;

pub fn is_puzzle_day(time: &DateTime<FixedOffset>) -> bool {
    time.month() == 12 && time.day() <= NUM_PUZZLE_DAYS
}

pub async fn wait_till_next_hour_approx<C: Clock>(clock: &mut C) {
    // Get current time in EST
    let time = clock.now_etc();

    // Wait until the hour
    let mins_until_hour = 60 - time.minute() as u64;
    log::info!("Waiting for {mins_until_hour} mins until next hour...");
    clock.sleep(Duration::from_secs(60 * mins_until_hour)).await;
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::daily_posting::clock::{fake_clock::FakeClock, EST_TZ};
    use chrono::TimeZone;

    macro_rules! date {
        ($y: expr, $m: expr, $d: expr, $hh: expr, $mm: expr, $ss: expr) => {
            EST_TZ.with_ymd_and_hms($y, $m, $d, $hh, $mm, $ss).unwrap()
        };
    }

    #[test]
    fn test_is_puzzle_day() {
        for i in 1..=12 {
            assert!(is_puzzle_day(&date!(2022, 12, i, 00, 00, 00)));
        }

        for i in 13..=20 {
            assert!(!is_puzzle_day(&date!(2022, 12, i, 00, 00, 00)));
        }

        for i in 1..=12 {
            assert!(!is_puzzle_day(&date!(2022, 11, i, 00, 00, 00)));
        }
    }

    #[tokio::test]
    async fn test_wait_till_next_hour() {
        let mut clock = FakeClock::new(date!(2022, 11, 30, 23, 00, 00));
        wait_till_next_hour_approx(&mut clock).await;
        assert_eq!(clock.day(), 1);
    }
}
