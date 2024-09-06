pub struct Solution;

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        if num < 4 {
            return num == 1;
        }
        let num = num as u64;
        let mut left = 1;
        let mut right = num / 2;
        while left <= right {
            let mid = left + (right - left) / 2;
            match (mid * mid).cmp(&(num)) {
                std::cmp::Ordering::Less => left = mid + 1,
                std::cmp::Ordering::Equal => return true,
                std::cmp::Ordering::Greater => right = mid - 1,
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::is_perfect_square(16));
    }

    #[test]
    fn case2() {
        assert!(Solution::is_perfect_square(1));
    }

    #[test]
    fn case3() {
        assert!(!Solution::is_perfect_square(i32::MAX));
    }
}
