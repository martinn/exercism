use std::time::Duration;

use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let one_billion_seconds = Duration::new(1000000000, 0);
    let after = start + one_billion_seconds;

    return after;
}
