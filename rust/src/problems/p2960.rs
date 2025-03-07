pub struct Solution;

impl Solution {
    pub fn count_tested_devices(battery_percentages: Vec<i32>) -> i32 {
        battery_percentages
            .into_iter()
            .fold(0, |r, x| if x > r { r + 1 } else { r })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::count_tested_devices(vec![1, 1, 2, 1, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::count_tested_devices(vec![0, 1, 2]));
    }
}
