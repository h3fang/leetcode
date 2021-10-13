pub struct Solution;

impl Solution {
    pub fn interval_intersection(
        first_list: Vec<Vec<i32>>,
        second_list: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let mut i = 0;
        let mut j = 0;
        let mut result = Vec::new();
        while i < first_list.len() && j < second_list.len() {
            let seg1 = &first_list[i];
            let seg2 = &second_list[j];
            if seg1[1] < seg2[0] {
                i += 1;
            } else if seg2[1] < seg1[0] {
                j += 1;
            } else {
                let a = seg1[0].max(seg2[0]);
                if seg1[1] < seg2[1] {
                    result.push(vec![a, seg1[1]]);
                    i += 1;
                } else {
                    result.push(vec![a, seg2[1]]);
                    j += 1;
                }
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
        let first_list = [[0, 2], [5, 10], [13, 23], [24, 25]];
        let second_list = [[1, 5], [8, 12], [15, 24], [25, 26]];
        let expected = [[1, 2], [5, 5], [8, 10], [15, 23], [24, 24], [25, 25]];
        let first_list = first_list.iter().map(|v| v.to_vec()).collect::<Vec<_>>();
        let second_list = second_list.iter().map(|v| v.to_vec()).collect::<Vec<_>>();
        let expected = expected.iter().map(|v| v.to_vec()).collect::<Vec<_>>();
        assert_eq!(
            expected,
            Solution::interval_intersection(first_list, second_list)
        );
    }

    #[test]
    fn case2() {
        let first_list = [[1, 3], [5, 9]];
        let second_list: [[i32; 2]; 0] = [];
        let expected: [[i32; 2]; 0] = [];
        let first_list = first_list.iter().map(|v| v.to_vec()).collect::<Vec<_>>();
        let second_list = second_list.iter().map(|v| v.to_vec()).collect::<Vec<_>>();
        let expected = expected.iter().map(|v| v.to_vec()).collect::<Vec<_>>();
        assert_eq!(
            expected,
            Solution::interval_intersection(first_list, second_list)
        );
    }
}
