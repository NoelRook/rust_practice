use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let base: u64 = 10 ;
    let giga =  base.pow(9); // this is to get the giga prefix
    let giga_sec = std::time::Duration::from_secs(giga); // change giga to seconds
    let future_time = start + giga_sec;
    return future_time
        
}
