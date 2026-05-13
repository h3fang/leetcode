pub struct Solution;

impl Solution {
    pub fn min_moves(nums: Vec<i32>, limit: i32) -> i32 {
        let (n, limit) = (nums.len(), limit as usize);
        let mut d = vec![0; (limit + 1) * 2];
        for (i, &a) in nums.iter().enumerate().take(n / 2) {
            let b = nums[n - 1 - i];
            let (a, b) = (a.min(b) as usize, a.max(b) as usize);
            d[2] += 2;
            d[a + 1] -= 1;
            d[a + b] -= 1;
            d[a + b + 1] += 1;
            d[b + limit + 1] += 1;
        }
        let (mut ans, mut m) = (i32::MAX, 0);
        for d in d.into_iter().skip(2) {
            m += d;
            ans = ans.min(m);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::min_moves(vec![1, 2, 4, 3], 4));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::min_moves(vec![1, 2, 2, 1], 2));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::min_moves(vec![1, 2, 1, 2], 2));
    }
}
