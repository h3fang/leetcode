pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn flip_lights(n: i32, presses: i32) -> i32 {
        let mut seen = HashSet::new();
        for i in 0..16 {
            let mut arr = [0; 4];
            for (j, a) in arr.iter_mut().enumerate() {
                *a = (i >> j) & 1;
            }
            let s = arr.iter().sum::<i32>();
            if s % 2 != presses % 2 || s > presses {
                continue;
            }
            let mut status = arr[0] ^ arr[1] ^ arr[3];
            if n >= 2 {
                status |= (arr[0] ^ arr[1]) << 1;
            }
            if n >= 3 {
                status |= (arr[0] ^ arr[2]) << 2;
            }
            if n >= 4 {
                status |= (arr[0] ^ arr[1] ^ arr[3]) << 3;
            }
            seen.insert(status);
        }
        seen.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::flip_lights(1, 1));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::flip_lights(2, 1));
    }

    #[test]
    fn case3() {
        assert_eq!(4, Solution::flip_lights(3, 1));
    }
}
