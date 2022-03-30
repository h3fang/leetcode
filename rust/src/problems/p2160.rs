pub struct Solution;

impl Solution {
    pub fn minimum_sum(mut num: i32) -> i32 {
        let mut d = vec![];
        while num > 0 {
            d.push(num % 10);
            num /= 10;
        }

        d.sort_unstable();
        d[0] * 10 + d[3] + d[1] * 10 + d[2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(52, Solution::minimum_sum(2932));
    }

    #[test]
    fn case2() {
        assert_eq!(13, Solution::minimum_sum(4009));
    }
}
