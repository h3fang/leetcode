pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn max_total_reward(mut reward_values: Vec<i32>) -> i32 {
        reward_values.sort_unstable();
        reward_values.dedup();
        let m = *reward_values.last().unwrap();
        let mut set = HashSet::new();
        for &x in &reward_values {
            if x == m - 1 || set.contains(&(m - 1 - x)) {
                return 2 * m - 1;
            }
            set.insert(x);
        }
        let mut f = vec![false; 2 * m as usize];
        f[0] = true;
        for x in reward_values {
            for j in (x..2 * x).rev() {
                unsafe {
                    *f.get_unchecked_mut(j as usize) |= *f.get_unchecked((j - x) as usize);
                }
            }
        }
        f.iter().rposition(|x| *x).unwrap() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::max_total_reward(vec![1, 1, 3, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(11, Solution::max_total_reward(vec![1, 6, 4, 3, 2]));
    }
}

