pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn maximum_total_damage(power: Vec<i32>) -> i64 {
        let mut m = HashMap::with_capacity(20_0000);
        for p in power {
            *m.entry(p).or_insert(0) += 1;
        }
        let mut s = m.into_iter().collect::<Vec<_>>();
        s.sort_unstable();

        let n = s.len();
        let mut f = vec![0; n + 1];
        let mut j = 0;
        for (i, &(a, b)) in s.iter().enumerate() {
            while s[j].0 < a - 2 {
                j += 1;
            }
            f[i + 1] = f[i].max(f[j] + a as i64 * b as i64);
        }
        f[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(6, Solution::maximum_total_damage(vec![1, 1, 3, 4]));
    }

    #[test]
    fn case2() {
        assert_eq!(13, Solution::maximum_total_damage(vec![7, 1, 6, 6]));
    }
}
