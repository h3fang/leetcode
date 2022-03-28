pub struct Solution;

impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut result = Vec::with_capacity(nums.len());
        let mut right = Vec::new();
        for &n in &nums {
            match n.cmp(&pivot) {
                std::cmp::Ordering::Less => result.push(n),
                std::cmp::Ordering::Equal => {}
                std::cmp::Ordering::Greater => right.push(n),
            }
        }
        result.extend(vec![pivot; nums.len() - result.len() - right.len()]);
        result.extend(right);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![9, 5, 3, 10, 10, 12, 14],
            Solution::pivot_array(vec![9, 12, 5, 10, 14, 3, 10], 10)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![-14, -11, -7, 19, 15, 12, 8],
            Solution::pivot_array(vec![19, 15, 12, -14, 8, -7, -11], -7)
        );
    }
}
