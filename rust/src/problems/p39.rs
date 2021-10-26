pub struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn bt(candidates: &[i32], target: i32, curr: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
            match target.cmp(&0) {
                std::cmp::Ordering::Less => {}
                std::cmp::Ordering::Equal => result.push(curr.clone()),
                std::cmp::Ordering::Greater => {
                    for (i, c) in candidates.iter().enumerate() {
                        curr.push(*c);
                        bt(&candidates[i..], target - c, curr, result);
                        curr.pop();
                    }
                }
            }
        }

        let mut result = Vec::new();
        let mut curr = Vec::new();
        bt(&candidates, target, &mut curr, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let candidates = vec![2, 3, 6, 7];
        let target = 7;
        let mut expected = vec![vec![2, 2, 3], vec![7]];
        expected.sort_unstable();
        let mut result = Solution::combination_sum(candidates, target);
        result.sort_unstable();
        assert_eq!(expected, result);
    }

    #[test]
    fn case2() {
        let candidates = vec![2, 3, 5];
        let target = 8;
        let mut expected = vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]];
        expected.sort_unstable();
        let mut result = Solution::combination_sum(candidates, target);
        result.sort_unstable();
        assert_eq!(expected, result);
    }

    #[test]
    fn case3() {
        let candidates = vec![2];
        let target = 1;
        let mut expected: Vec<Vec<i32>> = vec![];
        expected.sort_unstable();
        let mut result = Solution::combination_sum(candidates, target);
        result.sort_unstable();
        assert_eq!(expected, result);
    }

    #[test]
    fn case4() {
        let candidates = vec![1];
        let target = 1;
        let mut expected = vec![vec![1]];
        expected.sort_unstable();
        let mut result = Solution::combination_sum(candidates, target);
        result.sort_unstable();
        assert_eq!(expected, result);
    }

    #[test]
    fn case5() {
        let candidates = vec![1];
        let target = 2;
        let mut expected = vec![vec![1, 1]];
        expected.sort_unstable();
        let mut result = Solution::combination_sum(candidates, target);
        result.sort_unstable();
        assert_eq!(expected, result);
    }
}
