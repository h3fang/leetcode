pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn minimum_substrings_in_partition(s: String) -> i32 {
        let n = s.len();
        let s = s.as_bytes();
        let mut f = vec![i32::MAX / 2; n + 1];
        f[0] = 0;
        for i in 1..=n {
            let mut max = 0;
            let mut freq = HashMap::new();
            for j in (1..=i).rev() {
                let c = s[j - 1];
                let e = freq.entry(c).or_insert(0);
                *e += 1;
                max = max.max(*e);
                if max * freq.len() == i - j + 1 && f[j - 1] < i32::MAX / 2 {
                    f[i] = f[i].min(f[j - 1] + 1);
                }
            }
        }
        f[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            3,
            Solution::minimum_substrings_in_partition("fabccddg".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            2,
            Solution::minimum_substrings_in_partition("abababaccddb".to_string())
        );
    }
}
