fn main() {
    let min = 264360;
    let max = 746325;

    let num_good = (min..=max)
        .map(validate)
        .filter(|is_good| is_good.clone())
        .count();
    println!("{}", num_good);
}

fn validate(passwd: i32) -> bool {
    if passwd < 100000 || passwd >= 1000000 {
        return false;
    }

    let mut remaining = passwd.clone();
    let mut digits: Vec<i32> = Vec::new();
    while remaining > 0 {
        digits.push(remaining % 10);
        remaining /= 10;
    }
    digits.reverse();

    let mut last = digits[0];
    let mut consecutive_same = false;
    for i in 1..digits.len() {
        let digit = digits[i];

        if digit < last {
            return false;
        }
        if digit == last {
            consecutive_same = true;
        }

        last = digit;
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
