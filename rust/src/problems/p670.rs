pub struct Solution;

impl Solution {
    pub fn maximum_swap(mut num: i32) -> i32 {
        let mut digits = Vec::with_capacity(16);
        while num > 0 {
            digits.push(num % 10);
            num /= 10;
        }
        digits.reverse();
        for (i, &d1) in digits.iter().enumerate() {
            if let Some((j, _)) = digits
                .iter()
                .enumerate()
                .skip(i + 1)
                .filter(|(_, d)| **d > d1)
                .max_by_key(|e| e.1)
            {
                digits.swap(i, j);
                break;
            }
        }
        digits.into_iter().fold(0, |acc, d| acc * 10 + d)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(7236, Solution::maximum_swap(2736));
    }

    #[test]
    fn case2() {
        assert_eq!(9973, Solution::maximum_swap(9973));
    }
}
