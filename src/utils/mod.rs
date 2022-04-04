use chrono::prelude::*;
use chrono::Duration;

pub fn code_prefix() -> String {
    let today_last_datetime = Local::now().date().to_string() + " 17:00:00";
    let flag_dt =
        DateTime::parse_from_str(today_last_datetime.as_str(), "%Y-%m-%d%z %H:%M:%S").unwrap();
    let flag_tt = flag_dt.timestamp();

    let current = Local::now();

    let current_tt = current.timestamp();

    let bill_datetime = if current_tt > flag_tt {
        flag_dt + Duration::days(1)
    } else {
        flag_dt
    };
    let bill_date = bill_datetime.date().format("SA.%Y.%m.%d").to_string();
    bill_date + ".%"
}

fn flag_tt() -> (DateTime<FixedOffset>, i64) {
    let today_last_datetime = Local::now().date().to_string() + " 17:00:00";
    let flag_dt =
        DateTime::parse_from_str(today_last_datetime.as_str(), "%Y-%m-%d%z %H:%M:%S").unwrap();
    (flag_dt, flag_dt.timestamp())
}

fn current_tt() -> i64 {
    let current = Local::now();
    current.timestamp()
}

pub fn diff() -> i32 {
    let (_, flag_tt) = flag_tt();
    if current_tt() > flag_tt {
        -1
    } else {
        0
    }
}

pub fn code_prefix2() -> String {
    let (flag_dt, flag_tt) = flag_tt();
    let bill_datetime = if current_tt() > flag_tt {
        flag_dt + Duration::days(1)
    } else {
        flag_dt
    };
    let bill_date = bill_datetime.date().format("SA.%Y.%m.%d").to_string();
    bill_date + ".%"
}
