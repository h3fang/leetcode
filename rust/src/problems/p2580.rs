pub struct Solution;

impl Solution {
    pub fn count_ways(mut ranges: Vec<Vec<i32>>) -> i32 {
        ranges.sort_unstable();
        let n = ranges.len();
        let mut result = 1;
        let mut i = 0;
        while i < n {
            let mut r = ranges[i][1];
            let mut j = i + 1;
            while j < n && ranges[j][0] <= r {
                r = r.max(ranges[j][1]);
                j += 1;
            }
            result = (result * 2) % 10_0000_0007;
            i = j;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let ranges = [[6, 10], [5, 15]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(2, Solution::count_ways(ranges));
    }

    #[test]
    fn case2() {
        let ranges = [[1, 3], [10, 20], [2, 5], [4, 8]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(4, Solution::count_ways(ranges));
    }
}
