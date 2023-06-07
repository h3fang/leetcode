pub struct Solution;

impl Solution {
    pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
        let d = (a | b) ^ c;
        (d.count_ones() + (d & (a & b)).count_ones()) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::min_flips(2, 6, 5));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::min_flips(4, 2, 7));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::min_flips(1, 2, 3));
    }
}
