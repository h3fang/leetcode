pub struct Solution;

fn solve(a_start: &[i32], a_duration: &[i32], b_start: &[i32], b_duration: &[i32]) -> i32 {
    let a_min = a_start
        .iter()
        .zip(a_duration)
        .map(|(s, d)| s + d)
        .min()
        .unwrap();

    b_start
        .iter()
        .zip(b_duration)
        .map(|(&s, d)| a_min.max(s) + d)
        .min()
        .unwrap()
}

impl Solution {
    pub fn earliest_finish_time(
        land_start_time: Vec<i32>,
        land_duration: Vec<i32>,
        water_start_time: Vec<i32>,
        water_duration: Vec<i32>,
    ) -> i32 {
        let a = solve(
            &land_start_time,
            &land_duration,
            &water_start_time,
            &water_duration,
        );
        let b = solve(
            &water_start_time,
            &water_duration,
            &land_start_time,
            &land_duration,
        );
        a.min(b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            9,
            Solution::earliest_finish_time(vec![2, 8], vec![4, 1], vec![6], vec![3])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            14,
            Solution::earliest_finish_time(vec![5], vec![3], vec![1], vec![10])
        );
    }
}
