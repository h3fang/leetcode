pub struct Solution;

impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut result = Vec::with_capacity(n as usize);
        let mut num = 1;
        for _ in 0..n {
            result.push(num);
            if num * 10 <= n {
                num *= 10;
            } else {
                while num % 10 == 9 || num + 1 > n {
                    num /= 10;
                }
                num += 1;
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
        assert_eq!(
            vec![1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9],
            Solution::lexical_order(13)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(vec![1, 2], Solution::lexical_order(2));
    }
}
