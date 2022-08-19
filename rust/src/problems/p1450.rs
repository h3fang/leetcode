pub struct Solution;

impl Solution {
    pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        start_time
            .iter()
            .zip(&end_time)
            .filter(|(&s, &e)| (s..=e).contains(&query_time))
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::busy_student(vec![1, 2, 3], vec![3, 2, 7], 4));
    }
}
