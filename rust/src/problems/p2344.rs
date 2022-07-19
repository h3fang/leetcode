pub struct Solution;

use std::collections::BTreeMap;

fn gcd(a: i32, b: i32) -> i32 {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, nums_divide: Vec<i32>) -> i32 {
        let min = *nums_divide.iter().min().unwrap();
        let mut min_gcd = min;
        for &n in &nums_divide {
            let g = gcd(min, n);
            min_gcd = min_gcd.min(g);
        }

        let mut m = BTreeMap::new();
        for &n in &nums {
            *m.entry(n).or_insert(0) += 1;
        }
        let mut valid = false;
        let mut count = 0;
        for (n, c) in m {
            if min_gcd % n == 0 {
                valid = true;
                break;
            }
            count += c;
        }
        if valid {
            count
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            2,
            Solution::min_operations(vec![2, 3, 2, 4, 3], vec![9, 6, 9, 3, 15])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            -1,
            Solution::min_operations(vec![4, 3, 6], vec![8, 2, 6, 10])
        );
    }
}
