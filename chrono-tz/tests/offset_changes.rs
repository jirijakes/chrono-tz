use chrono::{NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Timelike, Utc};
use chrono_tz::Tz;

#[test]
fn no_next_utc_change() {
    // Buenos Aires ended DST in 2009
    let d = NaiveDateTime::new(NaiveDate::from_ymd_opt(2010, 1, 1).unwrap(), NaiveTime::default());
    let next = Tz::America__Buenos_Aires.next_offset_change_utc(&d);
    assert!(next.is_none())
}

// #[test]
// fn utc_change() {
//     let d = NaiveDateTime::new(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(), NaiveTime::default());
//     let next = Tz::America__Buenos_Aires.next_offset_change_utc(&d);
//     assert!(next.is_none())
// }

// #[test]
// fn next_utc_change() {
//     let d = NaiveDateTime::new(NaiveDate::from_ymd_opt(200, 1, 1).unwrap(), NaiveTime::default());
//     let next = Tz::America__Buenos_Aires.next_offset_change_utc(&d);
//     assert!(next.is_none())
// }

#[test]
fn no_previous_utc_change() {
    let d = NaiveDateTime::new(NaiveDate::from_ymd_opt(1700, 1, 1).unwrap(), NaiveTime::default());
    let next = Tz::America__Buenos_Aires.next_offset_change_utc(&d);
    assert!(next.is_none())
}
