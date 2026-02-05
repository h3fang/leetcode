pub struct Solution;

impl Solution {
    pub fn construct_transformed_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len() as i32;
        nums.iter()
            .enumerate()
            .map(|(i, x)| {
                let j = (i as i32 + x).rem_euclid(n);
                nums[j as usize]
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![1, 1, 1, 3],
            Solution::construct_transformed_array(vec![3, -2, 1, 1])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![-1, -1, 4],
            Solution::construct_transformed_array(vec![-1, 4, -1])
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            vec![-4, -10, -10],
            Solution::construct_transformed_array(vec![-10, -10, -4])
        );
    }
}
