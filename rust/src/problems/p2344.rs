pub struct Solution;

fn gcd(a: i32, b: i32) -> i32 {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}

impl Solution {
    pub fn min_operations(mut nums: Vec<i32>, nums_divide: Vec<i32>) -> i32 {
        let g = nums_divide[0];
        let g = nums_divide.into_iter().fold(g, gcd);

        nums.sort_unstable();
        for (i, &n) in nums.iter().enumerate() {
            if g % n == 0 {
                return i as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            2,
            Solution::min_operations(vec![2, 3, 2, 4, 3], vec![9, 6, 9, 3, 15])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            -1,
            Solution::min_operations(vec![4, 3, 6], vec![8, 2, 6, 10])
        );
    }
}
