pub struct Solution;

use std::collections::HashMap;

fn reverse(mut x: i32) -> i32 {
    let mut ans = 0;
    while x > 0 {
        ans = ans * 10 + x % 10;
        x /= 10;
    }
    ans
}

impl Solution {
    pub fn min_mirror_pair_distance(nums: Vec<i32>) -> i32 {
        let mut m = HashMap::with_capacity(nums.len() * 2);
        let mut ans = i32::MAX;

        for (i, &x) in nums.iter().enumerate().rev() {
            let rev = reverse(x);
            if let Some(j) = m.get(&rev) {
                ans = ans.min(j - i as i32);
            }

            m.insert(x, i as i32);
        }

        if ans == i32::MAX { -1 } else { ans }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            1,
            Solution::min_mirror_pair_distance(vec![12, 21, 45, 33, 54])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::min_mirror_pair_distance(vec![120, 21]));
    }

    #[test]
    fn case3() {
        assert_eq!(-1, Solution::min_mirror_pair_distance(vec![21, 120]));
    }
}
