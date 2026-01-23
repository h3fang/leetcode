pub struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn minimum_pair_removal(nums: Vec<i32>) -> i32 {
        let mut nums: Vec<i64> = nums.into_iter().map(|x| x as i64).collect();
        let n = nums.len();
        let mut pairs: BinaryHeap<_> = BinaryHeap::with_capacity(n);
        let (mut inverted, mut ans) = (0, 0);
        for (i, w) in nums.windows(2).enumerate() {
            if w[0] > w[1] {
                inverted += 1;
            }
            pairs.push(Reverse((w[0] + w[1], i)));
        }

        let n = n as i32;
        let mut left: Vec<i32> = (-1..n).collect();
        let mut right: Vec<i32> = (1..n + 1).collect();
        while inverted > 0 {
            while let Some(&Reverse((s, i))) = pairs.peek()
                && (right[i] == n || nums[i] + nums[right[i] as usize] != s)
            {
                pairs.pop();
            }

            let Reverse((sum, i)) = pairs.pop().unwrap();
            let next = right[i] as usize;
            if nums[i] > nums[next] {
                inverted -= 1;
            }

            if left[i] >= 0 {
                let prev = left[i] as usize;
                if nums[prev] > nums[i] {
                    inverted -= 1;
                }
                if nums[prev] > sum {
                    inverted += 1;
                }
                pairs.push(Reverse((nums[prev] + sum, prev)));
            }
            let next2 = right[next];
            if next2 < n {
                let next2 = next2 as usize;
                if nums[next] > nums[next2] {
                    inverted -= 1;
                }
                if sum > nums[next2] {
                    inverted += 1;
                }
                pairs.push(Reverse((sum + nums[next2], i)));
            }
            right[i] = next2;
            left[next2 as usize] = i as i32;
            right[next] = n;
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
