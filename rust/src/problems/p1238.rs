pub struct Solution;

impl Solution {
    pub fn circular_permutation(n: i32, start: i32) -> Vec<i32> {
        let mut result = vec![0; 1 << n];
        for (i, e) in result.iter_mut().enumerate() {
            *e = ((i as i32 >> 1) ^ i as i32) ^ start;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let result = Solution::circular_permutation(2, 3);
        let expected = [vec![3, 2, 0, 1], vec![3, 1, 0, 2]];
        assert!(expected.contains(&result));
    }

    #[test]
    fn case2() {
        let result = Solution::circular_permutation(3, 2);
        let expected = [vec![2, 6, 7, 5, 4, 0, 1, 3], vec![2, 3, 1, 0, 4, 5, 7, 6]];
        assert!(expected.contains(&result));
    }
}
