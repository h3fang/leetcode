pub struct Solution;

impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn bt(nums: &[i32], curr: &mut Vec<i32>, used: &mut [bool], result: &mut Vec<Vec<i32>>) {
            let n = nums.len();
            if curr.len() == n {
                result.push(curr.clone());
            } else {
                for i in 0..n {
                    if used[i] || (i > 0 && nums[i] == nums[i - 1] && !used[i - 1]) {
                        continue;
                    } else {
                        used[i] = true;
                        curr.push(nums[i]);
                        bt(nums, curr, used, result);
                        curr.pop();
                        used[i] = false;
                    }
                }
            }
        }
        nums.sort_unstable();
        let mut used = vec![false; nums.len()];
        let mut curr = Vec::new();
        let mut result = Vec::new();
        bt(&nums, &mut curr, &mut used, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut result = Solution::permute_unique(vec![1, 1, 2]);
        result.sort_unstable();
        let mut expected = vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]];
        expected.sort_unstable();
        assert_eq!(expected, result);
    }

    #[test]
    fn case2() {
        let mut result = Solution::permute_unique(vec![1, 2, 3]);
        result.sort_unstable();
        let mut expected = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ];
        expected.sort_unstable();
        assert_eq!(expected, result);
    }

    #[test]
    fn case3() {
        let mut result = Solution::permute_unique(vec![2, 2, 1, 1]);
        result.sort_unstable();
        let mut expected = vec![
            vec![1, 1, 2, 2],
            vec![1, 2, 1, 2],
            vec![1, 2, 2, 1],
            vec![2, 1, 1, 2],
            vec![2, 1, 2, 1],
            vec![2, 2, 1, 1],
        ];
        expected.sort_unstable();
        assert_eq!(expected, result);
    }
}
