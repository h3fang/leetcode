pub struct Solution;

impl Solution {
    pub fn sort_jumbled(mapping: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        fn map(mapping: &[i32], mut x: i32) -> i32 {
            if x == 0 {
                return mapping[0];
            }
            let mut digits = Vec::with_capacity(10);
            while x > 0 {
                let d = x % 10;
                digits.push(d);
                x /= 10;
            }
            let mut result = 0;
            for d in digits.into_iter().rev() {
                result = result * 10 + mapping[d as usize];
            }
            result
        }
        let mut nums = nums
            .into_iter()
            .map(|x| (map(&mapping, x), x))
            .collect::<Vec<_>>();
        nums.sort_by_key(|e| e.0);
        nums.into_iter().map(|e| e.1).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![338, 38, 991],
            Solution::sort_jumbled(vec![8, 9, 4, 0, 2, 1, 3, 5, 7, 6], vec![991, 338, 38])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![123, 456, 789],
            Solution::sort_jumbled(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], vec![789, 456, 123])
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
            Solution::sort_jumbled(
                vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
                vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
            )
        );
    }
}
