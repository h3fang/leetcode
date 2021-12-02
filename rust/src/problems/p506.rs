pub struct Solution;

impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut s = score.into_iter().enumerate().collect::<Vec<_>>();
        s.sort_unstable_by_key(|e| e.1);
        let n = s.len();
        let mut result = vec!["".to_string(); n];
        for (i, &(k, _)) in s.iter().enumerate() {
            if i + 1 == n {
                result[k] = "Gold Medal".to_string();
            } else if i + 2 == n {
                result[k] = "Silver Medal".to_string();
            } else if i + 3 == n {
                result[k] = "Bronze Medal".to_string();
            } else {
                result[k] = (n - i).to_string();
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let expected = ["Gold Medal", "Silver Medal", "Bronze Medal", "4", "5"];
        let expected = expected.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert_eq!(expected, Solution::find_relative_ranks(vec![5, 4, 3, 2, 1]));
    }
}
