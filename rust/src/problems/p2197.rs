pub struct Solution;

fn gcd(mut x: i32, mut y: i32) -> i32 {
    while x != 0 {
        (x, y) = (y % x, x);
    }
    y
}

impl Solution {
    pub fn replace_non_coprimes(mut nums: Vec<i32>) -> Vec<i32> {
        let mut i = 0;
        for j in 0..nums.len() {
            let mut y = nums[j];
            while i > 0 {
                let x = nums[i - 1];
                let f = gcd(x, y);
                if f <= 1 {
                    break;
                }
                i -= 1;
                y *= x / f;
            }
            nums[i] = y;
            i += 1;
        }
        nums.resize(i, 0);
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![12, 7, 6],
            Solution::replace_non_coprimes(vec![6, 4, 3, 2, 7, 6, 2])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![2, 1, 1, 3],
            Solution::replace_non_coprimes(vec![2, 2, 1, 1, 3, 3, 3])
        );
    }
}
