pub struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn find_maximized_capital(k: i32, mut w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let n = profits.len();
        let mut curr = 0;
        let mut q = BinaryHeap::new();
        let mut arr = vec![];
        for (c, p) in capital.into_iter().zip(profits) {
            arr.push((c, p));
        }
        arr.sort_unstable();
        for _ in 0..k {
            while curr < n && arr[curr].0 <= w {
                q.push(arr[curr].1);
                curr += 1;
            }
            if let Some(e) = q.pop() {
                w += e;
            } else {
                break;
            }
        }
        w
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            4,
            Solution::find_maximized_capital(2, 0, vec![1, 2, 3], vec![0, 1, 1])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            6,
            Solution::find_maximized_capital(3, 0, vec![1, 2, 3], vec![0, 1, 2])
        );
    }
}
