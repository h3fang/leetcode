pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn maximum_length(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut m = HashMap::with_capacity(nums.len());
        let mut n = 0;
        for x in nums.iter_mut() {
            *x = *m.entry(*x).or_insert_with(|| {
                n += 1;
                n - 1
            });
        }

        let k = k as usize;
        let mut f = vec![vec![0; k + 1]; n as usize];
        let mut max = vec![0; k + 2];
        for &x in &nums {
            for i in (0..=k).rev() {
                f[x as usize][i] = f[x as usize][i].max(max[i]) + 1;
                max[i + 1] = max[i + 1].max(f[x as usize][i]);
            }
        }
        max[k + 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::maximum_length(vec![1, 2, 1, 1, 3], 2));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::maximum_length(vec![1, 2, 3, 4, 5, 1], 0));
    }
}
