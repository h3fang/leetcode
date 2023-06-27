pub struct Solution;

impl Solution {
    pub fn maximum_sum(arr: Vec<i32>) -> i32 {
        arr.iter()
            .skip(1)
            .fold((arr[0], 0, arr[0]), |dp, e| {
                let dp0 = 0.max(dp.0) + e;
                let dp1 = (dp.0).max(dp.1 + e);
                (dp0, dp1, (dp.2).max(dp0.max(dp1)))
            })
            .2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::maximum_sum(vec![1, -2, 0, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::maximum_sum(vec![1, -2, -2, 3]));
    }
    #[test]
    fn case3() {
        assert_eq!(-1, Solution::maximum_sum(vec![-1, -1, -1, -1]));
    }
}
