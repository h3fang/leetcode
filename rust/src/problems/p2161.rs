pub struct Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut result = vec![pivot; nums.len()];
        let (mut l, mut r) = (0, nums.len() - 1);
        for &n in &nums {
            match n.cmp(&pivot) {
                Ordering::Less => {
                    result[l] = n;
                    l += 1;
                }
                Ordering::Equal => {}
                Ordering::Greater => {
                    result[r] = n;
                    r = r.saturating_sub(1);
                }
            }
        }
        result[r + 1..].reverse();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![9, 5, 3, 10, 10, 12, 14],
            Solution::pivot_array(vec![9, 12, 5, 10, 14, 3, 10], 10)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![-14, -11, -7, 19, 15, 12, 8],
            Solution::pivot_array(vec![19, 15, 12, -14, 8, -7, -11], -7)
        );
    }
}
