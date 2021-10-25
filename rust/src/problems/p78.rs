pub struct Solution;

impl Solution {
    pub fn subsets_bitmask(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut mask = 0;
        let mut result = Vec::new();
        let m = nums.len();
        let n = 1 << m;
        result.reserve(n);
        while mask < n {
            let curr = (0..m)
                .filter_map(|i| {
                    if mask & (1 << i) > 0 {
                        Some(nums[i])
                    } else {
                        None
                    }
                })
                .collect();
            result.push(curr);
            mask += 1;
        }
        result
    }

    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn bt(nums: &[i32], i: usize, curr: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
            result.push(curr.clone());
            for j in i..nums.len() {
                curr.push(nums[j]);
                bt(nums, j + 1, curr, result);
                curr.pop();
            }
        }
        let mut curr = Vec::new();
        let mut result = Vec::new();

        bt(&nums, 0, &mut curr, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![1, 2, 3];
        let mut expected = vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
        ];
        expected.sort_unstable();

        let mut result = Solution::subsets_bitmask(nums.clone());
        result.sort_unstable();
        assert_eq!(expected, result);

        let mut result = Solution::subsets(nums);
        result.sort_unstable();
        assert_eq!(expected, result);
    }

    #[test]
    fn case2() {
        let nums = vec![1];
        let mut expected = vec![vec![], vec![1]];
        expected.sort_unstable();

        let mut result = Solution::subsets_bitmask(nums.clone());
        result.sort_unstable();
        assert_eq!(expected, result);

        let mut result = Solution::subsets(nums);
        result.sort_unstable();
        assert_eq!(expected, result);
    }
}
