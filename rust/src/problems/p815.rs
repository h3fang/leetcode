pub struct Solution;

use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        let mut buses: HashMap<i32, Vec<i32>> = HashMap::with_capacity(1024);
        for (i, r) in routes.iter().enumerate() {
            for &x in r {
                buses.entry(x).or_default().push(i as i32);
            }
        }
        let mut q = VecDeque::new();
        let mut visited_stops = HashSet::new();
        let mut visited_buses = HashSet::new();
        q.push_back((source, 0));
        while let Some((i, cost)) = q.pop_front() {
            if i == target {
                return cost;
            }
            visited_stops.insert(i);
            if let Some(v) = buses.get(&i) {
                for &j in v {
                    if visited_buses.contains(&j) {
                        continue;
                    }
                    visited_buses.insert(j);
                    for &k in &routes[j as usize] {
                        if visited_stops.contains(&k) {
                            continue;
                        }
                        visited_stops.insert(k);
                        q.push_back((k, cost + 1));
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
        let routes = vec![vec![1, 2, 7], vec![3, 6, 7]];
        assert_eq!(2, Solution::num_buses_to_destination(routes, 1, 6));
    }

    #[test]
    fn case2() {
        let routes = vec![
            vec![7, 12],
            vec![4, 5, 15],
            vec![6],
            vec![15, 19],
            vec![9, 12, 13],
        ];
        assert_eq!(-1, Solution::num_buses_to_destination(routes, 15, 12));
    }
}
