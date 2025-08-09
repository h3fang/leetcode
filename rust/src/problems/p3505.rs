pub struct Solution;

use std::collections::{BinaryHeap, HashMap};

struct LazyHeap {
    heap: BinaryHeap<i64>,
    lazy: HashMap<i64, u32>,
    size: i64,
    sum: i64,
}

impl LazyHeap {
    fn with_capacity(n: usize) -> Self {
        Self {
            heap: BinaryHeap::with_capacity(n),
            lazy: Default::default(),
            size: 0,
            sum: 0,
        }
    }

    fn lazy_remove(&mut self) {
        while let Some(&x) = self.heap.peek() {
            if let Some(y) = self.lazy.get_mut(&x) {
                if *y > 0 {
                    *y -= 1;
                    self.heap.pop();
                } else {
                    return;
                }
            } else {
                return;
            }
        }
    }

    fn remove(&mut self, x: i64) {
        *self.lazy.entry(x).or_default() += 1;
        self.sum -= x;
        self.size -= 1;
    }

    fn peek(&mut self) -> i64 {
        self.lazy_remove();
        *self.heap.peek().unwrap()
    }

    fn push(&mut self, x: i64) {
        self.size += 1;
        self.sum += x;
        if let Some(c) = self.lazy.get_mut(&x)
            && *c > 0
        {
            *c -= 1;
            return;
        }
        self.heap.push(x);
    }

    fn pop(&mut self) -> i64 {
        self.lazy_remove();
        self.size -= 1;
        let v = self.heap.pop().unwrap();
        self.sum -= v;
        v
    }

    fn push_pop(&mut self, x: i64) -> i64 {
        if self.size > 0 && self.peek() > x {
            self.push(x);
            self.pop()
        } else {
            x
        }
    }
}

fn min_op_for_sliding_window(nums: &[i32], w: usize) -> Vec<i64> {
    let mut ans = vec![0; nums.len() - w + 1];
    let mut left = LazyHeap::with_capacity(w);
    let mut right = LazyHeap::with_capacity(w);
    for (i, &x) in nums.iter().enumerate() {
        let x = x as i64;
        if left.size == right.size {
            let y = right.push_pop(-x);
            left.push(-y);
        } else {
            let y = left.push_pop(x);
            right.push(-y);
        }

        if i + 1 < w {
            continue;
        }
        let l = i + 1 - w;

        let v = left.peek();
        let s = v * left.size - left.sum - right.sum - v * right.size;
        ans[l] = s;

        let v = nums[l] as i64;
        if v <= left.peek() {
            left.remove(v);
            if left.size < right.size {
                left.push(-right.pop());
            }
        } else {
            right.remove(-v);
            if left.size > right.size + 1 {
                right.push(-left.pop());
            }
        }
    }
    ans
}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32, k: i32) -> i64 {
        let (n, x, k) = (nums.len(), x as usize, k as usize);
        let w = min_op_for_sliding_window(&nums, x);
        let mut f = vec![vec![0; n + 1]; k + 1];
        for i in 1..=k {
            f[i][i * x - 1] = i64::MAX;
            for j in i * x..=(n - (k - i) * x) {
                f[i][j] = f[i][j - 1].min(f[i - 1][j - x] + w[j - x]);
            }
        }
        f[k][n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            8,
            Solution::min_operations(vec![5, -2, 1, 3, 7, 3, 6, 4, -1], 3, 2)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::min_operations(vec![9, -2, -2, -2, 1, 5], 2, 2));
    }
}
