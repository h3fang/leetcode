pub struct Solution;

impl Solution {
    pub fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32 {
        let n = arr.len();
        let mut f = vec![i32::MAX / 2; n];
        let (mut l, mut sum, mut min) = (0, 0, i32::MAX / 2);
        let mut ans = min;

        for (r, x) in arr.iter().enumerate() {
            sum += x;
            while sum > target {
                sum -= arr[l];
                l += 1;
            }
            if sum == target {
                let len = (r - l + 1) as i32;
                min = min.min(len);
                if l > 0 {
                    ans = ans.min(len + f[l - 1]);
                }
            }

            f[r] = min;
        }

        if ans == i32::MAX / 2 { -1 } else { ans }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::min_sum_of_lengths(vec![3, 2, 2, 4, 3], 3));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::min_sum_of_lengths(vec![7, 3, 4, 7], 7));
    }

    #[test]
    fn case3() {
        assert_eq!(
            -1,
            Solution::min_sum_of_lengths(vec![4, 3, 2, 6, 2, 3, 4], 6)
        );
    }

    #[test]
    fn case4() {
        assert_eq!(-1, Solution::min_sum_of_lengths(vec![5, 5, 4, 4, 5], 3));
    }

    #[test]
    fn case5() {
        assert_eq!(
            3,
            Solution::min_sum_of_lengths(vec![3, 1, 1, 1, 5, 1, 2, 1], 3)
        );
    }
}
