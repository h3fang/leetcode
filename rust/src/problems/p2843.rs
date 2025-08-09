pub struct Solution;

use std::collections::HashMap;

fn digits(mut x: i32) -> Vec<i8> {
    let mut ans = Vec::with_capacity(5);
    while x > 0 {
        ans.push((x % 10) as i8);
        x /= 10;
    }
    ans.reverse();
    ans
}

struct Input {
    low: Vec<i8>,
    high: Vec<i8>,
    cache: HashMap<(i8, i8, i8), i32>,
}

impl Input {
    fn dfs(&mut self, i: i8, start: i8, diff: i8, limit_low: bool, limit_high: bool) -> i32 {
        let n = self.high.len() as i8;
        if i == n {
            return i32::from(diff == 0);
        }
        if start != -1
            && !limit_low
            && !limit_high
            && let Some(&r) = self.cache.get(&(i, start, diff))
        {
            return r;
        }
        let gap = n - self.low.len() as i8;
        let lb = if i >= gap && limit_low {
            self.low[(i - gap) as usize]
        } else {
            0
        };
        let ub = if limit_high { self.high[i as usize] } else { 9 };
        if start == -1 && (n - i) % 2 == 1 {
            if lb > 0 {
                return 0;
            } else {
                return self.dfs(i + 1, start, diff, limit_low, limit_high && lb == ub);
            }
        }
        let mut ans = 0;
        let left = start == -1 || i < (n + start) / 2;
        for d in lb..=ub {
            let start = if start < 0 && d > 0 { i } else { start };
            ans += self.dfs(
                i + 1,
                start,
                diff + if left { -d } else { d },
                limit_low && d == lb,
                limit_high && d == ub,
            );
        }
        if start != -1 && !limit_low && !limit_high {
            self.cache.insert((i, start, diff), ans);
        }
        ans
    }
}

impl Solution {
    pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
        let low = digits(low);
        let high = digits(high);
        let mut input = Input {
            low,
            high,
            cache: HashMap::with_capacity(1024),
        };
        input.dfs(0, -1, 0, true, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(9, Solution::count_symmetric_integers(1, 100));
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::count_symmetric_integers(1200, 1230));
    }
}
