pub struct Solution;

impl Solution {
    pub fn find_continuous_sequence(target: i32) -> Vec<Vec<i32>> {
        let k_max = (((8.0 * target as f64 + 1.5).sqrt() - 3.0) * 0.5) as i32;
        (1..=k_max)
            .rev()
            .filter_map(|k| {
                let a = 2 * target - k * k - k;
                let b = 2 * (k + 1);
                if a % b == 0 {
                    let a = a / b;
                    Some((a..=a + k).collect())
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let result = Solution::find_continuous_sequence(9);
        let expected = vec![vec![2, 3, 4], vec![4, 5]];
        assert_eq!(expected, result);
    }

    #[test]
    fn case2() {
        let result = Solution::find_continuous_sequence(15);
        let expected = vec![vec![1, 2, 3, 4, 5], vec![4, 5, 6], vec![7, 8]];
        assert_eq!(expected, result);
    }
}
