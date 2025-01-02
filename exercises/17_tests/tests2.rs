// Calculates the power of 2 using a bit shift.
// `1 << n` is equivalent to "2 to the power of n".
fn power_of_2(n: u8) -> u64 {
    1 << n
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert_eq() {
        // DONE: Test the function `power_of_2` with some values.
        assert_eq!(power_of_2(0), 1);
        assert_eq!(power_of_2(5), 32);
        assert_eq!(power_of_2(12), 4096);
        assert_eq!(power_of_2(20), 1048576);
    }
}
