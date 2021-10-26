pub struct Solution;

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn bt(candidates: &[i32], target: i32, curr: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
            match target.cmp(&0) {
                std::cmp::Ordering::Less => {}
                std::cmp::Ordering::Equal => result.push(curr.clone()),
                std::cmp::Ordering::Greater => {
                    let mut last = -1;
                    for (i, c) in candidates.iter().enumerate() {
                        if *c == last {
                            continue;
                        }
                        last = *c;
                        curr.push(*c);
                        bt(&candidates[i + 1..], target - c, curr, result);
                        curr.pop();
                    }
                }
            }
        }

        candidates.sort_unstable();
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
        let candidates = vec![10, 1, 2, 7, 6, 1, 5];
        let target = 8;
        let mut expected = vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]];
        expected.sort_unstable();
        let mut result = Solution::combination_sum2(candidates, target);
        result.sort_unstable();
        assert_eq!(expected, result);
    }

    #[test]
    fn case2() {
        let candidates = vec![2, 5, 2, 1, 2];
        let target = 5;
        let mut expected = vec![vec![1, 2, 2], vec![5]];
        expected.sort_unstable();
        let mut result = Solution::combination_sum2(candidates, target);
        result.sort_unstable();
        assert_eq!(expected, result);
    }
}
