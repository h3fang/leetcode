pub struct Solution;

impl Solution {
    pub fn original_digits(s: String) -> String {
        let mut freq = [0; 256];
        for b in s.as_bytes() {
            freq[*b as usize] += 1;
        }

        let mut result = Vec::new();

        let zero = freq[b'z' as usize];
        if zero > 0 {
            result.extend(vec!['0'; zero]);
            freq[b'e' as usize] -= zero;
            freq[b'r' as usize] -= zero;
            freq[b'o' as usize] -= zero;
        }

        let two = freq[b'w' as usize];
        if two > 0 {
            result.extend(vec!['2'; two]);
            freq[b't' as usize] -= two;
            freq[b'o' as usize] -= two;
        }

        let eight = freq[b'g' as usize];
        if eight > 0 {
            result.extend(vec!['8'; eight]);
            for b in [b'e', b'i', b'h', b't'] {
                freq[b as usize] -= eight;
            }
        }

        let three = freq[b'h' as usize];
        if three > 0 {
            result.extend(vec!['3'; three]);
            for b in [b't', b'r', b'e', b'e'] {
                freq[b as usize] -= three;
            }
        }

        let four = freq[b'u' as usize];
        if four > 0 {
            result.extend(vec!['4'; four]);
            for b in [b'f', b'o', b'r'] {
                freq[b as usize] -= four;
            }
        }

        let one = freq[b'o' as usize];
        if one > 0 {
            result.extend(vec!['1'; one]);
            for b in [b'n', b'e'] {
                freq[b as usize] -= one;
            }
        }

        let five = freq[b'f' as usize];
        if five > 0 {
            result.extend(vec!['5'; five]);
            for b in [b'i', b'v', b'e'] {
                freq[b as usize] -= five;
            }
        }

        let six = freq[b'x' as usize];
        if six > 0 {
            result.extend(vec!['6'; six]);
            for b in [b's', b'i'] {
                freq[b as usize] -= six;
            }
        }

        result.extend(vec!['7'; freq[b's' as usize]]);
        result.extend(vec!['9'; freq[b'i' as usize]]);
        result.sort_unstable();
        result.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = "owoztneoer".to_string();
        assert_eq!("012".to_string(), Solution::original_digits(s));
    }

    #[test]
    fn case2() {
        let s = "fviefuro".to_string();
        assert_eq!("45".to_string(), Solution::original_digits(s));
    }
}
