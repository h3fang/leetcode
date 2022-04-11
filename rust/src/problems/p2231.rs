pub struct Solution;

impl Solution {
    pub fn largest_integer(mut num: i32) -> i32 {
        let mut digits = vec![];
        while num > 0 {
            digits.push(num % 10);
            num /= 10;
        }
        digits.reverse();
        let (mut odd, mut odd_pos) = (vec![], vec![]);
        let (mut even, mut even_pos) = (vec![], vec![]);
        for (i, &d) in digits.iter().enumerate() {
            if d % 2 == 0 {
                even.push(d);
                even_pos.push(i);
            } else {
                odd.push(d);
                odd_pos.push(i);
            }
        }
        odd.sort_unstable();
        even.sort_unstable();

        for (d, i) in odd.into_iter().rev().zip(odd_pos) {
            digits[i] = d;
        }

        for (d, i) in even.into_iter().rev().zip(even_pos) {
            digits[i] = d;
        }

        digits.into_iter().fold(0, |acc, d| acc * 10 + d)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3412, Solution::largest_integer(1234));
    }

    #[test]
    fn case2() {
        assert_eq!(87655, Solution::largest_integer(65875));
    }
}
