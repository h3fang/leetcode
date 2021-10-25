pub struct Solution;

impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn helper(nums: &[i32], i: usize, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
            result.push(current.clone());

            let mut j = i;
            while j < nums.len() {
                if j == i || nums[j] != nums[j - 1] {
                    current.push(nums[j]);
                    helper(nums, j + 1, current, result);
                    current.pop();
                }
                j += 1;
            }
        }

        nums.sort_unstable();
        let mut result = Vec::new();
        let mut current = Vec::new();
        helper(&nums, 0, &mut current, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let expected = vec![
            vec![],
            vec![1],
            vec![1, 2],
            vec![1, 2, 2],
            vec![2],
            vec![2, 2],
        ];
        assert_eq!(expected, Solution::subsets_with_dup(vec![1, 2, 2]));
    }
}
