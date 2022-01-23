pub struct Solution;

impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        fn sequential(len: i32, start: i32, low: i32, high: i32) -> Option<i32> {
            if start + len > 10 {
                return None;
            }
            let num = (start..start + len).fold(0, |acc, n| acc * 10 + n);
            if num >= low && num <= high {
                Some(num)
            } else {
                None
            }
        }

        let mut result = vec![];
        let l1 = low.to_string().len();
        let l2 = high.to_string().len();
        for len in l1..=l2 {
            for start in 1..=9 {
                if let Some(n) = sequential(len as i32, start, low, high) {
                    result.push(n);
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(vec![123, 234], Solution::sequential_digits(100, 300));
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![1234, 2345, 3456, 4567, 5678, 6789, 12345],
            Solution::sequential_digits(1000, 13000)
        );
    }
}
