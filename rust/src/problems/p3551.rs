pub struct Solution;

use std::collections::HashMap;

fn digits_sum(mut x: i32) -> i32 {
    let mut sum = 0;
    while x > 0 {
        sum += x % 10;
        x /= 10;
    }
    sum
}

impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut sums = Vec::with_capacity(n);
        for &x in &nums {
            sums.push((digits_sum(x), x));
        }
        sums.sort_unstable();
        let mut pos = HashMap::with_capacity(n);
        for (i, &(_, x)) in sums.iter().enumerate() {
            pos.insert(x, i);
        }
        let mut vis = vec![false; n];
        let mut ans = 0;
        for i in 0..n {
            if vis[i] {
                continue;
            }
            let mut cycles = 0;
            let mut j = i;
            while !vis[j] {
                vis[j] = true;
                j = *pos.get(&nums[j]).unwrap();
                cycles += 1;
            }
            if cycles > 1 {
                ans += cycles - 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::min_swaps(vec![37, 100]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::min_swaps(vec![22, 14, 33, 7]));
    }

    #[test]
    fn case3() {
        assert_eq!(2, Solution::min_swaps(vec![18, 43, 34, 16]));
    }

    #[test]
    fn case4() {
        assert_eq!(2, Solution::min_swaps(vec![268835996, 65052660, 415128775]));
    }
}
