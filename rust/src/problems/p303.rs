pub struct NumArray {
    presum: Vec<i32>,
}

impl NumArray {
    pub fn new(nums: Vec<i32>) -> Self {
        let mut presum = vec![0; nums.len() + 1];
        for (i, x) in nums.into_iter().enumerate() {
            presum[i + 1] = presum[i] + x;
        }
        Self { presum }
    }

    pub fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.presum[right as usize + 1] - self.presum[left as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let na = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
        assert_eq!(1, na.sum_range(0, 2));
        assert_eq!(-1, na.sum_range(2, 5));
        assert_eq!(-3, na.sum_range(0, 5));
    }
}
