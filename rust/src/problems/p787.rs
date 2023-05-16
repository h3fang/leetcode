pub struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let n = n as usize;
        let mut g = vec![vec![]; n];

        for f in flights {
            g[f[0] as usize].push((f[1] as usize, f[2]));
        }
        let mut q = BinaryHeap::new();
        q.push(Reverse((0, src as usize, 0)));

        let mut dist = vec![i32::MAX; n];
        dist[src as usize] = 0;

        while let Some(Reverse((steps, i, total_distance))) = q.pop() {
            if steps > k {
                break;
            }
            for &(j, weight) in &g[i] {
                let new = weight + total_distance;
                if new < dist[j] {
                    dist[j] = new;
                    q.push(Reverse((steps + 1, j, new)));
                }
            }
        }

        if dist[dst as usize] == i32::MAX {
            -1
        } else {
            dist[dst as usize]
        }
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    #[test]
    fn case1() {
        let flights = [[0, 1, 100], [1, 2, 100], [0, 2, 500]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(200, Solution::find_cheapest_price(3, flights, 0, 2, 1));
    }

    #[test]
    fn case2() {
        let flights = [[0, 1, 100], [1, 2, 100], [0, 2, 500]]
            .iter()
            .map(|e: &[i32; 3]| e.to_vec())
            .collect();
        assert_eq!(500, Solution::find_cheapest_price(3, flights, 0, 2, 0));
    }
}
