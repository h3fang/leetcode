pub struct Solution;

impl Solution {
    pub fn find_final_value(nums: Vec<i32>, mut original: i32) -> i32 {
        const MAX: i32 = 1000;
        let mut m = vec![false; MAX as usize + 1];
        for x in nums {
            m[x as usize] = true;
        }
        while original <= MAX && m[original as usize] {
            original *= 2;
        }
        original
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(24, Solution::find_final_value(vec![5, 3, 6, 1, 12], 3));
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::find_final_value(vec![2, 7, 9], 4));
    }
}
