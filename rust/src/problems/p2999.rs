pub struct Solution;

struct Input {
    low: Vec<u8>,
    high: Vec<u8>,
    s: Vec<u8>,
    limit: u8,
    cache: Vec<i64>,
}

impl Input {
    fn dfs(&mut self, i: usize, limit_high: bool, limit_low: bool) -> i64 {
        if i == self.low.len() {
            return 1;
        }
        if !limit_low && !limit_high && self.cache[i] != -1 {
            return self.cache[i];
        }
        let l = if limit_low { self.low[i] } else { b'0' };
        let h = if limit_high { self.high[i] } else { b'9' };
        let mut ans = 0;
        if i < self.low.len() - self.s.len() {
            for d in l..=h.min(self.limit) {
                ans += self.dfs(i + 1, limit_high && d == h, limit_low && d == l);
            }
        } else {
            let d = self.s[i - (self.low.len() - self.s.len())];
            if d >= l && d <= h.min(self.limit) {
                ans += self.dfs(i + 1, limit_high && d == h, limit_low && d == l);
            }
        }
        if !limit_low && !limit_high {
            self.cache[i] = ans;
        }
        ans
    }
}

impl Solution {
    pub fn number_of_powerful_int(start: i64, finish: i64, limit: i32, s: String) -> i64 {
        let limit = limit as u8 + b'0';
        let mut start = start.to_string();
        let finish = finish.to_string();
        if start.len() < finish.len() {
            start = format!("{}{start}", "0".repeat(finish.len() - start.len()));
        }
        let mut input = Input {
            cache: vec![-1; start.len()],
            low: start.into_bytes(),
            high: finish.into_bytes(),
            s: s.into_bytes(),
            limit,
        };
        input.dfs(0, true, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            5,
            Solution::number_of_powerful_int(1, 6000, 4, "124".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            2,
            Solution::number_of_powerful_int(15, 215, 6, "10".to_string())
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            0,
            Solution::number_of_powerful_int(1000, 2000, 4, "3000".to_string())
        );
    }

    #[test]
    fn case4() {
        assert_eq!(
            9,
            Solution::number_of_powerful_int(1, 971, 9, "72".to_string())
        );
    }

    #[test]
    fn case5() {
        assert_eq!(
            5470,
            Solution::number_of_powerful_int(1829505, 1255574165, 7, "11223".to_string())
        );
    }
}
