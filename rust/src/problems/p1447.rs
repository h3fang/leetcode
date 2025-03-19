pub struct Solution;

impl Solution {
    pub fn simplified_fractions(n: i32) -> Vec<String> {
        fn gcd(a: i32, b: i32) -> i32 {
            if a == 0 { b } else { gcd(b % a, a) }
        }

        let mut result = Vec::with_capacity((n * n) as usize);
        for nom in 2..=n {
            for denom in 1..nom {
                if gcd(denom, nom) == 1 {
                    result.push(format!("{denom}/{nom}"));
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
        let mut result = Solution::simplified_fractions(2);
        result.sort_unstable();
        let expected = ["1/2"];
        let mut expected = expected.iter().map(|f| f.to_string()).collect::<Vec<_>>();
        expected.sort_unstable();
        assert_eq!(expected, result);
    }

    #[test]
    fn case2() {
        let mut result = Solution::simplified_fractions(3);
        result.sort_unstable();
        let expected = ["1/2", "1/3", "2/3"];
        let mut expected = expected.iter().map(|f| f.to_string()).collect::<Vec<_>>();
        expected.sort_unstable();
        assert_eq!(expected, result);
    }

    #[test]
    fn case3() {
        let mut result = Solution::simplified_fractions(4);
        result.sort_unstable();
        let expected = ["1/2", "1/3", "1/4", "2/3", "3/4"];
        let mut expected = expected.iter().map(|f| f.to_string()).collect::<Vec<_>>();
        expected.sort_unstable();
        assert_eq!(expected, result);
    }
}
