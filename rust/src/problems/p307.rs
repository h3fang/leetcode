pub struct NumArray {
    nums: Vec<i32>,
    t: Vec<i32>,
}

impl NumArray {
    pub fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        let mut t = vec![0; n + 1];
        for (i, x) in nums.iter().enumerate() {
            t[i + 1] += x;
            let j = i as i32 + 1;
            let j = (j + (j & -j)) as usize;
            if j <= n {
                t[j] += t[i + 1];
            }
        }
        Self { nums, t }
    }

    pub fn update(&mut self, index: i32, val: i32) {
        let d = val - self.nums[index as usize];
        self.nums[index as usize] = val;
        let mut i = index + 1;
        while i < self.t.len() as i32 {
            self.t[i as usize] += d;
            i += i & -i;
        }
    }

    fn query(&self, mut i: i32) -> i32 {
        let mut sum = 0;
        while i > 0 {
            sum += self.t[i as usize];
            i -= i & -i;
        }
        sum
    }

    pub fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.query(right + 1) - self.query(left)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut arr = NumArray::new(vec![1, 3, 5]);
        assert_eq!(9, arr.sum_range(0, 2));
        arr.update(1, 2);
        assert_eq!(8, arr.sum_range(0, 2));
    }

    #[test]
    fn case2() {
        let mut arr = NumArray::new(vec![-28, -39, 53, 65, 11, -56, -65, -39, -43, 97]);
        assert_eq!(-121, arr.sum_range(5, 6));
        arr.update(9, 27);
        assert_eq!(118, arr.sum_range(2, 3));
    }
}
