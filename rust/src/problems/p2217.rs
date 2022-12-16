pub struct Solution;

impl Solution {
    pub fn kth_palindrome(queries: Vec<i32>, int_length: i32) -> Vec<i64> {
        fn helper(q: i32, len: i32, inner: bool) -> i64 {
            let c = 10i32.pow((len + 1) as u32 / 2 - 1);
            let i = (q - 1) / c + i32::from(!inner);
            if i > 9 {
                return -1;
            }
            match len {
                1 => {
                    if inner {
                        q as i64 - 1
                    } else {
                        q as i64
                    }
                }
                2 => {
                    let q = if inner { q as i64 - 1 } else { q as i64 };
                    q * 10 + q
                }
                len => {
                    i as i64 * 10i64.pow(len as u32 - 1)
                        + helper((q - 1) % c + 1, len - 2, true) * 10
                        + i as i64
                }
            }
        }
        queries
            .iter()
            .map(|q| helper(*q, int_length, false))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![101, 111, 121, 131, 141, 989, 999],
            Solution::kth_palindrome(vec![1, 2, 3, 4, 5, 89, 90], 3)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![1111, 1331, 1551, 2552],
            Solution::kth_palindrome(vec![2, 4, 6, 16], 4)
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            vec![2, -1, 8, -1, -1, -1, -1, 9, 7, 6],
            Solution::kth_palindrome(
                vec![2, 201429812, 8, 520498110, 492711727, 339882032, 462074369, 9, 7, 6],
                1
            )
        );
    }

    #[test]
    fn case4() {
        assert_eq!(
            vec![-1, -1, -1, -1, 10801, -1],
            Solution::kth_palindrome(
                vec![449229674, 501930675, 40059525, 908875541, 9, 672504016],
                5
            )
        );
    }
}
