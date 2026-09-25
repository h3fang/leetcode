pub struct Solution;

impl Solution {
    pub fn number_of_employees_who_met_target(hours: Vec<i32>, target: i32) -> i32 {
        hours.into_iter().filter(|&h| h >= target).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            3,
            Solution::number_of_employees_who_met_target(vec![0, 1, 2, 3, 4], 2)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            0,
            Solution::number_of_employees_who_met_target(vec![5, 1, 4, 2, 2], 6)
        );
    }
}
