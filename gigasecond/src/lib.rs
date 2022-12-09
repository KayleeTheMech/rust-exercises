use std::ops::Add;

use time::Duration;
use time::PrimitiveDateTime as DateTime;
// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let gigasecond = Duration::seconds(i64::pow(10, 9));
    start.add(gigasecond)
}
