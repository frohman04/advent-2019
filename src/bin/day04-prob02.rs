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
    let mut in_group = false;
    let mut group_size = 1;
    for i in 1..digits.len() {
        let digit = digits[i];

        if last > digit {
            return false;
        }
        if digit == last {
            in_group = true;
            group_size += 1;
        } else {
            if in_group {
                if group_size == 2 {
                    consecutive_same = true;
                }
            }
            in_group = false;
            group_size = 1;
        }

        last = digit;
    }

    consecutive_same || (in_group && group_size == 2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_validate1() {
        assert_eq!(validate(111111), false);
    }

    #[test]
    fn test_validate2() {
        assert_eq!(validate(223450), false);
    }

    #[test]
    fn test_validate3() {
        assert_eq!(validate(123789), false);
    }

    #[test]
    fn test_validate4() {
        assert_eq!(validate(112233), true);
    }

    #[test]
    fn test_validate5() {
        assert_eq!(validate(123444), false);
    }

    #[test]
    fn test_validate6() {
        assert_eq!(validate(111122), true);
    }
}
