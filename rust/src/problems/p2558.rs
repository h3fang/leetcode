pub struct Solution;

use std::collections::BinaryHeap;

fn sqrt(x: i32) -> i32 {
    let x = x as i64;
    let (mut l, mut r, mut result) = (0, x, 0);
    while l <= r {
        let m = (r - l) / 2 + l;
        match (m * m).cmp(&x) {
            std::cmp::Ordering::Less => {
                result = m;
                l = m + 1
            }
            std::cmp::Ordering::Equal => return m as i32,
            std::cmp::Ordering::Greater => r = m - 1,
        }
    }
    result as i32
}

impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        let mut q = gifts.into_iter().collect::<BinaryHeap<_>>();
        for _ in 0..k {
            let x = q.pop().unwrap();
            let s = sqrt(x);
            q.push(s);
        }
        q.into_iter().map(|x| x as i64).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(29, Solution::pick_gifts(vec![25, 64, 9, 4, 100], 4));
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::pick_gifts(vec![1, 1, 1, 1], 4));
    }
}
