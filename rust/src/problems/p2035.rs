pub struct Solution;

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        // assert!(n % 2 == 0);
        let n = n / 2;
        let left = &nums[..n];
        let right = &nums[n..];
        let mut diffs = vec![vec![]; n + 1];
        for comb in 0usize..(1 << n) {
            let k = comb.count_ones();
            let mut d = 0;
            (0..n).for_each(|i| {
                if comb & (1 << i) > 0 {
                    d += left[i];
                } else {
                    d -= left[i];
                }
            });
            diffs[k as usize].push(d);
        }

        diffs.iter_mut().for_each(|ds| ds.sort_unstable());

        let mut result = i32::MAX;

        for comb in 0usize..(1 << n) {
            let k = comb.count_ones();
            let mut d = 0;
            (0..n).for_each(|i| {
                if comb & (1 << i) > 0 {
                    d += right[i];
                } else {
                    d -= right[i];
                }
            });
            let diffs = &diffs[n - k as usize];
            if let Err(idx) = diffs.binary_search(&-d) {
                if idx == 0 {
                    result = result.min((d + diffs[idx]).abs());
                } else {
                    result = result.min((d + diffs[idx - 1]).abs());
                }
            } else {
                result = 0;
                break;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![3, 9, 7, 3];
        assert_eq!(2, Solution::minimum_difference(nums));
    }

    #[test]
    fn case2() {
        let nums = vec![2, -1, 0, 4, -2, -9];
        assert_eq!(0, Solution::minimum_difference(nums));
    }

    #[test]
    fn case3() {
        let nums = vec![-10, 10];
        assert_eq!(20, Solution::minimum_difference(nums));
    }
}
