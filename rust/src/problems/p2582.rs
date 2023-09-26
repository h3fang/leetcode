pub struct Solution;

impl Solution {
    pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
        let t = time % (2 * (n - 1));
        if t < n {
            1 + t
        } else {
            n - (t - n + 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::pass_the_pillow(4, 5));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::pass_the_pillow(3, 2));
    }
}
