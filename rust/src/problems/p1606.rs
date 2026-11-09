pub struct Solution;

use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap};

impl Solution {
    pub fn busiest_servers(k: i32, arrival: Vec<i32>, load: Vec<i32>) -> Vec<i32> {
        let mut idle = (0..k).collect::<BTreeSet<_>>();
        let mut busy: BinaryHeap<(Reverse<i32>, i32)> = BinaryHeap::new();
        let mut reqs = vec![0; k as usize];
        for (i, &arr) in arrival.iter().enumerate() {
            while busy.peek().is_some_and(|(Reverse(t), _)| *t <= arr) {
                let (_, idx) = busy.pop().unwrap();
                idle.insert(idx);
            }
            if idle.is_empty() {
                continue;
            }
            let j = *idle
                .range((i as i32 % k)..)
                .next()
                .unwrap_or_else(|| idle.iter().next().unwrap());

            reqs[j as usize] += 1;
            idle.remove(&j);
            busy.push((Reverse(arr + load[i]), j));
        }
        let max = reqs.iter().max().unwrap();
        reqs.iter()
            .enumerate()
            .filter_map(|(i, e)| if e == max { Some(i as i32) } else { None })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let k = 3;
        let arrival = vec![1, 2, 3, 4, 5];
        let load = vec![5, 2, 3, 3, 3];
        assert_eq!(vec![1], Solution::busiest_servers(k, arrival, load));
    }

    #[test]
    fn case2() {
        let k = 3;
        let arrival = vec![1, 2, 3, 4];
        let load = vec![1, 2, 1, 2];
        assert_eq!(vec![0], Solution::busiest_servers(k, arrival, load));
    }

    #[test]
    fn case3() {
        let k = 3;
        let arrival = vec![1, 2, 3];
        let load = vec![10, 12, 11];
        assert_eq!(vec![0, 1, 2], Solution::busiest_servers(k, arrival, load));
    }

    #[test]
    fn case4() {
        let k = 3;
        let arrival = vec![1, 2, 3, 4, 8, 9, 10];
        let load = vec![5, 2, 10, 3, 1, 2, 2];
        assert_eq!(vec![1], Solution::busiest_servers(k, arrival, load));
    }

    #[test]
    fn case5() {
        let k = 1;
        let arrival = vec![1];
        let load = vec![1];
        assert_eq!(vec![0], Solution::busiest_servers(k, arrival, load));
    }
}
