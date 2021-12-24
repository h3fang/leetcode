pub struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_unstable();
        let mut result = vec![];
        let n = intervals.len();
        let mut i = 0;
        while i < n {
            let left = intervals[i][0];
            let mut right = intervals[i][1];
            while i + 1 < n && intervals[i + 1][0] <= right {
                right = right.max(intervals[i + 1][1]);
                i += 1;
            }
            result.push(vec![left, right]);
            i += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let intervals = [[1, 3], [2, 6], [8, 10], [15, 18]];
        let intervals = intervals.iter().map(|i| i.to_vec()).collect();
        let expected = [[1, 6], [8, 10], [15, 18]];
        let expected = expected.iter().map(|i| i.to_vec()).collect::<Vec<_>>();
        assert_eq!(expected, Solution::merge(intervals));
    }

    #[test]
    fn case2() {
        let intervals = [[1, 4], [4, 5]];
        let intervals = intervals.iter().map(|i| i.to_vec()).collect();
        let expected = [[1, 5]];
        let expected = expected.iter().map(|i| i.to_vec()).collect::<Vec<_>>();
        assert_eq!(expected, Solution::merge(intervals));
    }
}
