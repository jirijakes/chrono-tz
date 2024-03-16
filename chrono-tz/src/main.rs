use chrono::*;
use chrono_tz::*;

fn main() {
    let tz: Tz = Tz::Europe__Prague;
    // let tz: Tz = Tz::Asia__Taipei;

    // let x = tz.previous_offset_change_utc(
    //     &NaiveDateTime::parse_from_str("2024-03-15 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
    // );
    // println!("1. {x:?}");

    let date = Utc.from_utc_datetime(
        &NaiveDateTime::parse_from_str("2024-04-15 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
    );
    // println!("2. {x:?}");

    let date: DateTime<Tz> = tz
        .from_local_datetime(
            &NaiveDateTime::parse_from_str("2024-04-16 05:30:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        )
        .single()
        .unwrap();

    // println!("{:?}", date.offset());

    let x: Option<_> = tz.previous_offset_change(&date);

    // let x = tz.offset_from_local_datetime(
    //     &NaiveDateTime::parse_from_str("2023-10-29 02:30:00", "%Y-%m-%d %H:%M:%S").unwrap(),
    // );

    println!("3. {x:?}");

    // let x = x.unwrap();
    // println!("{:?}", o.dst_offset());
}
