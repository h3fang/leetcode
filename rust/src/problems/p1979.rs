pub struct Solution;

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while a != 0 {
        (a, b) = (b % a, a);
    }
    b
}

impl Solution {
    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        let (min, max) = nums
            .into_iter()
            .fold((i32::MAX, i32::MIN), |(min, max), x| {
                (min.min(x), max.max(x))
            });
        gcd(min, max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::find_gcd(vec![2, 5, 6, 9, 10]));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::find_gcd(vec![7, 5, 6, 8, 3]));
    }

    #[test]
    fn case3() {
        assert_eq!(3, Solution::find_gcd(vec![3, 3]));
    }
}
