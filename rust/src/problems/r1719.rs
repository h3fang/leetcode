pub struct Solution;

impl Solution {
    pub fn missing_two(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len() + 2;
        let s1 = (1 + n) * n / 2;
        let ab = s1 - nums.iter().map(|&e| e as usize).sum::<usize>();
        let s2 = n * (n + 1) * (2 * n + 1) / 6;
        let ab2 = s2 - nums.iter().map(|&e| e as usize * e as usize).sum::<usize>();
        let d = (8 * ab2 - 4 * ab * ab) as f64;
        let d = d.sqrt().round() as usize;
        let a = (d + 2 * ab) / 4;
        let b = ab - a;
        vec![a as i32, b as i32]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut result = Solution::missing_two(vec![1]);
        result.sort_unstable();
        assert_eq!(vec![2, 3], result);
    }

    #[test]
    fn case2() {
        let mut result = Solution::missing_two(vec![2, 3]);
        result.sort_unstable();
        assert_eq!(vec![1, 4], result);
    }
}
