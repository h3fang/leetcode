pub struct Solution;

use std::{
    collections::{HashMap, VecDeque},
    sync::OnceLock,
};

const MAX: usize = 100_0001;

static PRIME_FACTORS: OnceLock<Vec<Vec<i32>>> = OnceLock::new();

fn prime_factors() -> Vec<Vec<i32>> {
    let mut p = vec![vec![]; MAX];
    for i in 2..MAX {
        if p[i].is_empty() {
            for j in (i..MAX).step_by(i) {
                p[j].push(i as i32);
            }
        }
    }
    p
}

impl Solution {
    pub fn min_jumps(nums: Vec<i32>) -> i32 {
        let p = PRIME_FACTORS.get_or_init(prime_factors);
        let n = nums.len();
        let mut groups: HashMap<i32, Vec<u32>> = HashMap::with_capacity(n + 1);
        for (i, &x) in nums.iter().enumerate() {
            if p[x as usize].len() == 1 {
                groups.entry(x).or_default().push(i as u32);
            }
        }
        let mut vis = vec![false; n];
        let mut q = VecDeque::with_capacity(n);
        q.push_back((0, n - 1));
        vis[n - 1] = true;
        while let Some((d, i)) = q.pop_front() {
            if i == 0 {
                return d;
            }
            if i > 0 && !vis[i - 1] {
                vis[i - 1] = true;
                q.push_back((d + 1, i - 1));
            }
            if i + 1 < n && !vis[i + 1] {
                vis[i + 1] = true;
                q.push_back((d + 1, i + 1));
            }
            for p in &p[nums[i] as usize] {
                if let Some(g) = groups.get_mut(p) {
                    for j in g.drain(..) {
                        if !vis[j as usize] {
                            vis[j as usize] = true;
                            q.push_back((d + 1, j as usize));
                        }
                    }
                }
            }
        }
        n as i32 - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::min_jumps(vec![1, 2, 4, 6]));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::min_jumps(vec![2, 3, 4, 7, 9]));
    }

    #[test]
    fn case3() {
        assert_eq!(3, Solution::min_jumps(vec![4, 6, 5, 8]));
    }
}
