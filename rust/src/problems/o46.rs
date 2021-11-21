pub struct Solution;

impl Solution {
    pub fn translate_num(mut num: i32) -> i32 {
        let mut digits = vec![];
        loop {
            digits.push(num % 10);
            num /= 10;
            if num == 0 {
                break;
            }
        }
        digits.reverse();
        let n = digits.len();
        let mut pp = 1;
        let mut p = 1;
        for i in 1..n {
            let mut curr = p;
            match digits[i] {
                d if d <= 5 => {
                    if digits[i - 1] == 1 || digits[i - 1] == 2 {
                        curr += pp;
                    }
                }
                _ => {
                    if digits[i - 1] == 1 {
                        curr += pp;
                    }
                }
            }
            pp = p;
            p = curr;
        }
        p
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::translate_num(12258));
    }
}
