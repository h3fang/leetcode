pub struct Solution;

use std::collections::HashMap;

const MOD: i32 = 10_0000_0007;

struct Input {
    low: Vec<u8>,
    high: Vec<u8>,
    cache: HashMap<(u16, u8), i32>,
    base: u8,
}

impl Input {
    fn dfs(&mut self, i: u16, prev: u8, limit_low: bool, limit_high: bool) -> i32 {
        let key = (i, prev);
        let i = i as usize;
        if i == self.high.len() {
            return 1;
        }
        if !limit_high
            && !limit_low
            && let Some(&r) = self.cache.get(&key)
        {
            return r;
        }
        let mut ans = 0;
        let lo = if limit_low { self.low[i] } else { 0 };
        let hi = if limit_high {
            self.high[i]
        } else {
            self.base - 1
        };
        for d in lo.max(prev)..=hi {
            ans += self.dfs(i as u16 + 1, d, limit_low && d == lo, limit_high && d == hi);
            ans %= MOD;
        }
        if !limit_low && !limit_high {
            self.cache.insert(key, ans);
        }
        ans
    }
}

fn convert(x: String, base: u8) -> Vec<u8> {
    let mut x = x.into_bytes();
    x.iter_mut().for_each(|e| *e -= b'0');
    let mut ans = Vec::with_capacity(x.len());
    while !x.is_empty() {
        let mut next = Vec::with_capacity(x.len());
        let mut rem = 0;
        for e in x {
            rem = rem * 10 + e;
            let q = rem / base;
            if !next.is_empty() || q > 0 {
                next.push(q);
            }
            rem %= base;
        }
        ans.push(rem);
        x = next;
    }
    ans.reverse();
    ans
}

impl Solution {
    pub fn count_numbers(l: String, r: String, b: i32) -> i32 {
        let base = b as u8;
        let high = convert(r, base);
        let low = {
            let low = convert(l, base);
            if high.len() > low.len() {
                let mut r = vec![0; high.len() - low.len()];
                r.extend(low);
                r
            } else {
                low
            }
        };
        let mut input = Input {
            low,
            high,
            base,
            cache: HashMap::with_capacity(1024),
        };
        input.dfs(0, 0, true, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            3,
            Solution::count_numbers("23".to_string(), "28".to_string(), 8)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            2,
            Solution::count_numbers("2".to_string(), "7".to_string(), 2)
        );
    }
}
