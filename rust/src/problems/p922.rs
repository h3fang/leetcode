pub struct Solution;

impl Solution {
    pub fn sort_array_by_parity_ii(mut nums: Vec<i32>) -> Vec<i32> {
        let (mut odd, mut even, mut i) = (1, 0, 0);
        while i < nums.len() {
            if (i % 2 == 1 && i < odd) || (i % 2 == 0 && i < even) {
                i += 1;
            } else if nums[i] % 2 == 0 {
                nums.swap(i, even);
                even += 2;
            } else {
                nums.swap(i, odd);
                odd += 2;
            }
        }
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let result = Solution::sort_array_by_parity_ii(vec![4, 2, 5, 7]);
        assert!(result.chunks(2).all(|w| w[0] % 2 == 0 && w[1] % 2 == 1));
    }

    #[test]
    fn case2() {
        let result = Solution::sort_array_by_parity_ii(vec![2, 3]);
        assert!(result.chunks(2).all(|w| w[0] % 2 == 0 && w[1] % 2 == 1));
    }

    #[test]
    fn case3() {
        let result = Solution::sort_array_by_parity_ii(vec![1, 2, 3, 3, 2, 3, 0, 4]);
        assert!(result.chunks(2).all(|w| w[0] % 2 == 0 && w[1] % 2 == 1));
    }
}
