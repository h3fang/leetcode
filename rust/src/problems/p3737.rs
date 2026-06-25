pub struct Solution;

impl Solution {
    pub fn count_majority_subarrays(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let mut cnt = vec![0; n * 2 + 1];
        cnt[n] = 1;
        let (mut ans, mut s, mut f) = (0, n, 0);

        for x in nums {
            if x == target {
                f += cnt[s];
                s += 1;
            } else {
                s -= 1;
                f -= cnt[s];
            }
            ans += f;
            cnt[s] += 1;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::count_majority_subarrays(vec![1, 2, 2, 3], 2));
    }

    #[test]
    fn case2() {
        assert_eq!(10, Solution::count_majority_subarrays(vec![1, 1, 1, 1], 1));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::count_majority_subarrays(vec![1, 2, 3], 4));
    }
}
