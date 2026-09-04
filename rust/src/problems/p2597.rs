pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn beautiful_subsets(nums: Vec<i32>, k: i32) -> i32 {
        let mut m: HashMap<i32, HashMap<i32, i32>> = HashMap::new();
        for x in nums {
            *m.entry(x % k).or_default().entry(x).or_default() += 1;
        }
        m.values()
            .map(|g| {
                let mut g = g.iter().map(|(k, v)| (*k, *v)).collect::<Vec<_>>();
                g.sort_unstable();
                let n = g.len();
                let mut f = vec![0; n + 1];
                f[0] = 1;
                f[1] = 1 << g[0].1;
                for (i, w) in g.windows(2).enumerate() {
                    let i = i + 1;
                    if w[1].0 - w[0].0 == k {
                        f[i + 1] = f[i] + f[i - 1] * ((1 << w[1].1) - 1);
                    } else {
                        f[i + 1] = f[i] * (1 << (w[1].1));
                    }
                }
                f[n]
            })
            .product::<i32>()
            - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::beautiful_subsets(vec![2, 4, 6], 2));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::beautiful_subsets(vec![1], 1));
    }
}
