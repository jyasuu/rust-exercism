pub fn is_armstrong_number(num: u32) -> bool {
    let num_str = num.to_string();
    let num_digits = num_str.len() as u32;
    let mut sum_of_powers = 0;

    for digit_char in num_str.chars() {
        if let Some(digit) = digit_char.to_digit(10) {
            sum_of_powers += digit.pow(num_digits);
        }
    }

    sum_of_powers == num
}

#[cfg(test)]
mod test {
    use crate::is_armstrong_number;

    #[test]
    fn zero_is_an_armstrong_number() {
        assert!(is_armstrong_number(0))
    }
    #[test]
    // #[ignore]
    fn single_digit_numbers_are_armstrong_numbers() {
        assert!(is_armstrong_number(5))
    }
    #[test]
    // #[ignore]
    fn there_are_no_two_digit_armstrong_numbers() {
        assert!(!is_armstrong_number(10))
    }
    #[test]
    // #[ignore]
    fn three_digit_number_that_is_an_armstrong_number() {
        assert!(is_armstrong_number(153))
    }
    #[test]
    // #[ignore]
    fn three_digit_number_that_is_not_an_armstrong_number() {
        assert!(!is_armstrong_number(100))
    }
    #[test]
    // #[ignore]
    fn four_digit_number_that_is_an_armstrong_number() {
        assert!(is_armstrong_number(9_474))
    }
    #[test]
    // #[ignore]
    fn four_digit_number_that_is_not_an_armstrong_number() {
        assert!(!is_armstrong_number(9_475))
    }
    #[test]
    // #[ignore]
    fn seven_digit_number_that_is_an_armstrong_number() {
        assert!(is_armstrong_number(9_926_315))
    }
    #[test]
    // #[ignore]
    fn seven_digit_number_that_is_not_an_armstrong_number() {
        assert!(!is_armstrong_number(9_926_314))
    }
}
