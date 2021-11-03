pub struct Solution;

impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        match n {
            2 => 1,
            3 => 2,
            _ => {
                let a = n / 3;
                match n % 3 {
                    1 => 3i32.pow((a - 1) as u32) * 4,
                    2 => 3i32.pow(a as u32) * 2,
                    _ => 3i32.pow(a as u32),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::integer_break(2));
    }

    #[test]
    fn case2() {
        assert_eq!(36, Solution::integer_break(10));
    }

    #[test]
    fn case3() {
        assert_eq!(2, Solution::integer_break(3));
    }

    #[test]
    fn case4() {
        assert_eq!(59049, Solution::integer_break(30));
    }

    #[test]
    fn case5() {
        assert_eq!(18, Solution::integer_break(8));
    }
}
