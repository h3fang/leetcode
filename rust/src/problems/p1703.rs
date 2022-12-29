pub struct Solution;

impl Solution {
    pub fn min_moves(nums: Vec<i32>, k: i32) -> i32 {
        let mut g = vec![];
        let mut presum = Vec::with_capacity(nums.len() + 1);
        presum.push(0);
        for (i, &n) in nums.iter().enumerate() {
            if n == 1 {
                let s = i - g.len();
                g.push(s);
                presum.push(presum.last().unwrap() + s);
            }
        }
        let k = k as usize;
        let mut result = usize::MAX;
        for i in 0..=g.len() - k {
            let mid = i + k / 2;
            result = result.min(
                (1 - k % 2) * g[mid] + (presum[i + k] - presum[mid + 1])
                    - (presum[mid] - presum[i]),
            );
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::min_moves(vec![1, 0, 0, 1, 0, 1], 2));
    }

    #[test]
    fn case2() {
        assert_eq!(5, Solution::min_moves(vec![1, 0, 0, 0, 0, 0, 1, 1], 3));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::min_moves(vec![1, 1, 0, 1], 2));
    }
}
