pub struct Solution;

impl Solution {
    pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
        let (mut result, mut max) = (0, values[0]);
        for (j, x) in values.into_iter().enumerate().skip(1) {
            result = result.max(x + max - j as i32);
            max = max.max(x + j as i32);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            11,
            Solution::max_score_sightseeing_pair(vec![8, 1, 5, 2, 6])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::max_score_sightseeing_pair(vec![1, 2]));
    }
}
