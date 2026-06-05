pub struct Solution;

fn digits(mut x: i64) -> Vec<i8> {
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
    cache: Vec<[[(i64, i64); 10]; 3]>,
}

impl State {
    fn new(num1: i64, num2: i64) -> Self {
        let n1 = digits(num1);
        let n2 = digits(num2);
        let n = n2.len();
        let cache = vec![[[(0, 0); 10]; 3]; n];
        Self { n1, n2, cache }
    }

    fn dfs(
        &mut self,
        i: usize,
        last_dir: i8,
        last_digit: i8,
        limit_low: bool,
        limit_high: bool,
    ) -> (i64, i64) {
        if i == self.n2.len() {
            return (0, 1);
        }

        let r = self.cache[i][(last_dir + 1) as usize][last_digit as usize];
        if !limit_low && !limit_high && r.1 > 0 {
            return r;
        }

        let diff = self.n2.len() - self.n1.len();
        let low = if limit_low && i >= diff {
            self.n1[i - diff]
        } else {
            0
        };
        let high = if limit_high { self.n2[i] } else { 9 };

        let (mut waviness, mut count) = (0, 0);
        let is_num = !limit_low || i > diff;
        for digit in low..=high {
            let dir = if is_num {
                (digit - last_digit).signum()
            } else {
                0
            };
            let (sub_wave, sub_count) = self.dfs(
                i + 1,
                dir,
                digit,
                limit_low && digit == low,
                limit_high && digit == high,
            );
            waviness += sub_wave;
            count += sub_count;
            if dir * last_dir < 0 {
                waviness += sub_count;
            }
        }

        if !limit_low && !limit_high {
            self.cache[i][(last_dir + 1) as usize][last_digit as usize] = (waviness, count);
        }

        (waviness, count)
    }
}

impl Solution {
    pub fn total_waviness(num1: i64, num2: i64) -> i64 {
        let mut s = State::new(num1, num2);
        s.dfs(0, 0, 0, true, true).0
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
