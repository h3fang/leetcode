pub struct Solution;

impl Solution {
    pub fn minimum_perimeter(needed_apples: i64) -> i64 {
        let (mut left, mut right) = (1, ((needed_apples / 4) as f64).powf(1.0 / 3.0) as i64);
        while left <= right {
            let d = (right - left) / 2 + left;
            let a = (1 + d) * d * (2 * d + 1) * 2;
            if a >= needed_apples {
                right = d - 1;
            } else {
                left = d + 1;
            }
        }
        8 * left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(8, Solution::minimum_perimeter(1));
    }

    #[test]
    fn case2() {
        assert_eq!(16, Solution::minimum_perimeter(13));
    }

    #[test]
    fn case3() {
        assert_eq!(5040, Solution::minimum_perimeter(1000000000));
    }
}
