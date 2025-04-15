pub struct Solution;

struct Bit {
    tree: Vec<i32>,
}

impl Bit {
    fn new(n: usize) -> Self {
        Self {
            tree: vec![0; n + 1],
        }
    }

    fn query(&self, mut i: i32) -> i32 {
        let mut ans = 0;
        while i > 0 {
            ans += self.tree[i as usize];
            i &= i - 1;
        }
        ans
    }

    fn update(&mut self, mut i: i32, delta: i32) {
        while i < self.tree.len() as i32 {
            self.tree[i as usize] += delta;
            i += i & (-i);
        }
    }
}

impl Solution {
    pub fn good_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let n = nums1.len();
        let mut pos = vec![0; n];
        for (i, x) in nums1.into_iter().enumerate() {
            pos[x as usize] = i as i32;
        }
        let mut ans = 0;
        let mut t = Bit::new(n + 1);
        for (i, x) in nums2.into_iter().enumerate() {
            let y = pos[x as usize];
            let less = t.query(y) as i64;
            ans += less * (n as i64 - 1 - y as i64 - (i as i64 - less));
            t.update(y + 1, 1);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            1,
            Solution::good_triplets(vec![2, 0, 1, 3], vec![0, 1, 2, 3])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            4,
            Solution::good_triplets(vec![4, 0, 1, 3, 2], vec![4, 1, 0, 2, 3])
        );
    }
}
