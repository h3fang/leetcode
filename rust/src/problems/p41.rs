pub struct Solution;

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        for x in &mut nums {
            if *x <= 0 {
                *x = n + 1;
            }
        }
        for i in 0..n as usize {
            let x = nums[i].abs();
            if x <= n {
                let j = x as usize - 1;
                nums[j] = -nums[j].abs();
            }
        }
        for (i, &x) in nums.iter().enumerate() {
            if x > 0 {
                return i as i32 + 1;
            }
        }
        n + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::first_missing_positive(vec![1, 2, 0]));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::first_missing_positive(vec![3, 4, -1, 1]));
    }

    #[test]
    fn case3() {
        assert_eq!(1, Solution::first_missing_positive(vec![7, 8, 9, 11, 12]));
    }
}
