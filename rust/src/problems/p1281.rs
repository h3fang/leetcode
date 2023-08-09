pub struct Solution;

impl Solution {
    pub fn subtract_product_and_sum(mut n: i32) -> i32 {
        let (mut prod, mut sum) = (1, 0);
        while n > 0 {
            let d = n % 10;
            prod *= d;
            sum += d;
            n /= 10;
        }
        prod - sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(15, Solution::subtract_product_and_sum(234));
    }

    #[test]
    fn case2() {
        assert_eq!(21, Solution::subtract_product_and_sum(4421));
    }
}
