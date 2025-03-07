pub struct Solution;

use std::collections::HashMap;

struct BinaryIndexedTree {
    vals: Vec<i32>,
}

impl BinaryIndexedTree {
    fn new(n: usize) -> Self {
        Self {
            vals: vec![0; n + 1],
        }
    }

    fn add(&mut self, mut v: i32) {
        while v < self.vals.len() as i32 {
            self.vals[v as usize] += 1;
            v += v & -v;
        }
    }

    fn query(&self, mut v: i32) -> i32 {
        let mut result = 0;
        while v > 0 {
            result += self.vals[v as usize];
            v &= v - 1;
        }
        result
    }
}

impl Solution {
    pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut sorted = nums.clone();
        sorted.sort_unstable();
        let mut m = HashMap::new();
        for (i, x) in sorted.into_iter().enumerate() {
            m.insert(x, i as i32 + 1);
        }
        let (mut a1, mut a2) = (Vec::with_capacity(n), Vec::with_capacity(n));
        a1.push(nums[0]);
        a2.push(nums[1]);
        let (mut t1, mut t2) = (BinaryIndexedTree::new(n), BinaryIndexedTree::new(n));
        t1.add(m[&nums[0]]);
        t2.add(m[&nums[1]]);
        for x in &nums[2..] {
            let i = m[x];
            let c1 = a1.len() as i32 - t1.query(i);
            let c2 = a2.len() as i32 - t2.query(i);
            if c1 > c2 || (c1 == c2 && a1.len() <= a2.len()) {
                a1.push(*x);
                t1.add(i);
            } else {
                a2.push(*x);
                t2.add(i);
            }
        }
        a1.extend(a2);
        a1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(vec![2, 3, 1, 3], Solution::result_array(vec![2, 1, 3, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![5, 3, 1, 2, 14],
            Solution::result_array(vec![5, 14, 3, 1, 2])
        );
    }

    #[test]
    fn case3() {
        assert_eq!(vec![3, 3, 3, 3], Solution::result_array(vec![3, 3, 3, 3]));
    }
}
