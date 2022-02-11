pub struct Solution;

impl Solution {
    pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        nums.sort_unstable();
        nums.windows(k).map(|w| w[k - 1] - w[0]).min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(0, Solution::minimum_difference(vec![90], 1));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::minimum_difference(vec![9, 4, 1, 7], 2));
    }
}
