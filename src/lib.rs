pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64")
    }
    //  no need to assign variable just use directly required type
    // and use :: to access the associated function
    u64::pow(2, s-1) // Solved by geometric progression.
}

pub fn total() -> u64 {
    (1..=64).map(square).sum()
}
