pub struct Solution;

impl Solution {
    pub fn find_delayed_arrival_time(arrival_time: i32, delayed_time: i32) -> i32 {
        (arrival_time + delayed_time) % 24
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(20, Solution::find_delayed_arrival_time(15, 5));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::find_delayed_arrival_time(12, 12));
    }
}
