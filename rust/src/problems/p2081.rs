pub struct Solution;

impl Solution {
    pub fn k_mirror(k: i32, n: i32) -> i64 {
        // all base-10 palindromes of n digits, sorted
        fn palindromes(n: u32) -> Vec<i64> {
            if n % 2 == 0 {
                let j = n / 2;
                let min = 10i64.pow(j - 1);
                (min..10 * min)
                    .map(|num| {
                        let mut front = num;
                        let mut back = 0;
                        while front > 0 {
                            back = back * 10 + front % 10;
                            front /= 10;
                        }
                        num * min * 10 + back
                    })
                    .collect::<Vec<_>>()
            } else {
                let j = n.div_ceil(2);
                let min = 10i64.pow(j - 1);
                (min..10 * min)
                    .map(|num| {
                        let mut front = num / 10;
                        let mut back = 0;
                        while front > 0 {
                            back = back * 10 + front % 10;
                            front /= 10;
                        }
                        num * min + back
                    })
                    .collect::<Vec<_>>()
            }
        }
        // base-10 to base-k
        fn to_base_k(mut n: i64, k: i64) -> Vec<u8> {
            let mut r = vec![];
            while n > 0 {
                r.push((n % k) as u8);
                n /= k;
            }
            r
        }
        fn is_palindrome(n: &[u8]) -> bool {
            for i in 0..n.len() / 2 {
                if n[i] != n[n.len() - 1 - i] {
                    return false;
                }
            }
            true
        }

        let mut n_digits = 1;
        let mut count = 0;
        let mut result = 0;
        loop {
            for num in palindromes(n_digits) {
                if is_palindrome(&to_base_k(num, k as i64)) {
                    result += num;
                    count += 1;
                    if count == n {
                        return result;
                    }
                }
            }
            n_digits += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(25, Solution::k_mirror(2, 5));
    }

    #[test]
    fn case2() {
        assert_eq!(499, Solution::k_mirror(3, 7));
    }

    #[test]
    fn case3() {
        assert_eq!(20379000, Solution::k_mirror(7, 17));
    }
}
