pub struct Solution;

impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        arr.sort_unstable();
        let mut min = i32::MAX;
        let mut result = vec![];
        for w in arr.windows(2) {
            match (w[1] - w[0]).cmp(&min) {
                std::cmp::Ordering::Less => {
                    min = w[1] - w[0];
                    result.clear();
                    result.push(w.to_vec());
                }
                std::cmp::Ordering::Equal => result.push(w.to_vec()),
                std::cmp::Ordering::Greater => {}
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
        let expected = [[1, 2], [2, 3], [3, 4]];
        let expected = expected.iter().map(|e| e.to_vec()).collect::<Vec<_>>();
        assert_eq!(expected, Solution::minimum_abs_difference(vec![4, 2, 1, 3]));
    }
}
