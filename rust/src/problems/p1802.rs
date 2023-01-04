pub struct Solution;

impl Solution {
    pub fn max_value(n: i32, index: i32, max_sum: i32) -> i32 {
        fn count(max: i64, size: i64) -> i64 {
            if size + 1 < max {
                let min = max - size;
                (max - 1 + min) * size / 2
            } else {
                let ones = size - (max - 1);
                (max - 1) * max / 2 + ones
            }
        }
        let sl = index as i64;
        let sr = n as i64 - 1 - index as i64;
        let max_sum = max_sum as i64;
        let mut left = 1;
        let mut right = max_sum;
        while left < right {
            let mid = left + (right - left + 1) / 2;
            let sum = mid + count(mid, sl) + count(mid, sr);
            if sum <= max_sum {
                left = mid;
            } else {
                right = mid - 1;
            }
        }
        left as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::max_value(4, 2, 6));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::max_value(6, 1, 10));
    }
}
