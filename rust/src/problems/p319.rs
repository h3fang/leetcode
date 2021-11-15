pub struct Solution;

impl Solution {
    pub fn bulb_switch(n: i32) -> i32 {
        (n as f64 + 0.5).sqrt() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::bulb_switch(3));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::bulb_switch(0));
    }

    #[test]
    fn case3() {
        assert_eq!(1, Solution::bulb_switch(1));
    }
}
