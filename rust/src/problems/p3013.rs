pub struct Solution;

use std::collections::{BinaryHeap, HashMap};

struct LazyHeap {
    heap: BinaryHeap<i32>,
    to_remove: HashMap<i32, i32>,
    size: usize,
    sum: i64,
}

impl LazyHeap {
    fn with_capacity(cap: usize) -> Self {
        Self {
            heap: BinaryHeap::with_capacity(cap),
            to_remove: HashMap::with_capacity(cap),
            size: 0,
            sum: 0,
        }
    }

    fn push(&mut self, x: i32) {
        self.heap.push(x);
        self.size += 1;
        self.sum += x as i64;
    }

    fn pop(&mut self) -> Option<i32> {
        self.lazy_remove();
        let v = self.heap.pop();
        if let Some(x) = v {
            self.size -= 1;
            self.sum -= x as i64;
        }
        v
    }

    fn peek(&mut self) -> Option<&i32> {
        self.lazy_remove();
        self.heap.peek()
    }

    fn remove(&mut self, x: i32) {
        self.size -= 1;
        self.sum -= x as i64;
        *self.to_remove.entry(x).or_default() += 1;
    }

    fn lazy_remove(&mut self) {
        while let Some(&x) = self.heap.peek() {
            if let Some(v) = self.to_remove.get_mut(&x)
                && *v > 0
            {
                *v -= 1;
                self.heap.pop();
            } else {
                break;
            }
        }
    }
}

impl Solution {
    pub fn minimum_cost(nums: Vec<i32>, k: i32, dist: i32) -> i64 {
        let (k, dist) = (k as usize - 1, dist as usize);
        let (mut l, mut r) = (
            LazyHeap::with_capacity(k),
            LazyHeap::with_capacity(dist + 1),
        );

        for &x in &nums[1..dist + 2] {
            l.push(x);
        }

        while l.size > k {
            let x = l.pop().unwrap();
            r.push(-x);
        }

        let mut ans = l.sum;

        for (i, &x) in nums.iter().enumerate().skip(dist + 2) {
            if x < *l.peek().unwrap() {
                l.push(x);
                if l.size > k {
                    r.push(-l.pop().unwrap());
                }
            } else {
                r.push(-x);
            }

            let left = nums[i - dist - 1];
            if left <= *l.peek().unwrap() {
                l.remove(left);
                if l.size < k {
                    l.push(-r.pop().unwrap());
                }
            } else {
                r.remove(-left);
            }

            ans = ans.min(l.sum);
        }

        ans + nums[0] as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::minimum_cost(vec![1, 3, 2, 6, 4, 2], 3, 3));
    }

    #[test]
    fn case2() {
        assert_eq!(15, Solution::minimum_cost(vec![10, 1, 2, 2, 2, 1], 4, 3));
    }

    #[test]
    fn case3() {
        assert_eq!(36, Solution::minimum_cost(vec![10, 8, 18, 9], 3, 1));
    }

    #[test]
    fn case4() {
        assert_eq!(9, Solution::minimum_cost(vec![1, 5, 3, 6], 3, 2));
    }

    #[test]
    fn case5() {
        assert_eq!(9, Solution::minimum_cost(vec![1, 5, 3, 7], 3, 1));
    }

    #[test]
    fn case6() {
        assert_eq!(
            163,
            Solution::minimum_cost(
                vec![
                    6, 40, 41, 11, 50, 15, 47, 48, 50, 12, 16, 30, 38, 45, 13, 34, 26, 25, 32, 28
                ],
                9,
                13
            )
        );
    }
}
