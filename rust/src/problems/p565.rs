pub struct Solution;

impl Solution {
    pub fn array_nesting(mut nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let mut result = 0;
        for mut i in 0..n {
            let mut cnt = 0;
            while nums[i as usize] < n {
                let e = nums[i as usize];
                nums[i as usize] = n;
                i = e;
                cnt += 1;
            }
            result = result.max(cnt);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::array_nesting(vec![5, 4, 0, 3, 1, 6, 2]));
    }
}
