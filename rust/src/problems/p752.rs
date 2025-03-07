pub struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let mut visited = vec![false; 10000];
        for d in deadends {
            let d: usize = d.parse().unwrap();
            if d == 0 {
                return -1;
            }
            visited[d] = true;
        }

        let mut dist = vec![i32::MAX; 10000];
        dist[0] = 0;
        let mut q = BinaryHeap::new();
        q.push((Reverse(0), 0));
        let target: i32 = target.parse().unwrap();
        while let Some((Reverse(dis), x)) = q.pop() {
            if x == target {
                return dis;
            }
            if dist[x as usize] < dis {
                continue;
            }
            visited[x as usize] = true;
            for delta in [1, 10, 100, 1000] {
                let d = (x / delta) % 10;
                for d_new in [(d + 1) % 10, (d + 10 - 1) % 10] {
                    let y = x + (d_new - d) * delta;
                    if !visited[y as usize] && dis + 1 < dist[y as usize] {
                        dist[y as usize] = dis + 1;
                        q.push((Reverse(dis + 1), y));
                    }
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let deadends = ["0201", "0101", "0102", "1212", "2002"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(6, Solution::open_lock(deadends, "0202".to_string()));
    }

    #[test]
    fn case2() {
        let deadends = ["8888"].iter().map(|s| s.to_string()).collect();
        assert_eq!(1, Solution::open_lock(deadends, "0009".to_string()));
    }

    #[test]
    fn case3() {
        let deadends = [
            "8887", "8889", "8878", "8898", "8788", "8988", "7888", "9888",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(-1, Solution::open_lock(deadends, "8888".to_string()));
    }
}
