pub struct Solution;

const OFFSET: i32 = 10000;

struct Bit {
    tree: Vec<i32>,
}

impl Bit {
    fn new(n: usize) -> Self {
        Self { tree: vec![0; n] }
    }

    fn query(&self, mut x: i32) -> i32 {
        x += 3 * OFFSET;
        let mut result = 0;
        while x > 0 {
            result += self.tree[x as usize];
            x -= x & (-x);
        }
        result
    }

    fn add(&mut self, mut x: i32) {
        x += 3 * OFFSET;
        while x < self.tree.len() as i32 {
            self.tree[x as usize] += 1;
            x += x & (-x);
        }
    }
}

impl Solution {
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, diff: i32) -> i64 {
        let mut result = 0;
        let mut t = Bit::new(OFFSET as usize * 6 + 1);
        for (a, b) in nums1.into_iter().zip(nums2) {
            result += t.query(a - b + diff) as i64;
            t.add(a - b);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums1 = vec![3, 2, 5];
        let nums2 = vec![2, 2, 1];
        let diff = 1;
        assert_eq!(3, Solution::number_of_pairs(nums1, nums2, diff));
    }

    #[test]
    fn case2() {
        let nums1 = vec![3, -1];
        let nums2 = vec![-2, 2];
        let diff = -1;
        assert_eq!(0, Solution::number_of_pairs(nums1, nums2, diff));
    }
}
