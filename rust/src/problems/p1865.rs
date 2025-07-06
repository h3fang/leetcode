use std::collections::HashMap;

pub struct FindSumPairs {
    f1: HashMap<i32, i32>,
    f2: HashMap<i32, i32>,
    nums2: Vec<i32>,
}

impl FindSumPairs {
    pub fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
        let mut f1 = HashMap::with_capacity(2000);
        for x in nums1 {
            *f1.entry(x).or_default() += 1;
        }
        let mut f2 = HashMap::with_capacity(10_0000);
        for &x in &nums2 {
            *f2.entry(x).or_default() += 1;
        }
        Self { f1, f2, nums2 }
    }

    pub fn add(&mut self, index: i32, val: i32) {
        let old = self.nums2[index as usize];
        *self.f2.get_mut(&old).unwrap() -= 1;
        self.nums2[index as usize] += val;
        *self.f2.entry(old + val).or_default() += 1;
    }

    pub fn count(&self, tot: i32) -> i32 {
        let mut ans = 0;
        for (x, f) in &self.f1 {
            ans += f * self.f2.get(&(tot - x)).unwrap_or(&0);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut fsp = FindSumPairs::new(vec![1, 1, 2, 2, 2, 3], vec![1, 4, 5, 2, 5, 4]);
        assert_eq!(8, fsp.count(7));
        fsp.add(3, 2);
        assert_eq!(2, fsp.count(8));
        assert_eq!(1, fsp.count(4));
        fsp.add(0, 1);
        fsp.add(1, 1);
        assert_eq!(11, fsp.count(7));
    }
}
