pub struct Solution;

impl Solution {
    pub fn edge_score(edges: Vec<i32>) -> i32 {
        let mut scores = vec![0; edges.len()];
        for (i, e) in edges.into_iter().enumerate() {
            scores[e as usize] += i as i64;
        }
        let mut max = 0;
        let mut result = 0;
        for (i, s) in scores.into_iter().enumerate() {
            if s > max {
                max = s;
                result = i as i32;
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
        assert_eq!(7, Solution::edge_score(vec![1, 0, 0, 0, 0, 7, 7, 5]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::edge_score(vec![2, 0, 0, 2]));
    }
}
