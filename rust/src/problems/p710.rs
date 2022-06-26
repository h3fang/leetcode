use rand::prelude::*;
use std::collections::{HashMap, HashSet};

pub struct Solution {
    blacklist: HashMap<i32, i32>,
    k: i32,
    rng: ThreadRng,
}

impl Solution {
    pub fn new(n: i32, blacklist: Vec<i32>) -> Self {
        let k = n - blacklist.len() as i32;
        let black = blacklist.iter().cloned().collect::<HashSet<_>>();
        let mut blacklist = HashMap::new();
        let mut j = k;
        for &b in &black {
            if b < k {
                while black.contains(&j) {
                    j += 1;
                }
                blacklist.insert(b, j);
                j += 1;
            }
        }
        Self {
            blacklist,
            k,
            rng: ThreadRng::default(),
        }
    }

    pub fn pick(&mut self) -> i32 {
        let r = self.rng.gen_range(0..self.k);
        self.blacklist.get(&r).cloned().unwrap_or(r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 7;
        let blacklist = vec![2, 3, 5];
        let black = blacklist.iter().cloned().collect::<HashSet<_>>();
        let mut s = Solution::new(n, blacklist);
        let mut freq = vec![0; 7];
        const N: i32 = 100000;
        (0..N).for_each(|_| {
            let num = s.pick();
            freq[num as usize] += 1;
        });
        let f = 1.0 / 4.0;
        for k in 0..n {
            if black.contains(&k) {
                assert_eq!(0, freq[k as usize])
            } else {
                let f1 = freq[k as usize] as f64 / N as f64;
                assert!((f1 - f).abs() < 1e-2);
            }
        }
    }
}
