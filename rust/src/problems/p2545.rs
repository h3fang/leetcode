pub struct Solution;

impl Solution {
    pub fn sort_the_students(mut score: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        score.sort_unstable_by_key(|r| -r[k as usize]);
        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let score = [[10, 6, 9, 1], [7, 5, 11, 2], [4, 8, 3, 15]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        let expected = [[7, 5, 11, 2], [10, 6, 9, 1], [4, 8, 3, 15]]
            .iter()
            .map(|r| r.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(expected, Solution::sort_the_students(score, 2));
    }

    #[test]
    fn case2() {
        let score = [[3, 4], [5, 6]].iter().map(|r| r.to_vec()).collect();
        let expected = [[5, 6], [3, 4]]
            .iter()
            .map(|r| r.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(expected, Solution::sort_the_students(score, 0));
    }
}
