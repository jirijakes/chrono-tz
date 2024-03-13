use chrono::*;
use chrono_tz::*;

fn main() {
    let tz: Tz = Tz::America__Indiana__Indianapolis;

    let x = tz.previous_offset_change_utc(
        &NaiveDateTime::parse_from_str("2024-03-15 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
    );
    println!("1. {x:?}");

    let x = tz.next_offset_change_utc(
        &NaiveDateTime::parse_from_str("2024-03-15 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
    );
    println!("2. {x:?}");

    let x: Option<(DateTime<Tz>, _)> = tz.next_offset_change(
        &tz.from_local_datetime(
            &NaiveDateTime::parse_from_str("2024-03-15 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        )
        .unwrap(),
    );

    println!("3. {x:?}");
}
