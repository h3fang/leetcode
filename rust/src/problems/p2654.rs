pub struct Solution;

fn gcd(mut x: i32, mut y: i32) -> i32 {
    while y != 0 {
        (x, y) = (y, x % y);
    }
    x
}

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let (mut g, mut cnt1) = (0, 0);
        for &x in &nums {
            g = gcd(g, x);
            if x == 1 {
                cnt1 += 1;
            }
        }
        if g > 1 {
            return -1;
        }
        if cnt1 > 0 {
            return n - cnt1;
        }
        let mut min = n;
        for i in 0..n as usize {
            let mut g = 0;
            for (j, &x) in nums.iter().enumerate().skip(i) {
                g = gcd(g, x);
                if g == 1 {
                    min = min.min((j - i) as i32);
                    break;
                }
            }
        }
        min + n - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::min_operations(vec![2, 6, 3, 4]));
    }

    #[test]
    fn case2() {
        assert_eq!(-1, Solution::min_operations(vec![2, 10, 6, 14]));
    }
}
