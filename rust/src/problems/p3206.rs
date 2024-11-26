pub struct Solution;

impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>) -> i32 {
        let n = colors.len();
        (0..n)
            .filter(|&i| colors[i] != colors[(i + 1) % n] && colors[i] != colors[(i + n - 1) % n])
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(0, Solution::number_of_alternating_groups(vec![1, 1, 1]));
    }

    #[test]
    fn case2() {
        assert_eq!(
            3,
            Solution::number_of_alternating_groups(vec![0, 1, 0, 0, 1])
        );
    }
}
