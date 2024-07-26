pub struct Solution;

impl Solution {
    pub fn find_value_of_partition(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        nums.windows(2).map(|w| w[1] - w[0]).min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::find_value_of_partition(vec![1, 3, 2, 4]));
    }

    #[test]
    fn case2() {
        assert_eq!(9, Solution::find_value_of_partition(vec![100, 1, 10]));
    }
}
