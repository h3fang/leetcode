pub struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn magic_tower(nums: Vec<i32>) -> i32 {
        if nums.iter().map(|&x| x as i64).sum::<i64>() < 0 {
            return -1;
        }
        let mut result = 0;
        let mut hp = 1;
        let mut h = BinaryHeap::new();
        for &x in &nums {
            if x < 0 {
                h.push(-x);
            }
            hp += x as i64;
            if hp < 1 {
                hp += h.pop().unwrap() as i64;
                result += 1;
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
        assert_eq!(
            1,
            Solution::magic_tower(vec![100, 100, 100, -250, -60, -140, -50, -50, 100, 150])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(-1, Solution::magic_tower(vec![-200, -300, 400, 0]));
    }
}
