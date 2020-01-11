fn main() {
    let min = 264_360;
    let max = 746_325;

    let num_good = (min..=max).map(validate).count();
    println!("{}", num_good);
}

fn validate(passwd: i32) -> bool {
    if passwd < 100_000 || passwd >= 1_000_000 {
        return false;
    }

    let mut remaining = passwd;
    let mut digits: Vec<i32> = Vec::new();
    while remaining > 0 {
        digits.push(remaining % 10);
        remaining /= 10;
    }
    digits.reverse();

    let mut last = digits[0];
    let mut consecutive_same = false;
    for digit in digits.iter().skip(1) {
        if *digit < last {
            return false;
        }
        if *digit == last {
            consecutive_same = true;
        }

        last = *digit;
    }

    consecutive_same
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_validate1() {
        assert_eq!(validate(111111), true);
    }

    #[test]
    fn test_validate2() {
        assert_eq!(validate(223450), false);
    }

    #[test]
    fn test_validate3() {
        assert_eq!(validate(123789), false);
    }
}
