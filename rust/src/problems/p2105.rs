pub struct Solution;

impl Solution {
    pub fn minimum_refill(plants: Vec<i32>, capacity_a: i32, capacity_b: i32) -> i32 {
        let mut result = 0;
        let mut a = capacity_a;
        let mut b = capacity_b;
        let mut left = 0;
        let mut right = plants.len() - 1;
        while left < right {
            if a < plants[left] {
                result += 1;
                a = capacity_a;
            }
            a -= plants[left];
            left += 1;

            if b < plants[right] {
                result += 1;
                b = capacity_b;
            }
            b -= plants[right];
            right -= 1;
        }
        if left == right && a.max(b) < plants[left] {
            result += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::minimum_refill(vec![2, 2, 3, 3], 5, 5));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::minimum_refill(vec![2, 2, 3, 3], 3, 4));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::minimum_refill(vec![5], 10, 8));
    }

    #[test]
    fn case4() {
        assert_eq!(2, Solution::minimum_refill(vec![1, 2, 4, 4, 5], 6, 5));
    }

    #[test]
    fn case5() {
        assert_eq!(1, Solution::minimum_refill(vec![2, 2, 5, 2, 2], 5, 5));
    }
}
