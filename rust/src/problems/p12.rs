pub struct Solution;

const S: [&str; 13] = [
    "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
];
const V: [i32; 13] = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let mut result = String::with_capacity(32);
        for (&s, &v) in S.iter().zip(&V) {
            while num >= v {
                num -= v;
                result.push_str(s);
            }
            if num == 0 {
                break;
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
        assert_eq!("III", Solution::int_to_roman(3));
    }

    #[test]
    fn case2() {
        assert_eq!("LVIII", Solution::int_to_roman(58));
    }

    #[test]
    fn case3() {
        assert_eq!("MCMXCIV", Solution::int_to_roman(1994));
    }
}
