pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64")
    }

    let mut grain:u64 = 1;
    for _ in 2..=s {
        grain *= 2;
    }
    grain
}

pub fn total() -> u64 {
    let mut counter = 0;
    let mut sum = 1;
    loop{
        sum  *= 2;
        counter += 1;

        if counter == 64 {
            break sum
        }
    }
}
