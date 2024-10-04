pub struct Solution;

impl Solution {
    pub fn nth_person_gets_nth_seat(n: i32) -> f64 {
        if n == 1 {
            1.0
        } else {
            0.5
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-5);
    }

    #[test]
    fn case1() {
        assert_close(1.0, Solution::nth_person_gets_nth_seat(1));
    }

    #[test]
    fn case2() {
        assert_close(0.5, Solution::nth_person_gets_nth_seat(2));
    }
}
