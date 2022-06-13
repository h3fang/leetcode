pub struct Solution;

impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut sorted = heights.clone();
        sorted.sort_unstable();
        sorted.iter().zip(&heights).filter(|(a, b)| a != b).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::height_checker(vec![1, 1, 4, 2, 1, 3]));
    }
}
