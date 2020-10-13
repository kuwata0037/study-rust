#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    #[test]
    fn measure_the_elapsed_time_between_two_code_section() {
        use std::time::{Duration, Instant};

        fn expensive_function() {
            std::thread::sleep(Duration::from_secs(1));
        }

        let start = Instant::now();
        expensive_function();
        let duration = start.elapsed();
        assert_eq!(duration.as_secs(), 1);
    }

    #[test]
    fn perform_checked_date_and_time_calculations() {
        use chrono::{DateTime, Duration, Utc};

        fn day_earlier(date_time: DateTime<Utc>) -> Option<DateTime<Utc>> {
            date_time.checked_sub_signed(Duration::days(1))
        }

        let now = Utc.ymd(2020, 10, 1).and_hms(0, 0, 0);

        let almost_three_weeks_from_now = now
            .checked_add_signed(Duration::weeks(2))
            .and_then(|in_2weeks| in_2weeks.checked_add_signed(Duration::weeks(1)))
            .and_then(day_earlier);
        assert_eq!(
            almost_three_weeks_from_now,
            Some(Utc.ymd(2020, 10, 21).and_hms(0, 0, 0))
        );

        assert_eq!(now.checked_add_signed(Duration::max_value()), None);
    }

    #[test]
    fn convert_a_local_time_to_another_timezone() {
        use chrono::{DateTime, FixedOffset, Local, Utc};

        let local_time = Local.ymd(2020, 10, 01).and_hms(0, 0, 0);
        let utc_time = DateTime::<Utc>::from_utc(local_time.naive_utc(), Utc);
        let china_timezone = FixedOffset::east(8 * 3600);
        let rio_timezone = FixedOffset::west(2 * 3600);
        println!("Local time now is {}", local_time);
        println!("UTF time now is {}", utc_time);
        println!(
            "Time in Hong Kong now is {}",
            utc_time.with_timezone(&china_timezone)
        );
        println!(
            "Time in Rio de Janeiro now is {}",
            utc_time.with_timezone(&rio_timezone)
        );
    }
}
