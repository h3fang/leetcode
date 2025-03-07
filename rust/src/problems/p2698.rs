pub struct Solution;

fn bt(digits: &[i32], target: i32) -> bool {
    if digits.is_empty() {
        target == 0
    } else {
        let mut x = 0;
        for (j, &d) in digits.iter().enumerate() {
            x = x * 10 + d;
            if x > target {
                return false;
            }
            if bt(&digits[j + 1..], target - x) {
                return true;
            }
        }
        false
    }
}

impl Solution {
    pub fn punishment_number(n: i32) -> i32 {
        (1..=n)
            .map(|i| {
                let i2 = i * i;
                let mut digits = vec![];
                let mut x = i2;
                while x > 0 {
                    digits.push(x % 10);
                    x /= 10;
                }
                digits.reverse();
                if bt(&digits, i) {
                    i2
                } else {
                    0
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(182, Solution::punishment_number(10));
    }

    #[test]
    fn case2() {
        assert_eq!(1478, Solution::punishment_number(37));
    }
}
