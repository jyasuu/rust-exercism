pub fn nth(n: u32) -> u32 {
    let mut count = 0;
    let mut num = 2;

    while count <= n {
        if is_prime(num) {
            count += 1;
        }
        if count > n {
            break;
        }
        num += 1;
    }
    num
}

fn is_prime(num: u32) -> bool {
    if num < 2 {
        return false;
    }
    for i in 2..=(num as f64).sqrt() as u32 {
        if num % i == 0 {
            return false;
        }
    }
    true
}

fn main() {}
#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn first_prime() {
        let output = nth(0);
        let expected = 2;
        assert_eq!(output, expected);
    }
    #[test]
    // #[ignore]
    fn second_prime() {
        let output = nth(1);
        let expected = 3;
        assert_eq!(output, expected);
    }
    #[test]
    // #[ignore]
    fn sixth_prime() {
        let output = nth(5);
        let expected = 13;
        assert_eq!(output, expected);
    }
    #[test]
    // #[ignore]
    fn big_prime() {
        let output = nth(10_000);
        let expected = 104_743;
        assert_eq!(output, expected);
    }
}
