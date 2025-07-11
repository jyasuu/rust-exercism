pub fn square(s: u32) -> u64 {
    if s == 0 || s > 64 {
        panic!("Input must be between 1 and 64");
    }
    2u64.pow(s - 1)
}

pub fn total() -> u64 {
    (1..=64).map(|s| 2u64.pow(s - 1)).sum()
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn grains_on_square_1() {
        assert_eq!(square(1), 1);
    }
    #[test]
    // #[ignore]
    fn grains_on_square_2() {
        assert_eq!(square(2), 2);
    }
    #[test]
    // #[ignore]
    fn grains_on_square_3() {
        assert_eq!(square(3), 4);
    }
    #[test]
    // #[ignore]
    fn grains_on_square_4() {
        assert_eq!(square(4), 8);
    }
    #[test]
    // #[ignore]
    fn grains_on_square_16() {
        assert_eq!(square(16), 32_768);
    }
    #[test]
    // #[ignore]
    fn grains_on_square_32() {
        assert_eq!(square(32), 2_147_483_648);
    }
    #[test]
    // #[ignore]
    fn grains_on_square_64() {
        assert_eq!(square(64), 9_223_372_036_854_775_808);
    }
    #[test]
    // #[ignore]
    #[should_panic]
    fn square_0_is_invalid() {
        square(0);
    }
    #[test]
    // #[ignore]
    #[should_panic]
    fn square_greater_than_64_is_invalid() {
        square(65);
    }
    #[test]
    // #[ignore]
    fn returns_the_total_number_of_grains_on_the_board() {
        assert_eq!(total(), 18_446_744_073_709_551_615);
    }
}
