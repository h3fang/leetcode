pub struct Solution;

impl Solution {
    pub fn max_sum(nums: Vec<i32>, k: i32, m: i32) -> i32 {
        let (n, k, m) = (nums.len(), k as usize, m as usize);
        let mut s = Vec::with_capacity(n + 1);
        s.push(0);
        for &x in &nums {
            s.push(s.last().unwrap() + x);
        }
        let mut f = vec![0; n + 1];
        for i in 1..=k {
            let mut max = i32::MIN;
            let mut g = vec![i32::MIN / 2; n + 1];
            for j in (i * m)..=(n - (k - i) * m) {
                max = max.max(f[j - m] - s[j - m]);
                g[j] = g[j - 1].max(s[j] + max);
            }
            f = g;
        }
        f[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(13, Solution::max_sum(vec![1, 2, -1, 3, 3, 4], 2, 2));
    }

    #[test]
    fn case2() {
        assert_eq!(-10, Solution::max_sum(vec![-10, 3, -1, -2], 4, 1));
    }
}
