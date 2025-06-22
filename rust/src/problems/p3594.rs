pub struct Solution;

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug)]
struct State {
    time: f64,
    stage: usize,
    left: usize,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

impl Eq for State {}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.time.partial_cmp(&self.time).unwrap()
    }
}

impl Solution {
    pub fn min_time(n: i32, k: i32, m: i32, time: Vec<i32>, mul: Vec<f64>) -> f64 {
        let (n, k, m) = (n as usize, k as usize, m as usize);
        let total = 1 << n;

        let mut max_time = vec![0.0f64; total];
        for (i, &t) in time.iter().enumerate() {
            let h = 1 << i;
            for m in 0..h {
                max_time[h | m] = max_time[m].max(t as f64);
            }
        }

        let mut subsets = Vec::with_capacity(total);
        for state in 0..total {
            let mut subs = Vec::new();
            let mut s = state;
            while s != 0 {
                if s.count_ones() <= k as u32 {
                    subs.push(s);
                }
                s = (s - 1) & state;
            }
            subsets.push(subs);
        }

        let mut dist = vec![vec![f64::MAX; total]; m];
        let mut q = BinaryHeap::new();

        dist[0][total - 1] = 0.0;
        q.push(State {
            time: 0.0,
            stage: 0,
            left: total - 1,
        });

        while let Some(State { time, stage, left }) = q.pop() {
            if left == 0 {
                return time;
            }
            if time > dist[stage][left] {
                continue;
            }

            for &sub in &subsets[left] {
                let t1 = max_time[sub] * mul[stage];
                let s1 = (stage + t1.floor() as usize) % m;

                if sub == left {
                    if time + t1 < dist[s1][0] {
                        dist[s1][0] = time + t1;
                        q.push(State {
                            time: time + t1,
                            stage: s1,
                            left: 0,
                        });
                    }
                    continue;
                }
                let mut s = ((total - 1) ^ left ^ sub) as i64;
                while s > 0 {
                    let lb = (s & (-s)) as usize;
                    let t2 = max_time[lb] * mul[s1];
                    let s2 = (s1 + t2.floor() as usize) % m;
                    let time = time + t1 + t2;
                    let left = left ^ sub ^ lb;
                    if time < dist[s2][left] {
                        dist[s2][left] = time;
                        q.push(State {
                            time,
                            stage: s2,
                            left,
                        });
                    }
                    s ^= lb as i64;
                }
            }
        }

        -1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-5, "a = {}, b = {}", a, b);
    }

    #[test]
    fn case1() {
        assert_close(5.0, Solution::min_time(1, 1, 2, vec![5], vec![1.0, 1.3]));
    }

    #[test]
    fn case2() {
        assert_close(
            14.5,
            Solution::min_time(3, 2, 3, vec![2, 5, 8], vec![1.0, 1.5, 0.75]),
        );
    }

    #[test]
    fn case3() {
        assert_close(
            -1.0,
            Solution::min_time(2, 1, 2, vec![10, 10], vec![2.0, 2.0]),
        );
    }

    #[test]
    fn case4() {
        assert_close(
            215.96,
            Solution::min_time(4, 2, 4, vec![51, 57, 22, 73], vec![0.98, 1.68, 0.55, 1.71]),
        );
    }
}
