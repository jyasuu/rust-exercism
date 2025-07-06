pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let mut count = 0;
    let mut current_n = n;

    while current_n != 1 {
        if current_n % 2 == 0 {
            current_n /= 2;
        } else {
            current_n = 3 * current_n + 1;
        }
        count += 1;
    }

    Some(count)
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn zero_steps_for_one() {
        let output = collatz(1);
        let expected = Some(0);
        assert_eq!(output, expected);
    }
    #[test]
    // #[ignore]
    fn divide_if_even() {
        let output = collatz(16);
        let expected = Some(4);
        assert_eq!(output, expected);
    }
    #[test]
    // #[ignore]
    fn even_and_odd_steps() {
        let output = collatz(12);
        let expected = Some(9);
        assert_eq!(output, expected);
    }
    #[test]
    // #[ignore]
    fn large_number_of_even_and_odd_steps() {
        let output = collatz(1_000_000);
        let expected = Some(152);
        assert_eq!(output, expected);
    }
    #[test]
    // #[ignore]
    fn zero_is_an_error() {
        let output = collatz(0);
        let expected = None;
        assert_eq!(output, expected);
    }
}
