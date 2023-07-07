pub struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn find_crossing_time(mut n: i32, _k: i32, time: Vec<Vec<i32>>) -> i32 {
        let mut left: BinaryHeap<(i32, usize)> = BinaryHeap::new();
        let mut right: BinaryHeap<(i32, usize)> = BinaryHeap::new();
        let mut pickup: BinaryHeap<(Reverse<i32>, usize)> = BinaryHeap::new();
        let mut putdown: BinaryHeap<(Reverse<i32>, usize)> = BinaryHeap::new();
        let priority = |i: usize| -> i32 { time[i][0] + time[i][2] };
        let mut t = 0;
        for i in 0..time.len() {
            left.push((priority(i), i));
        }
        while n > 0 || !right.is_empty() || !pickup.is_empty() {
            while let Some(&(Reverse(ti), i)) = pickup.peek() {
                if t >= ti {
                    pickup.pop();
                    right.push((priority(i), i));
                } else {
                    break;
                }
            }
            while let Some(&(Reverse(ti), i)) = putdown.peek() {
                if t >= ti {
                    putdown.pop();
                    left.push((priority(i), i));
                } else {
                    break;
                }
            }
            if !right.is_empty() {
                let i = right.pop().unwrap().1;
                t += time[i][2];
                putdown.push((Reverse(t + time[i][3]), i));
            } else if n > 0 && !left.is_empty() {
                let i = left.pop().unwrap().1;
                t += time[i][0];
                pickup.push((Reverse(t + time[i][1]), i));
                n -= 1;
            } else {
                let mut t1 = i32::MAX;
                if let Some(&(Reverse(ti), _)) = putdown.peek() {
                    t1 = t1.min(ti);
                }
                if let Some(&(Reverse(ti), _)) = pickup.peek() {
                    t1 = t1.min(ti);
                }
                if t1 != i32::MAX {
                    t = t1.max(t);
                }
            }
        }
        t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let time = [[1, 1, 2, 1], [1, 1, 3, 1], [1, 1, 4, 1]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(6, Solution::find_crossing_time(1, 3, time));
    }

    #[test]
    fn case2() {
        let time = [[1, 9, 1, 8], [10, 10, 10, 10]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(50, Solution::find_crossing_time(3, 2, time));
    }
}
