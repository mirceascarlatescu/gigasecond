use time::{PrimitiveDateTime as DateTime, ext::NumericalDuration};
use time_macros::{date, time};

fn main() {
    let my_date: DateTime = DateTime::new(date!(2015-01-24), time!(10:00));
    let after_my_date   = after(my_date);
    println!("My date {}, and after {}",my_date.to_string(), after_my_date.to_string());
}
fn after(start: DateTime) -> DateTime {
    start + 1000_000_000.seconds()
}