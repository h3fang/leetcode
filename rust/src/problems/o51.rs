use std::vec;

pub struct Solution;

struct Bit {
    tree: Vec<i32>,
}

fn lowbit(x: usize) -> usize {
    let x = x as isize;
    (x & (-x)) as usize
}

impl Bit {
    fn new(n: usize) -> Self {
        Self {
            tree: vec![0; n + 1],
        }
    }

    fn update(&mut self, mut x: usize) {
        let n = self.tree.len();
        while x < n {
            self.tree[x] += 1;
            x += lowbit(x);
        }
    }

    fn query(&self, mut x: usize) -> i32 {
        let mut r = 0;
        while x > 0 {
            r += self.tree[x];
            x -= lowbit(x);
        }
        r
    }
}

impl Solution {
    pub fn reverse_pairs(mut nums: Vec<i32>) -> i32 {
        fn dc(nums: &mut Vec<i32>, sorted: &mut Vec<i32>, i: usize, j: usize) -> i32 {
            if i != j {
                let mid = (i + j) / 2;
                let mut result = dc(nums, sorted, i, mid) + dc(nums, sorted, mid + 1, j);
                let mut left = i;
                let mut right = mid + 1;
                for s in sorted[i..=j].iter_mut() {
                    if left == mid + 1 {
                        *s = nums[right];
                        right += 1;
                    } else if right == j + 1 || nums[left] <= nums[right] {
                        *s = nums[left];
                        left += 1;
                    } else {
                        result += (mid - left) as i32 + 1;
                        *s = nums[right];
                        right += 1;
                    }
                }
                nums[i..=j].clone_from_slice(&sorted[i..=j]);
                result
            } else {
                0
            }
        }
        let n = nums.len();
        if n == 0 {
            return 0;
        }
        let mut sorted = vec![0; n];
        dc(&mut nums, &mut sorted, 0, n - 1)
    }

    pub fn reverse_pairs_bit(mut nums: Vec<i32>) -> i32 {
        fn lower_bound(sorted: &[i32], x: i32) -> usize {
            let mut left = 0;
            let mut right = sorted.len() - 1;
            while left < right {
                let mid = (left + right) / 2;
                match sorted[mid].cmp(&x) {
                    std::cmp::Ordering::Less => left = mid + 1,
                    _ => right = mid,
                }
            }
            right
        }
        let n = nums.len();
        if n == 0 {
            return 0;
        }
        let mut sorted = nums.clone();
        sorted.sort_unstable();
        for n in nums.iter_mut() {
            *n = lower_bound(&sorted, *n) as i32 + 1;
        }
        let mut bit = Bit::new(n);
        let mut result = 0;
        for n in nums.iter().rev() {
            result += bit.query(*n as usize - 1);
            bit.update(*n as usize);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::reverse_pairs(vec![7, 5, 6, 4]));
        assert_eq!(5, Solution::reverse_pairs_bit(vec![7, 5, 6, 4]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::reverse_pairs(vec![4, 5, 6, 7]));
        assert_eq!(0, Solution::reverse_pairs_bit(vec![4, 5, 6, 7]));
    }
}
