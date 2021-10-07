pub struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 1 {
            return 1;
        }

        let mut prev_1 = 1;
        let mut prev_2 = 1;
        for _ in 2..=n {
            let t = prev_1 + prev_2;
            prev_2 = prev_1;
            prev_1 = t;
        }
        prev_1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::climb_stairs(2));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::climb_stairs(3));
    }
}
