pub fn is_leap_year(year: u64) -> bool {
    if year % 100 == 0 {
        return if year % 400 == 0 { true } else { false };
    }
    return if year % 4 == 0 { true } else { false }
}
