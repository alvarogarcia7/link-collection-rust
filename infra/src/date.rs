use chrono::{DateTime, FixedOffset, NaiveDateTime, ParseError};
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
    fn try_parse(&self, date_value: &str) -> Result<DateTime<FixedOffset>, Vec<ParseError>>;
}

// https://docs.rs/chrono/latest/chrono/format/strftime/index.html
// %a is the abbreviated weekday name
// %d is the day of the month
// %b is the abbreviated month name
// %Y is the year with century
// %T is the time in 24-hour notation (%H:%M:%S)
// %z is the time zone as an offset from UTC
const INPUT_FORMATS: [&str; 3] = [
    // "Mon, 08 Feb 2021 22:05:16 +0000"
    "%a, %d %b %Y %H:%M:%S %z",
    // "Tue 26 Jun 2018 16:00:53 +0000"
    "%a %d %b %Y %H:%M:%S",
    // "Wed Jan 18 00:48:46 2023"
    "%a %b %d %T %Y",
];
const OUTPUT_FORMAT: &str = "%a, %d %b %Y %T %z";

#[derive(Default)]
pub struct DateFormatter {}
impl DateFormattable for DateFormatter {
    fn format(&self, date: &DateTime<FixedOffset>) -> String {
        date.format(OUTPUT_FORMAT).to_string()
    }

    fn try_parse(&self, date_value: &str) -> Result<DateTime<FixedOffset>, Vec<ParseError>> {
        let mut errors = Vec::with_capacity(INPUT_FORMATS.len());
        for format in INPUT_FORMATS.iter() {
            if format.contains("%z") {
                match DateTime::parse_from_str(date_value, format) {
                    Ok(date) => return Ok(date),
                    Err(e) => errors.push(e),
                }
            } else {
                match NaiveDateTime::parse_from_str(date_value, format) {
                    Ok(date) => {
                        return Ok(DateTime::from_naive_utc_and_offset(
                            date,
                            FixedOffset::east_opt(0).unwrap(),
                        ))
                    }
                    Err(e) => errors.push(e),
                }
            }
        }
        Err(errors)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    pub fn parse_format1() {
        let format = "Wed Jan 18 00:48:46 2023".to_string();

        let actual = DateFormatter::default().try_parse(&format);

        assert!(actual.is_ok());

        let expected_time = DateTime::<FixedOffset>::from_naive_utc_and_offset(
            NaiveDate::from_ymd_opt(2023, 1, 18)
                .unwrap()
                .and_hms_opt(0, 48, 46)
                .unwrap(),
            FixedOffset::east_opt(0).unwrap(),
        );
        assert_eq!(
            expected_time.timestamp_millis(),
            actual.unwrap().timestamp_millis()
        );
    }
    #[test]
    pub fn parse_format2() {
        let format = "Mon 08 Feb 2021 22:05:16".to_string();

        let actual = DateFormatter::default().try_parse(&format);

        assert!(actual.is_ok());

        let expected_time = DateTime::<FixedOffset>::from_naive_utc_and_offset(
            NaiveDate::from_ymd_opt(2021, 2, 8)
                .unwrap()
                .and_hms_opt(22, 5, 16)
                .unwrap(),
            FixedOffset::east_opt(0).unwrap(),
        );
        assert_eq!(
            expected_time.timestamp_millis(),
            actual.unwrap().timestamp_millis()
        );
    }

    #[test]
    pub fn parse_format3() {
        let format = "Mon, 13 Mar 2023 12:07:17 +0000".to_string();

        let actual = DateFormatter::default().try_parse(&format);

        assert!(actual.is_ok());

        let expected_time = DateTime::<FixedOffset>::from_naive_utc_and_offset(
            NaiveDate::from_ymd_opt(2023, 3, 13)
                .unwrap()
                .and_hms_opt(12, 7, 17)
                .unwrap(),
            FixedOffset::east_opt(0).unwrap(),
        );
        assert_eq!(
            expected_time.timestamp_millis(),
            actual.unwrap().timestamp_millis()
        );
    }

    #[test]
    fn convert_time_2() {
        let mut mock = MockDateProvidable::new();
        mock.expect_now().return_const(
            DateFormatter::default()
                .try_parse("Wed, 27 Jun 2018 15:50:21 +0000")
                .unwrap(),
        );
    }
}
