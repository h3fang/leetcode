pub struct Solution;

impl Solution {
    pub fn preimage_size_fzf(k: i32) -> i32 {
        fn zeros(mut n: i32) -> i32 {
            let mut result = 0;
            while n > 0 {
                n /= 5;
                result += n;
            }
            result
        }

        fn f(k: i32) -> i32 {
            let mut left = 0;
            let mut right = 5 * k;
            while left <= right {
                let mid = left + (right - left) / 2;
                if zeros(mid) < k {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
            right + 1
        }

        f(k + 1) - f(k)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::preimage_size_fzf(0));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::preimage_size_fzf(5));
    }

    #[test]
    fn case3() {
        assert_eq!(5, Solution::preimage_size_fzf(3));
    }
}
