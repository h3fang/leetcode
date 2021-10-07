pub struct Solution;

impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        (0..nums.len())
            .filter_map(|i| {
                let abs = nums[i].abs() - 1;
                if nums[abs as usize] < 0 {
                    Some(abs + 1)
                } else {
                    nums[abs as usize] *= -1;
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
        assert_eq!(
            vec![2, 3],
            Solution::find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(vec![1], Solution::find_duplicates(vec![1, 1, 2]));
    }

    #[test]
    fn case3() {
        assert_eq!(Vec::<i32>::new(), Solution::find_duplicates(vec![1]));
    }
}
