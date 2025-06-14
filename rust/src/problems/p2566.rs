pub struct Solution;

fn number(digits: &[i32]) -> i32 {
    digits.iter().fold(0, |acc, d| acc * 10 + d)
}

fn max(mut ds: Vec<i32>) -> i32 {
    if let Some(&d) = ds.iter().find(|&&d| d != 9) {
        ds.iter_mut().for_each(|x| {
            if *x == d {
                *x = 9;
            }
        });
    }
    number(&ds)
}

fn min(digits: &[i32]) -> i32 {
    let d0 = digits[0];
    digits
        .iter()
        .fold(0, |acc, &d| acc * 10 + if d == d0 { 0 } else { d })
}

impl Solution {
    pub fn min_max_difference(mut num: i32) -> i32 {
        let mut digits = Vec::with_capacity(9);
        while num > 0 {
            digits.push(num % 10);
            num /= 10;
        }
        digits.reverse();

        let min = min(&digits);
        let max = max(digits);
        max - min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(99009, Solution::min_max_difference(11891));
    }

    #[test]
    fn case2() {
        assert_eq!(99, Solution::min_max_difference(90));
    }
}
