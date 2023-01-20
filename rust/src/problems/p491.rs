pub struct Solution;

impl Solution {
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn dfs(i: usize, last: i32, nums: &[i32], s: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
            if i == nums.len() {
                if s.len() >= 2 {
                    result.push(s.clone());
                }
                return;
            }
            if nums[i] >= last {
                s.push(nums[i]);
                dfs(i + 1, nums[i], nums, s, result);
                s.pop();
            }
            if nums[i] != last {
                dfs(i + 1, last, nums, s, result);
            }
        }
        let mut result = vec![];
        dfs(0, i32::MIN, &nums, &mut vec![], &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut expected = vec![
            vec![4, 6],
            vec![4, 6, 7],
            vec![4, 6, 7, 7],
            vec![4, 7],
            vec![4, 7, 7],
            vec![6, 7],
            vec![6, 7, 7],
            vec![7, 7],
        ];
        expected.sort_unstable();
        let mut result = Solution::find_subsequences(vec![4, 6, 7, 7]);
        result.sort_unstable();
        assert_eq!(expected, result);
    }

    #[test]
    fn case2() {
        let mut expected = vec![vec![4, 4]];
        expected.sort_unstable();
        let mut result = Solution::find_subsequences(vec![4, 4, 3, 2, 1]);
        result.sort_unstable();
        assert_eq!(expected, result);
    }
}
