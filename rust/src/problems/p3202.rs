pub struct Solution;

impl Solution {
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut ans = 0;
        for m in 0..k {
            let mut f = vec![0; k];
            for &x in &nums {
                let x = (x % k as i32) as usize;
                f[x] = f[(m + k - x) % k] + 1;
            }
            ans = ans.max(f.into_iter().max().unwrap());
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::maximum_length(vec![1, 2, 3, 4, 5], 2));
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::maximum_length(vec![1, 4, 2, 3, 1, 4], 3));
    }
}
