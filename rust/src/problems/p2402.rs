pub struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn most_booked(n: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut count = vec![0; n];

        let mut empty = BinaryHeap::new();
        for i in 0..n {
            empty.push((Reverse(i), 0));
        }
        let mut occupied: BinaryHeap<(Reverse<i64>, Reverse<usize>)> = BinaryHeap::new();
        meetings.sort_unstable();
        for m in meetings {
            while !occupied.is_empty() && occupied.peek().unwrap().0 .0 <= m[0] as i64 {
                let (Reverse(t), i) = occupied.pop().unwrap();
                empty.push((i, t));
            }
            if let Some((i, t)) = empty.pop() {
                count[i.0] += 1;
                occupied.push((Reverse(t.max(m[0] as i64) + (m[1] - m[0]) as i64), i));
            } else {
                let (Reverse(t), i) = occupied.pop().unwrap();
                count[i.0] += 1;
                occupied.push((Reverse(t + (m[1] - m[0]) as i64), i));
            }
        }

        let mut max = 0;
        let mut result = 0;
        for (i, &c) in count.iter().enumerate() {
            if c > max {
                max = c;
                result = i as i32;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 2;
        let meetings = [[0, 10], [1, 5], [2, 7], [3, 4]]
            .iter()
            .map(|m| m.to_vec())
            .collect();
        assert_eq!(0, Solution::most_booked(n, meetings));
    }

    #[test]
    fn case2() {
        let n = 3;
        let meetings = [[1, 20], [2, 10], [3, 5], [4, 9], [6, 8]]
            .iter()
            .map(|m| m.to_vec())
            .collect();
        assert_eq!(1, Solution::most_booked(n, meetings));
    }

    #[test]
    fn case3() {
        let n = 4;
        let meetings = [[18, 19], [3, 12], [17, 19], [2, 13], [7, 10]]
            .iter()
            .map(|m| m.to_vec())
            .collect();
        assert_eq!(0, Solution::most_booked(n, meetings));
    }
}
