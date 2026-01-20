pub struct Solution;

impl Solution {
    pub fn min_bitwise_array(mut nums: Vec<i32>) -> Vec<i32> {
        for x in &mut nums {
            if *x == 2 {
                *x = -1;
            } else {
                let y = !*x;
                *x ^= (y & -y) >> 1;
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
        assert_eq!(
            vec![-1, 1, 4, 3],
            Solution::min_bitwise_array(vec![2, 3, 5, 7])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![9, 12, 15],
            Solution::min_bitwise_array(vec![11, 13, 31])
        );
    }
}
