pub struct Solution;

impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let max = *candies.iter().max().unwrap();
        candies
            .into_iter()
            .map(|c| c + extra_candies >= max)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![true, true, true, false, true],
            Solution::kids_with_candies(vec![2, 3, 5, 1, 3], 3)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![true, false, false, false, false],
            Solution::kids_with_candies(vec![4, 2, 1, 1, 2], 1)
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            vec![true, false, true],
            Solution::kids_with_candies(vec![12, 1, 12], 10)
        );
    }
}
