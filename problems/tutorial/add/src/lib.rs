#![forbid(unsafe_code)]

pub fn add(x: i32, y: i32) -> i32 {
    // Wrapping (modular) addition. Computes self + rhs, wrapping around at the boundary of the type.
    // return x.saturating_add(y) but it fails saturating_add() testcase

    // Saturating integer addition. Computes self + rhs, saturating at the numeric bounds instead of overflowing.
    return x.saturating_add(y)
}
