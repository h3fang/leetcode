pub struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        use std::cmp::Ordering;
        let mut a = 0i32;
        let mut b = numbers.len() as i32 - 1;
        while a < b {
            let sum = numbers[a as usize] + numbers[b as usize];
            match sum.cmp(&target) {
                Ordering::Greater => b -= 1,
                Ordering::Less => a += 1,
                Ordering::Equal => break,
            }
        }
        vec![a + 1, b + 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let numbers = vec![2, 7, 11, 15];
        let target = 9;
        let expected = vec![1, 2];
        assert_eq!(expected, Solution::two_sum(numbers, target));
    }

    #[test]
    fn case2() {
        let numbers = vec![2, 3, 4];
        let target = 6;
        let expected = vec![1, 3];
        assert_eq!(expected, Solution::two_sum(numbers, target));
    }

    #[test]
    fn case3() {
        let numbers = vec![-1, 0];
        let target = -1;
        let expected = vec![1, 2];
        assert_eq!(expected, Solution::two_sum(numbers, target));
    }
}
