pub struct Solution;

impl Solution {
    pub fn max_bottles_drunk(mut num_bottles: i32, mut num_exchange: i32) -> i32 {
        let mut ans = num_bottles;
        while num_bottles >= num_exchange {
            ans += 1;
            num_bottles -= num_exchange - 1;
            num_exchange += 1;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(15, Solution::max_bottles_drunk(13, 6));
    }

    #[test]
    fn case2() {
        assert_eq!(13, Solution::max_bottles_drunk(10, 3));
    }
}
