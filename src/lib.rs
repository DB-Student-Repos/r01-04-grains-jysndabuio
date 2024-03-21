pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64")
    }
    //  no need to assign variable just use directly required type
    // and use :: to access the associated function
    u64::pow(2, s-1) // Solved by geometric progression.
}

pub fn total() -> u64 {
    // u64::pow(2, 64) -1 overflow
    // ((1u128 << 64) -1) as u64
    const TOTAL:u64 = 18446744073709551615;
    TOTAL
}
