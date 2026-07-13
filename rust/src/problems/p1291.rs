pub struct Solution;

impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let mut ans = Vec::with_capacity(36);
        let (mut x0, mut pow10, mut len) = (12, 10, 2);

        while x0 <= high {
            pow10 *= 10;
            let mut x = x0;
            for i in len..10 {
                if x > high {
                    return ans;
                }
                if x >= low {
                    ans.push(x);
                }
                x = x * 10 + i + 1 - (i + 1 - len) * pow10;
            }
            len += 1;
            x0 = x0 * 10 + len;
        }

        ans
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
