pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let mut g = vec![vec![]; n as usize + 1];
        for t in &times {
            g[t[0] as usize].push((t[1], t[2]));
        }
        let mut times = vec![i32::MAX; n as usize + 1];
        times[k as usize] = 0;
        let mut q = VecDeque::new();
        q.push_back((k, 0));
        while let Some((node, t)) = q.pop_front() {
            for &(next, weight) in &g[node as usize] {
                let t1 = t + weight;
                if times[next as usize] > t1 {
                    times[next as usize] = t1;
                    q.push_back((next, t1));
                }
            }
        }
        let max = *times.iter().skip(1).max().unwrap();
        if max == i32::MAX { -1 } else { max }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let times = vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]];
        let n = 4;
        let k = 2;
        assert_eq!(2, Solution::network_delay_time(times, n, k));
    }

    #[test]
    fn case2() {
        let times = vec![vec![1, 2, 1]];
        let n = 2;
        let k = 1;
        assert_eq!(1, Solution::network_delay_time(times, n, k));
    }

    #[test]
    fn case3() {
        let times = vec![vec![1, 2, 1]];
        let n = 2;
        let k = 2;
        assert_eq!(-1, Solution::network_delay_time(times, n, k));
    }
}
