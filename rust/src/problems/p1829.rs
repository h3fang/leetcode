pub struct Solution;

impl Solution {
    pub fn get_maximum_xor(mut nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let mut xor = 0;
        let max = (1 << maximum_bit) - 1;
        nums.iter_mut().for_each(|x| {
            xor ^= *x;
            *x = max ^ xor;
        });
        nums.reverse();
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![0, 3, 2, 3],
            Solution::get_maximum_xor(vec![0, 1, 1, 3], 2)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![5, 2, 6, 5],
            Solution::get_maximum_xor(vec![2, 3, 4, 7], 3)
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            vec![4, 3, 6, 4, 6, 7],
            Solution::get_maximum_xor(vec![0, 1, 2, 2, 5, 7], 3)
        );
    }
}
