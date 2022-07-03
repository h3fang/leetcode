pub struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, mut stations: Vec<Vec<i32>>) -> i32 {
        stations.push(vec![target, 0]);
        let mut q = BinaryHeap::new();
        let mut result = 0;
        let mut prev = 0;
        let mut fuel = start_fuel;
        for s in stations {
            let curr = s[0];
            fuel -= curr - prev;
            while fuel < 0 && !q.is_empty() {
                let f = q.pop().unwrap();
                fuel += f;
                result += 1;
            }
            if fuel < 0 {
                return -1;
            }
            q.push(s[1]);
            prev = curr;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(0, Solution::min_refuel_stops(1, 1, vec![]));
    }

    #[test]
    fn case2() {
        assert_eq!(-1, Solution::min_refuel_stops(100, 1, vec![vec![10, 100]]));
    }

    #[test]
    fn case3() {
        assert_eq!(
            2,
            Solution::min_refuel_stops(
                100,
                10,
                vec![vec![10, 60], vec![20, 30], vec![30, 30], vec![60, 40]]
            )
        );
    }
}
