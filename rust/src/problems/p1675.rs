use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn minimum_deviation(nums: Vec<i32>) -> i32 {
        let mut pq = BinaryHeap::new();
        let mut min = i32::MAX;
        for n in nums {
            let n = if n & 1 == 0 { n } else { n * 2 };
            min = min.min(n);
            pq.push(n);
        }
        let mut result = pq.peek().unwrap() - min;
        while !pq.is_empty() && *pq.peek().unwrap() & 1 == 0 {
            let odd = pq.peek().unwrap() / 2;
            pq.pop();
            pq.push(odd);
            min = min.min(odd);
            result = result.min(pq.peek().unwrap() - min);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::minimum_deviation(vec![1, 2, 3, 4]));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::minimum_deviation(vec![4, 1, 5, 20, 3]));
    }

    #[test]
    fn case3() {
        assert_eq!(3, Solution::minimum_deviation(vec![2, 10, 8]));
    }
}
