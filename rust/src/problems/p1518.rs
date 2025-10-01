pub struct Solution;

impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        num_bottles + (num_bottles - 1) / (num_exchange - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(13, Solution::num_water_bottles(9, 3));
    }

    #[test]
    fn case2() {
        assert_eq!(19, Solution::num_water_bottles(15, 4));
    }

    #[test]
    fn case3() {
        assert_eq!(6, Solution::num_water_bottles(5, 5));
    }
}
