pub fn digit_count_i64(n: i64) -> u32 {
    if n == 0 {
        1
    } else {
        n.ilog10() + 1
    }
}

pub fn digit_count_u64(n: u64) -> u32 {
    if n == 0 {
        1
    } else {
        n.ilog10() + 1
    }
}
