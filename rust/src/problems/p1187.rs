pub struct Solution;

fn upper_bound(nums: &[i32], mut l: usize, target: i32) -> i32 {
    let mut r = nums.len();
    while l < r {
        let m = l + (r - l) / 2;
        if nums[m] > target {
            r = m;
        } else {
            l = m + 1;
        }
    }
    r as i32
}

impl Solution {
    pub fn make_array_increasing(arr1: Vec<i32>, mut arr2: Vec<i32>) -> i32 {
        arr2.sort_unstable();
        let mut a2 = Vec::with_capacity(arr2.len());
        let mut prev = i32::MAX;
        for a in arr2 {
            if a != prev {
                a2.push(a);
            }
            prev = a;
        }
        let m = arr1.len();
        let n = a2.len();
        let mut dp = vec![vec![i32::MAX; m.min(n) + 1]; m + 1];
        dp[0][0] = -1;
        for i in 1..=m {
            for j in 0..=i.min(n) {
                if arr1[i - 1] > dp[i - 1][j] {
                    dp[i][j] = arr1[i - 1];
                }
                if j > 0 && dp[i - 1][j - 1] != i32::MAX {
                    let ub = upper_bound(&a2, j - 1, dp[i - 1][j - 1]);
                    if ub < n as i32 {
                        dp[i][j] = dp[i][j].min(a2[ub as usize]);
                    }
                }
                if i == m && dp[i][j] != i32::MAX {
                    return j as i32;
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            1,
            Solution::make_array_increasing(vec![1, 5, 3, 6, 7], vec![1, 3, 2, 4])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            2,
            Solution::make_array_increasing(vec![1, 5, 3, 6, 7], vec![4, 3, 1])
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            -1,
            Solution::make_array_increasing(vec![1, 5, 3, 6, 7], vec![1, 6, 3, 3])
        );
    }
}
