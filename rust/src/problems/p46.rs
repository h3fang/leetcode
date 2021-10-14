pub struct Solution;

impl Solution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn bt(nums: &mut Vec<i32>, i: usize, result: &mut Vec<Vec<i32>>) {
            if i == nums.len() {
                result.push(nums.clone());
                return;
            }

            for j in i..nums.len() {
                nums.swap(i, j);
                bt(nums, i + 1, result);
                nums.swap(i, j);
            }
        }

        let mut result = Vec::new();
        bt(&mut nums, 0, &mut result);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![1, 2, 3];
        let expected = [
            [1, 2, 3],
            [1, 3, 2],
            [2, 1, 3],
            [2, 3, 1],
            [3, 1, 2],
            [3, 2, 1],
        ];
        let mut expected = expected.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        expected.sort_unstable();
        let mut result = Solution::permute(nums);
        result.sort_unstable();
        assert_eq!(expected, result);
    }
}
