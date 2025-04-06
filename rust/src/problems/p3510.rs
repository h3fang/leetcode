pub struct Solution;

use std::collections::BTreeSet;

impl Solution {
    pub fn minimum_pair_removal(nums: Vec<i32>) -> i32 {
        let mut nums: Vec<i64> = nums.into_iter().map(|x| x as i64).collect();
        let n = nums.len();
        let mut rem: BTreeSet<usize> = (0..n).collect();
        let mut pairs = BTreeSet::new();
        let (mut inverted, mut ans) = (0, 0);
        for (i, w) in nums.windows(2).enumerate() {
            if w[0] > w[1] {
                inverted += 1;
            }
            pairs.insert((w[0] + w[1], i));
        }
        while inverted > 0 {
            let (sum, i) = pairs.pop_first().unwrap();
            let next = *rem.range(i..).nth(1).unwrap();
            if nums[i] > nums[next] {
                inverted -= 1;
            }
            if let Some(&prev) = rem.range(..i).last() {
                if nums[prev] > nums[i] {
                    inverted -= 1;
                }
                if nums[prev] > sum {
                    inverted += 1;
                }
                pairs.remove(&(nums[prev] + nums[i], prev));
                pairs.insert((nums[prev] + sum, prev));
            }
            if let Some(&next2) = rem.range(i..).nth(2) {
                if nums[next] > nums[next2] {
                    inverted -= 1;
                }
                if sum > nums[next2] {
                    inverted += 1;
                }
                pairs.remove(&(nums[next] + nums[next2], next));
                pairs.insert((sum + nums[next2], i));
            }
            rem.remove(&next);
            nums[i] = sum;
            ans += 1;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::minimum_pair_removal(vec![5, 2, 3, 1]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::minimum_pair_removal(vec![1, 2, 2]));
    }

    #[test]
    fn case3() {
        assert_eq!(
            9,
            Solution::minimum_pair_removal(vec![2, 2, -1, 3, -2, 2, 1, 1, 1, 0, -1])
        );
    }
}
