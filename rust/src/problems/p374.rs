#![allow(dead_code)]
// unsafe is required by the problem definition

use std::sync::Mutex;

pub struct Solution;

static LOCK: Mutex<()> = Mutex::new(());
#[allow(non_upper_case_globals)]
static mut guess: &dyn Fn(i32) -> i32 = &|_x| 0;

impl Solution {
    #[allow(non_snake_case)]
    unsafe fn guessNumber(n: i32) -> i32 {
        let mut left = 1;
        let mut right = n;
        while left <= right {
            let mid = left + (right - left) / 2;
            let r = unsafe { guess(mid) };
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
            const V: i32 = 6;
            let _x = LOCK.lock().unwrap();
            guess = &|x| (V - x).signum();
            assert_eq!(V, Solution::guessNumber(10));
        }
    }

    #[test]
    fn case2() {
        unsafe {
            const V: i32 = 1;
            let _x = LOCK.lock().unwrap();
            guess = &|x| (V - x).signum();
            assert_eq!(V, Solution::guessNumber(1));
        }
    }
}
