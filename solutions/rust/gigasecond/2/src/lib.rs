use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    const GIGA_SEC: u64 = 1_000_000_000;
    let giga_sec = std::time::Duration::from_secs(GIGA_SEC); // change giga to seconds
    let future_time = start + giga_sec;
    return future_time
        
}
