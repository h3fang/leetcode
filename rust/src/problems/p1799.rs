pub struct Solution;

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

impl Solution {
    pub fn max_score(nums: Vec<i32>) -> i32 {
        let m = nums.len();
        let mut dp = vec![0; 1 << m];
        let mut gcd_tmp = vec![vec![0; m]; m];
        for i in 0..m {
            for j in i + 1..m {
                gcd_tmp[i][j] = gcd(nums[i], nums[j]);
            }
        }
        let all = 1usize << m;
        for s in 1..all {
            let t = s.count_ones() as i32;
            if t & 1 > 0 {
                continue;
            }
            for i in 0..m {
                if (s >> i) & 1 > 0 {
                    for j in i + 1..m {
                        if (s >> j) & 1 > 0 {
                            dp[s] = dp[s].max(dp[s ^ (1 << i) ^ (1 << j)] + t / 2 * gcd_tmp[i][j]);
                        }
                    }
                }
            }
        }
        dp[all - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::max_score(vec![1, 2]));
    }

    #[test]
    fn case2() {
        assert_eq!(11, Solution::max_score(vec![3, 4, 6, 8]));
    }

    #[test]
    fn case3() {
        assert_eq!(14, Solution::max_score(vec![1, 2, 3, 4, 5, 6]));
    }
}
