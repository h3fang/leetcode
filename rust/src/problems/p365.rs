pub struct Solution;

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

impl Solution {
    pub fn can_measure_water(a: i32, b: i32, t: i32) -> bool {
        if a + b < t {
            false
        } else if a == 0 || b == 0 {
            t == 0 || a + b == t
        } else {
            t % gcd(a, b) == 0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::can_measure_water(3, 5, 4));
    }

    #[test]
    fn case2() {
        assert!(!Solution::can_measure_water(2, 6, 5));
    }

    #[test]
    fn case3() {
        assert!(Solution::can_measure_water(1, 2, 3));
    }
}
