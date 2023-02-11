pub struct Solution;

impl Solution {
    pub fn fill_cups(mut a: Vec<i32>) -> i32 {
        a.sort_unstable();
        if a[0] + a[1] <= a[2] {
            a[2]
        } else {
            (a[0] + a[1] + a[2] + 1) / 2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::fill_cups(vec![1, 4, 2]));
    }

    #[test]
    fn case2() {
        assert_eq!(7, Solution::fill_cups(vec![5, 4, 4]));
    }
}
