pub struct Solution;

impl Solution {
    pub fn strong_password_checker(password: String) -> i32 {
        let n = password.len();
        let (mut lower, mut upper, mut digit) = (0, 0, 0);
        for ch in password.chars() {
            if ch.is_lowercase() {
                lower = 1;
            } else if ch.is_uppercase() {
                upper = 1;
            } else if ch.is_ascii_digit() {
                digit = 1;
            }
        }

        let categories = lower + upper + digit;

        if n < 6 {
            (6 - n).max(3 - categories) as i32
        } else if n <= 20 {
            let (mut replace, mut count) = (0, 0);
            let mut current = '#';

            for ch in password.chars() {
                if ch == current {
                    count += 1;
                } else {
                    replace += count / 3;
                    count = 1;
                    current = ch;
                }
            }
            replace += count / 3;

            replace.max(3 - categories as i32)
        } else {
            let (mut replace, mut remove) = (0, n - 20);
            let (mut rm2, mut count) = (0, 0);
            let mut current = '#';

            for ch in password.chars() {
                if ch == current {
                    count += 1;
                } else {
                    if remove > 0 && count >= 3 {
                        if count % 3 == 0 {
                            remove -= 1;
                            replace -= 1;
                        } else if count % 3 == 1 {
                            rm2 += 1;
                        }
                    }

                    replace += count / 3;
                    count = 1;
                    current = ch;
                }
            }

            if remove > 0 && count >= 3 {
                if count % 3 == 0 {
                    remove -= 1;
                    replace -= 1;
                } else if count % 3 == 1 {
                    rm2 += 1
                }
            }

            replace += count / 3;

            let del2 = replace.min(rm2).min(remove / 2);
            replace -= del2;
            remove -= del2 * 2;

            let del3 = replace.min(remove / 3);
            replace -= del3;

            (n as i32 - 20) + replace.max(3 - categories) as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::strong_password_checker("a".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::strong_password_checker("aA1".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::strong_password_checker("1337C0d3".to_string()));
    }

    #[test]
    fn case4() {
        assert_eq!(1, Solution::strong_password_checker("aaaB1".to_string()));
    }
}
