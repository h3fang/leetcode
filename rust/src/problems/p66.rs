pub struct Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        for d in digits.iter_mut().rev() {
            if *d < 9 {
                *d += 1;
                return digits;
            }
            *d = 0;
        }

        digits.push(0);
        digits[0] = 1;
        digits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(vec![1, 2, 4], Solution::plus_one(vec![1, 2, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(vec![4, 3, 2, 2], Solution::plus_one(vec![4, 3, 2, 1]));
    }

    #[test]
    fn case3() {
        assert_eq!(vec![1, 0], Solution::plus_one(vec![9]));
    }
}
