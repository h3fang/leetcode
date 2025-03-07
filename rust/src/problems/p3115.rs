pub struct Solution;

impl Solution {
    pub fn maximum_prime_difference(nums: Vec<i32>) -> i32 {
        let mut f = [true; 101];
        f[1] = false;
        for x in 2..=100 {
            let mut y = 2 * x;
            while y <= 100 {
                f[y as usize] = false;
                y += x;
            }
        }
        let l = nums.iter().position(|&i| f[i as usize]).unwrap();
        let r = nums.iter().rposition(|&i| f[i as usize]).unwrap();
        (r - l) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::maximum_prime_difference(vec![4, 2, 9, 5, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::maximum_prime_difference(vec![4, 8, 2, 8]));
    }
}
