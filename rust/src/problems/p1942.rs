pub struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn smallest_chair(times: Vec<Vec<i32>>, target_friend: i32) -> i32 {
        let n = times.len();
        let mut events = times
            .iter()
            .enumerate()
            .flat_map(|(i, v)| [(v[0], i as i32), (v[1], !(i as i32))])
            .collect::<Vec<_>>();
        events.sort_unstable();
        let mut occupied = BinaryHeap::new();
        let mut available = (0..n as i32).map(Reverse).collect::<BinaryHeap<_>>();
        for (t, i) in events {
            while let Some(&(Reverse(leave_t), j)) = occupied.peek() {
                if leave_t <= t {
                    occupied.pop();
                    available.push(Reverse(j));
                } else {
                    break;
                }
            }
            if i >= 0 {
                let j = available.pop().unwrap().0;
                if i == target_friend {
                    return j;
                }
                occupied.push((Reverse(times[i as usize][1]), j));
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let times = [[1, 4], [2, 3], [4, 6]]
            .iter()
            .map(|t| t.to_vec())
            .collect();
        assert_eq!(1, Solution::smallest_chair(times, 1));
    }

    #[test]
    fn case2() {
        let times = [[3, 10], [1, 5], [2, 6]]
            .iter()
            .map(|t| t.to_vec())
            .collect();
        assert_eq!(2, Solution::smallest_chair(times, 0));
    }
}
