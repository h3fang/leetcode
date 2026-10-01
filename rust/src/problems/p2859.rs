pub struct Solution;

impl Solution {
    pub fn sum_indices_with_k_set_bits(nums: Vec<i32>, k: i32) -> i32 {
        nums.into_iter()
            .enumerate()
            .filter_map(|(i, x)| {
                if i.count_ones() == k as u32 {
                    Some(x)
                } else {
                    None
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            13,
            Solution::sum_indices_with_k_set_bits(vec![5, 10, 1, 5, 2], 1)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            1,
            Solution::sum_indices_with_k_set_bits(vec![4, 3, 2, 1], 2)
        );
    }
}
