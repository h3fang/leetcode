pub struct Solution;

const MOD: i32 = 10_0000_0007;

struct State {
    n1: Vec<u8>,
    n2: Vec<u8>,
    cache: Vec<Vec<i32>>,
}

impl State {
    fn dfs(&mut self, i: usize, sum: i32, min_sum: i32, max_sum: i32, lb: bool, ub: bool) -> i32 {
        if sum > max_sum {
            return 0;
        }
        if i == self.n1.len() {
            return i32::from(sum >= min_sum);
        }
        if !lb && !ub && self.cache[i][sum as usize] != -1 {
            return self.cache[i][sum as usize];
        }
        let min = if lb { self.n1[i] - b'0' } else { 0 } as i32;
        let max = if ub { self.n2[i] - b'0' } else { 9 } as i32;
        let mut ans = 0;
        for d in min..=max {
            let r = self.dfs(
                i + 1,
                sum + d,
                min_sum,
                max_sum,
                lb && d == min,
                ub && d == max,
            );
            ans = (ans + r) % MOD;
        }
        if !lb && !ub {
            self.cache[i][sum as usize] = ans;
        }
        ans
    }
}

impl Solution {
    pub fn count(num1: String, num2: String, min_sum: i32, max_sum: i32) -> i32 {
        let cache = vec![vec![-1; max_sum as usize + 1]; num2.len()];
        let num1 = "0".repeat(num2.len() - num1.len()) + &num1;
        let mut s = State {
            n1: num1.into_bytes(),
            n2: num2.into_bytes(),
            cache,
        };
        s.dfs(0, 0, min_sum, max_sum, true, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(11, Solution::count("1".to_string(), "12".to_string(), 1, 8));
    }

    #[test]
    fn case2() {
        assert_eq!(5, Solution::count("1".to_string(), "5".to_string(), 1, 5));
    }
}
