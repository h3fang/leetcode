pub struct Solution;

impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .filter(|x| (x.ilog10() + 1) % 2 == 0)
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::find_numbers(vec![12, 345, 2, 6, 7896]));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::find_numbers(vec![555, 901, 482, 1771]));
    }
}
