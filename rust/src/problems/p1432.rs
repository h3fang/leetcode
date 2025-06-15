pub struct Solution;

fn number(digits: &[i32]) -> i32 {
    digits.iter().fold(0, |acc, d| acc * 10 + d)
}

fn max(digits: &[i32]) -> i32 {
    let mut ds = digits.to_vec();
    if let Some(&d) = digits.iter().find(|&&d| d < 9) {
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
    let mut ds = digits.to_vec();
    if d0 == 1 {
        if let Some(&d) = digits[1..].iter().find(|&&d| d > 1) {
            ds.iter_mut().for_each(|x| {
                if *x == d {
                    *x = 0;
                }
            });
        }
    } else {
        ds.iter_mut().for_each(|x| {
            if *x == d0 {
                *x = 1;
            }
        });
    }
    number(&ds)
}

impl Solution {
    pub fn max_diff(mut num: i32) -> i32 {
        let mut digits = Vec::with_capacity(9);
        while num > 0 {
            digits.push(num % 10);
            num /= 10;
        }
        digits.reverse();
        let min = min(&digits);
        let max = max(&digits);
        max - min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(888, Solution::max_diff(555));
    }

    #[test]
    fn case2() {
        assert_eq!(8, Solution::max_diff(9));
    }
}
