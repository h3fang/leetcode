pub struct Solution;

impl Solution {
    pub fn maximum_happiness_sum(mut happiness: Vec<i32>, k: i32) -> i64 {
        happiness.sort_unstable();
        happiness
            .into_iter()
            .rev()
            .take(k as usize)
            .enumerate()
            .map(|(i, h)| (h - i as i32).max(0) as i64)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::maximum_happiness_sum(vec![1, 2, 3], 2));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::maximum_happiness_sum(vec![1, 1, 1, 1], 2));
    }

    #[test]
    fn case3() {
        assert_eq!(5, Solution::maximum_happiness_sum(vec![2, 3, 4, 5], 1));
    }
}
