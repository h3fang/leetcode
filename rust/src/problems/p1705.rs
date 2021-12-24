use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution;

impl Solution {
    pub fn eaten_apples(apples: Vec<i32>, days: Vec<i32>) -> i32 {
        let mut q = BinaryHeap::new();
        let mut i = 0;
        let mut result = 0;
        while i < apples.len() || !q.is_empty() {
            if i < apples.len() && apples[i] > 0 {
                q.push((Reverse(i as i32 + days[i]), apples[i]));
            }
            while let Some(&(exp, n)) = q.peek() {
                if exp.0 > i as i32 {
                    result += 1;
                    if n == 1 {
                        q.pop();
                    } else {
                        q.peek_mut().unwrap().1 -= 1;
                    }
                    break;
                }
                q.pop();
            }
            i += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let apples = vec![1, 2, 3, 5, 2];
        let days = vec![3, 2, 1, 4, 2];
        assert_eq!(7, Solution::eaten_apples(apples, days));
    }

    #[test]
    fn case2() {
        let apples = vec![3, 0, 0, 0, 0, 2];
        let days = vec![3, 0, 0, 0, 0, 2];
        assert_eq!(5, Solution::eaten_apples(apples, days));
    }
}
