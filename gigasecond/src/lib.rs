extern crate chrono;
use chrono::*;

pub fn after(date: DateTime<UTC>) -> DateTime<UTC> {
    let ten: i64 = 10;
    date + Duration::seconds(ten.pow(9))
}
