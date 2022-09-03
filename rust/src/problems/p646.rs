pub struct Solution;

impl Solution {
    pub fn find_longest_chain(mut pairs: Vec<Vec<i32>>) -> i32 {
        pairs.sort_unstable_by_key(|p| (p[1], -(p[0] as i64)));
        let mut result = 1;
        let mut last = 0;
        for (i, p) in pairs.iter().enumerate() {
            if p[0] > pairs[last][1] {
                last = i;
                result += 1;
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
        let pairs = [[1, 2], [2, 3], [3, 4]]
            .iter()
            .map(|p| p.to_vec())
            .collect();
        assert_eq!(2, Solution::find_longest_chain(pairs));
    }
}
