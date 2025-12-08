pub struct Solution;

impl Solution {
    pub fn count_triples(n: i32) -> i32 {
        let mut ans = 0;
        for a in 1..n {
            for b in a..n {
                let c2 = a * a + b * b;
                if c2 > n * n {
                    break;
                }
                let c = c2.isqrt();
                if c * c == c2 {
                    ans += 2;
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::count_triples(5));
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::count_triples(10));
    }
}
