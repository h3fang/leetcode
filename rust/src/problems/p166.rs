pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        let numerator = numerator as i64;
        let denominator = denominator as i64;
        let integer = numerator / denominator;
        if numerator % denominator == 0 {
            return integer.to_string();
        }

        let sign = numerator.signum() * denominator.signum();
        let numerator = numerator.abs();
        let denominator = denominator.abs();
        let mut reminder = numerator % denominator;
        let mut map = HashMap::with_capacity(10000);
        let mut decimal = Vec::with_capacity(10000);
        let mut repeating = false;
        let mut start = 0;
        while reminder != 0 {
            reminder *= 10;
            let d = (reminder / denominator) as u8 + b'0';
            reminder %= denominator;

            let idx = map.get(&reminder).cloned().unwrap_or(0);
            if idx > 0 && decimal[idx - 1] == *decimal.last().unwrap() {
                repeating = true;
                decimal.pop();
                start = idx - 1;
                break;
            }

            map.insert(reminder, decimal.len());
            decimal.push(d);
        }

        let sign = if sign >= 0 { "" } else { "-" };
        let decimal = unsafe { String::from_utf8_unchecked(decimal) };

        if repeating {
            format!(
                "{}{}.{}({})",
                sign,
                integer.abs(),
                &decimal[..start],
                &decimal[start..]
            )
        } else {
            format!("{}{}.{}", sign, integer.abs(), decimal)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("0.00(000000465661289042462740251655654056577585848337359161441621040707904997124914069194026549138227660723878669455195477065427143370461252966751355553982241280310754777158628319049732085502639731402098131932683780538602845887105337854867197032523144157689601770377165713821223802198558308923834223016478952081795603341592860749337303449725)"
        .to_string(), Solution::fraction_to_decimal(1, 214748364));
    }

    #[test]
    fn case2() {
        assert_eq!(
            "0.0000000004656612873077392578125".to_string(),
            Solution::fraction_to_decimal(-1, -2147483648)
        );
    }
}
