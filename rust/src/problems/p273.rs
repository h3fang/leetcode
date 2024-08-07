pub struct Solution;

const DIGITS: [&str; 10] = [
    "Zero", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine",
];
const TEN_TO_TWENTY: [&str; 10] = [
    "Ten",
    "Eleven",
    "Twelve",
    "Thirteen",
    "Fourteen",
    "Fifteen",
    "Sixteen",
    "Seventeen",
    "Eighteen",
    "Nineteen",
];
const TENS: [&str; 10] = [
    "", "", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
];

impl Solution {
    pub fn number_to_words(num: i32) -> String {
        if num >= 10_0000_0000 {
            let a = Self::number_to_words(num / 10_0000_0000);
            let num = num % 10_0000_0000;
            if num > 0 {
                let b = Self::number_to_words(num);
                format!("{a} Billion {b}")
            } else {
                format!("{a} Billion")
            }
        } else if num >= 100_0000 {
            let a = Self::number_to_words(num / 100_0000);
            let num = num % 100_0000;
            if num > 0 {
                let b = Self::number_to_words(num);
                format!("{a} Million {b}")
            } else {
                format!("{a} Million")
            }
        } else if num >= 1000 {
            let a = Self::number_to_words(num / 1000);
            let num = num % 1000;
            if num > 0 {
                let b = Self::number_to_words(num);
                format!("{a} Thousand {b}")
            } else {
                format!("{a} Thousand")
            }
        } else if num >= 100 {
            let hundred = num / 100;
            let num = num % 100;
            if num > 0 {
                let b = Self::number_to_words(num);
                format!("{} Hundred {}", DIGITS[hundred as usize], b)
            } else {
                format!("{} Hundred", DIGITS[hundred as usize])
            }
        } else if num >= 20 {
            let ten = num / 10;
            let d = num % 10;
            if d > 0 {
                format!("{} {}", TENS[ten as usize], DIGITS[d as usize])
            } else {
                TENS[ten as usize].to_string()
            }
        } else if num >= 10 {
            let ten = num - 10;
            TEN_TO_TWENTY[ten as usize].to_string()
        } else {
            DIGITS[num as usize].to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("One Hundred Twenty Three", Solution::number_to_words(123));
    }

    #[test]
    fn case2() {
        assert_eq!(
            "Twelve Thousand Three Hundred Forty Five",
            Solution::number_to_words(12345)
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven",
            Solution::number_to_words(1234567)
        );
    }
}
