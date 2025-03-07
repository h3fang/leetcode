pub struct Solution;

impl Solution {
    pub fn maximum_value_sum(nums: Vec<i32>, k: i32, _edges: Vec<Vec<i32>>) -> i64 {
        let k = k as i64;
        let (mut f0, mut f1) = (0, i64::MIN);
        for x in nums {
            let x = x as i64;
            (f0, f1) = ((f0 + x).max(f1 + (x ^ k)), (f1 + x).max(f0 + (x ^ k)));
        }
        f0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let edges = [[0, 1], [0, 2]].iter().map(|e| e.to_vec()).collect();
        assert_eq!(6, Solution::maximum_value_sum(vec![1, 2, 1], 3, edges));
    }

    #[test]
    fn case2() {
        let edges = [[0, 1]].iter().map(|e| e.to_vec()).collect();
        assert_eq!(9, Solution::maximum_value_sum(vec![2, 3], 7, edges));
    }

    #[test]
    fn case3() {
        let edges = [[0, 1], [0, 2], [0, 3], [0, 4], [0, 5]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(
            42,
            Solution::maximum_value_sum(vec![7, 7, 7, 7, 7, 7], 3, edges)
        );
    }
}
