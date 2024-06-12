use chrono::{DateTime, FixedOffset};
use mockall::automock;

#[automock]
pub trait DateProvidable {
    fn now(&self) -> DateTime<FixedOffset>;
}

#[derive(Default)]
pub struct DateProvider {}

impl DateProvidable for DateProvider {
    fn now(&self) -> DateTime<FixedOffset> {
        DateTime::<FixedOffset>::from_naive_utc_and_offset(
            chrono::Utc::now().naive_utc(),
            FixedOffset::east_opt(0).unwrap(),
        )
    }
}

#[automock]
pub trait DateFormattable {
    fn format(&self, date: &DateTime<FixedOffset>) -> String;
    #[allow(dead_code)]
    fn parse(&self, date_value: &str) -> DateTime<FixedOffset>;
}

// %a is the abbreviated weekday name
// %d is the day of the month
// %b is the abbreviated month name
// %Y is the year with century
// %T is the time in 24-hour notation (%H:%M:%S)
// %z is the time zone as an offset from UTC
const FORMAT: &str = "%a, %d %b %Y %T %z";

#[derive(Default)]
pub struct DateFormatter {}
impl DateFormattable for DateFormatter {
    fn format(&self, date: &DateTime<FixedOffset>) -> String {
        date.format(FORMAT).to_string()
    }

    fn parse(&self, date_value: &str) -> DateTime<FixedOffset> {
        DateTime::parse_from_str(date_value, FORMAT).unwrap()
    }
}

impl DateFormatter {}

#[cfg(test)]
pub mod tests {
    use chrono::{DateTime, NaiveDate};

    use super::*;

    #[test]
    fn convert_time() {
        // let mut mock = MockDateProvider::new();
        // mock.expect_now()
        //     .return_const(
        // Instant::try_from(1).unwrap());

        let date_str = "Tue, 26 Jun 2018 15:50:21 +0000".to_string();
        let x = DateFormatter::default().parse(&date_str);
        let millis = 1530028221000;

        assert_eq!(x.timestamp_millis(), millis);

        let expected_time = DateTime::<FixedOffset>::from_naive_utc_and_offset(
            NaiveDate::from_ymd_opt(2018, 6, 26)
                .unwrap()
                .and_hms_opt(15, 50, 21)
                .unwrap(),
            FixedOffset::east_opt(0).unwrap(),
        );
        assert_eq!(expected_time.timestamp_millis(), millis);

        let formatted = DateFormatter::default().format(&expected_time);
        assert_eq!(formatted, date_str);
    }

    #[test]
    fn convert_time_2() {
        let mut mock = MockDateProvidable::new();
        mock.expect_now()
            .return_const(DateFormatter::default().parse("Wed, 27 Jun 2018 15:50:21 +0000"));
    }
}
