pub struct Solution;

const MOD: i64 = 10_0000_0007;

impl Solution {
    pub fn sum_distance(nums: Vec<i32>, s: String, d: i32) -> i32 {
        let n = nums.len();
        let d = d as i64;
        let mut nums = nums
            .into_iter()
            .zip(s.as_bytes())
            .map(|(x, &s)| match s {
                b'L' => x as i64 - d,
                _ => x as i64 + d,
            })
            .collect::<Vec<_>>();
        nums.sort_unstable();
        nums.windows(2).enumerate().fold(0, |r, (i, w)| {
            let c = ((i + 1) * (n - i - 1)) as i64 % MOD;
            let c = c * (w[1] - w[0]) % MOD;
            (r + c) % MOD
        }) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            8,
            Solution::sum_distance(vec![-2, 0, 2], "RLL".to_string(), 3)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(5, Solution::sum_distance(vec![1, 0], "RL".to_string(), 2));
    }
}
