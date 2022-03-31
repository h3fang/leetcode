pub struct Solution;

impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        (left..=right)
            .filter(|&num| {
                let mut n = num;
                while n > 0 {
                    let d = n % 10;
                    if d == 0 || num % d != 0 {
                        return false;
                    }
                    n /= 10;
                }
                true
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22],
            Solution::self_dividing_numbers(1, 22)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![48, 55, 66, 77],
            Solution::self_dividing_numbers(47, 85)
        );
    }
}
