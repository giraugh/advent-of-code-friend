use std::time::Duration;

use chrono::{DateTime, Datelike, FixedOffset, Timelike, Utc};

// AoC uses EST timezone for puzzle releases
const EST_SECS: i32 = -5 * 60 * 60;
pub const EST_TZ: FixedOffset = FixedOffset::east_opt(EST_SECS).unwrap();

/// How many puzzles are in AoC
/// this used to be 24 but is now just 12
const NUM_PUZZLE_DAYS: u32 = 12;

pub fn is_puzzle_day(time: &DateTime<FixedOffset>) -> bool {
    time.month() == 12 && time.day() <= NUM_PUZZLE_DAYS
}

pub fn now_etc() -> DateTime<FixedOffset> {
    Utc::now().with_timezone(&EST_TZ)
}

pub async fn wait_till_next_hour_approx() -> DateTime<FixedOffset> {
    // Get current time in EST
    let time = now_etc();

    // Wait until the hour
    let mins_until_hour = 60 - time.minute() as u64;
    log::info!("Waiting for {mins_until_hour} mins until next hour...");
    tokio::time::sleep(Duration::from_secs(60 * mins_until_hour)).await;

    // Then wait an arbitrary random amount of extra
    // under a minute ofc
    // this means the puzzle will be posted,
    // we prevent spam to AoC at start of day etc
    // NOTE: not exactly random
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Get the current time after waiting
    now_etc()
}
#[cfg(test)]
mod test {
    use super::*;
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

    // TODO: harder to test...
    #[test]
    fn test_wait_till_next_hour() {
        // TODO
    }
}
