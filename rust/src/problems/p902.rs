pub struct Solution;

impl Solution {
    pub fn at_most_n_given_digit_set(digits: Vec<String>, mut n: i32) -> i32 {
        let digits = digits
            .iter()
            .map(|d| d.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let mut ns = vec![];
        while n > 0 {
            ns.push(n % 10);
            n /= 10;
        }
        ns.reverse();

        // less than n's digits
        let mut result = 0;
        let mut k = 1;
        for _ in 1..ns.len() {
            k *= digits.len();
            result += k as i32;
        }

        // same as n's digits
        let p = ns.len();
        let mut dp = vec![0; p + 1];
        dp[p] = 1;
        let k = digits.len() as i32;
        for i in (0..p).rev() {
            let mut smaller = 0;
            for d in &digits {
                match d.cmp(&ns[i]) {
                    std::cmp::Ordering::Less => smaller += 1,
                    std::cmp::Ordering::Equal => {
                        dp[i] = dp[i + 1];
                        break;
                    }
                    std::cmp::Ordering::Greater => break,
                }
            }
            dp[i] += smaller * k.pow((p - i - 1) as u32);
        }

        result + dp[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let digits = ["1", "3", "5", "7"];
        let digits = digits.iter().map(|d| d.to_string()).collect();
        let n = 100;
        assert_eq!(20, Solution::at_most_n_given_digit_set(digits, n));
    }

    #[test]
    fn case2() {
        let digits = ["1", "4", "9"];
        let digits = digits.iter().map(|d| d.to_string()).collect();
        let n = 1000000000;
        assert_eq!(29523, Solution::at_most_n_given_digit_set(digits, n));
    }

    #[test]
    fn case3() {
        let digits = ["1", "3", "5", "9"];
        let digits = digits.iter().map(|d| d.to_string()).collect();
        let n = 854923789;
        assert_eq!(283988, Solution::at_most_n_given_digit_set(digits, n));
    }
}
