pub struct Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        fn recursive(
            k: i32,
            start: i32,
            end: i32,
            used: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            if k == 0 {
                result.push(used.to_vec());
                return;
            }
            for i in start..=end - k + 1 {
                used.push(i);
                recursive(k - 1, i + 1, end, used, result);
                used.pop();
            }
        }
        let mut used = Vec::new();
        let mut result = Vec::new();
        recursive(k, 1, n, &mut used, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let expected = [[2, 4], [3, 4], [2, 3], [1, 2], [1, 3], [1, 4]];
        let mut expected = expected
            .iter()
            .map(|r| {
                let mut r = r.to_vec();
                r.sort_unstable();
                r
            })
            .collect::<Vec<_>>();
        expected.sort();
        let result = Solution::combine(4, 2);
        let mut result = result
            .iter()
            .map(|r| {
                let mut r = r.to_vec();
                r.sort_unstable();
                r
            })
            .collect::<Vec<_>>();
        result.sort();
        assert_eq!(expected, result);
    }

    #[test]
    fn case2() {
        let expected = [[1]];
        let mut expected = expected.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        expected.sort();
        let result = Solution::combine(1, 1);
        let mut result = result
            .iter()
            .map(|r| {
                let mut r = r.to_vec();
                r.sort_unstable();
                r
            })
            .collect::<Vec<_>>();
        result.sort();
        assert_eq!(expected, result);
    }
}
