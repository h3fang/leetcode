pub struct Solution;

fn solve(k: usize, nums: &[i32], next: &[usize]) -> i32 {
    let n = nums.len();
    let mut last_max = 0;
    for r in (k - 1..n).step_by(k) {
        let l = r + 1 - k;
        let m = next[l];
        if m >= r {
            if nums[l] < last_max {
                return 0;
            }
            last_max = nums[r];
        } else {
            if next[m + 1] < r || nums[m + 1] < last_max || nums[r] > nums[l] {
                return 0;
            }
            last_max = nums[m];
        }
    }

    k as i32
}

impl Solution {
    pub fn sortable_integers(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut next = vec![0; n];
        next[n - 1] = n;
        let mut j = n;
        for (i, &x) in nums.iter().enumerate().rev().skip(1) {
            if x > nums[i + 1] {
                j = i;
            }
            next[i] = j;
        }

        let mut ans = 0;

        for k in 1..n.isqrt() + 1 {
            if n.is_multiple_of(k) {
                ans += solve(k, &nums, &next);
                if k * k < n {
                    ans += solve(n / k, &nums, &next);
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::sortable_integers(vec![3, 1, 2]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::sortable_integers(vec![7, 6, 5]));
    }

    #[test]
    fn case3() {
        assert_eq!(3, Solution::sortable_integers(vec![5, 8]));
    }
}
