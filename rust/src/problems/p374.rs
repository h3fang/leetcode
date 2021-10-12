#![allow(dead_code)]

pub struct Solution;

static mut N: i32 = 10;
static mut PICKED: i32 = 6;

unsafe fn guess(n: i32) -> i32 {
    match PICKED.cmp(&n) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}

impl Solution {
    #[allow(non_snake_case)]
    unsafe fn guessNumber(n: i32) -> i32 {
        let mut left = 1;
        let mut right = n;
        while left <= right {
            let mid = left + (right - left) / 2;
            let r = guess(mid);
            if r == 0 {
                return mid;
            } else if r == 1 {
                left = mid + 1;
            } else if r == -1 {
                right = mid - 1;
            }
        }
        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        unsafe {
            N = 10;
            PICKED = 6;
            assert_eq!(PICKED, Solution::guessNumber(N));
        }
    }

    #[test]
    fn case2() {
        unsafe {
            N = 1;
            PICKED = 1;
            assert_eq!(PICKED, Solution::guessNumber(N));
        }
    }
}
