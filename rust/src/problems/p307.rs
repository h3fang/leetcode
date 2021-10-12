pub struct NumArray {
    n: usize,
    seg_tree: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    pub fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        let mut seg_tree = vec![0; 2 * nums.len()];
        seg_tree[n..(2 * n)].clone_from_slice(&nums);
        for i in (0..n).rev() {
            seg_tree[i] = seg_tree[2 * i] + seg_tree[2 * i + 1];
        }
        Self { n, seg_tree }
    }

    pub fn update(&mut self, index: i32, val: i32) {
        let mut i = index as usize + self.n;
        self.seg_tree[i] = val;
        while i > 0 {
            let (left, right) = if i % 2 == 0 { (i, i + 1) } else { (i - 1, i) };
            self.seg_tree[i / 2] = self.seg_tree[left] + self.seg_tree[right];
            i /= 2;
        }
    }

    pub fn sum_range(&self, left: i32, right: i32) -> i32 {
        let mut l = left as usize + self.n;
        let mut r = right as usize + self.n;
        let mut result = 0;
        while l <= r {
            if l % 2 == 1 {
                result += self.seg_tree[l];
                l += 1;
            }
            if r % 2 == 0 {
                result += self.seg_tree[r];
                r -= 1;
            }
            l /= 2;
            r /= 2;
        }
        result
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
}
