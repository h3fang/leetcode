pub struct Solution;

impl Solution {
    pub fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        if nums.chunks(3).all(|c| c[2] - c[0] <= k) {
            nums.chunks(3).map(|c| c.to_vec()).collect()
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let k = 2;
        let result = Solution::divide_array(vec![1, 3, 4, 8, 7, 9, 3, 5, 1], k);
        assert!(result.iter().all(|v| v.len() == 3
            && (v[0] - v[1]).abs() <= k
            && (v[1] - v[2]).abs() <= k
            && (v[0] - v[2]).abs() <= k))
    }

    #[test]
    fn case2() {
        assert!(Solution::divide_array(vec![1, 3, 3, 2, 7, 3], 3).is_empty());
    }
}
