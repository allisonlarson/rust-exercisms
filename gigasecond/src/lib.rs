extern crate chrono;
use chrono::*;

pub fn after(date: DateTime<UTC>) -> DateTime<UTC> {
    let ten: i64 = 10;
    let giga = ten.pow(9);
    return date + Duration::seconds(giga)
}
