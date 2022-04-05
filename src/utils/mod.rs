use chrono::prelude::*;
use chrono::Duration;

pub mod quantity;

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


pub fn voucher_datetime() -> DateTime<FixedOffset> {
    let (flag_dt, flag_tt) = flag_tt();
    if current_tt() > flag_tt {
        flag_dt + Duration::days(1)
    } else {
        flag_dt
    }
}

pub fn code_prefix() -> String {
    let bill_date = voucher_datetime().date().format("SA.%Y.%m.%d").to_string();
    bill_date + ".%"
}

pub fn voucherdate() -> String {
    voucher_datetime().date().format("%Y-%m-%d").to_string()
}
