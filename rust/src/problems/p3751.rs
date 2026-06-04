pub struct Solution;

fn digits(mut x: i32) -> Vec<i8> {
    let mut ds = Vec::with_capacity(16);
    while x > 0 {
        ds.push((x % 10) as i8);
        x /= 10;
    }
    ds.reverse();
    ds
}

struct State {
    n1: Vec<i8>,
    n2: Vec<i8>,
    cache: Vec<Vec<[[i32; 10]; 3]>>,
}

impl State {
    fn new(num1: i32, num2: i32) -> Self {
        let n1 = digits(num1);
        let n2 = digits(num2);
        let n = n2.len();
        let cache = vec![vec![[[-1; 10]; 3]; n]; n];
        Self { n1, n2, cache }
    }

    fn dfs(
        &mut self,
        i: usize,
        waviness: i32,
        last_dir: i8,
        last_d: i8,
        limit_low: bool,
        limit_high: bool,
    ) -> i32 {
        if i == self.n2.len() {
            return waviness;
        }

        let r = self.cache[i][waviness as usize][(last_dir + 1) as usize][last_d as usize];
        if !limit_low && !limit_high && r >= 0 {
            return r;
        }

        let diff = self.n2.len() - self.n1.len();
        let low = if limit_low && i >= diff {
            self.n1[i - diff]
        } else {
            0
        };
        let high = if limit_high { self.n2[i] } else { 9 };

        let mut ans = 0;
        let is_num = !limit_low || i > diff;
        for d in low..=high {
            let dir = if is_num { (d - last_d).signum() } else { 0 };
            let w = waviness + i32::from(dir * last_dir < 0);
            ans += self.dfs(
                i + 1,
                w,
                dir,
                d,
                limit_low && d == low,
                limit_high && d == high,
            );
        }

        if !limit_low && !limit_high {
            self.cache[i][waviness as usize][(last_dir + 1) as usize][last_d as usize] = ans;
        }

        ans
    }
}

impl Solution {
    pub fn total_waviness(num1: i32, num2: i32) -> i32 {
        let mut s = State::new(num1, num2);
        s.dfs(0, 0, 0, 0, true, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::total_waviness(120, 130));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::total_waviness(198, 202));
    }

    #[test]
    fn case3() {
        assert_eq!(2, Solution::total_waviness(4848, 4848));
    }
}
