pub struct Solution;

impl Solution {
    pub fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
        fn valid(mid: i32, m: i32, n: i32, k: i32) -> bool {
            let mut r = 0;
            for i in 1..=m {
                r += n.min(mid / i);
                if r >= k {
                    return true;
                }
            }
            false
        }
        let mut left = 1;
        let mut right = m * n;
        let mut result = right;
        while left <= right {
            let mid = left + (right - left) / 2;
            if valid(mid, m, n, k) {
                result = mid;
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::find_kth_number(3, 3, 5));
    }

    #[test]
    fn case2() {
        assert_eq!(6, Solution::find_kth_number(2, 3, 6));
    }
}
