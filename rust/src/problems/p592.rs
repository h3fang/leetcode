pub struct Solution;

fn gcd(a: i32, b: i32) -> i32 {
    if a == 0 { b } else { gcd(b % a, a) }
}

fn lcm(a: i32, b: i32) -> i32 {
    a * b / gcd(a, b)
}

impl Solution {
    pub fn fraction_addition(expression: String) -> String {
        let mut nume = 0;
        let mut deno = 1;
        let mut sign = true;
        for t in expression.split_inclusive(['+', '-']) {
            if t.len() == 1 {
                sign = false;
                continue;
            }
            let s = sign;
            let last = *t.as_bytes().last().unwrap();
            let f = if last.is_ascii_digit() {
                t
            } else {
                sign = last == b'+';
                &t[..t.len() - 1]
            };
            let (nu, de) = f.split_once('/').unwrap();
            let nu = nu.parse::<i32>().unwrap();
            let de = de.parse::<i32>().unwrap();
            let m = lcm(deno, de);
            let n1 = nume * (m / deno);
            let n2 = nu * (m / de);
            nume = if s { n1 + n2 } else { n1 - n2 };
            deno = m;
        }
        let g = gcd(nume.abs(), deno);
        format!("{}/{}", nume / g, deno / g)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("0/1", Solution::fraction_addition("-1/2+1/2".into()));
    }

    #[test]
    fn case2() {
        assert_eq!("1/3", Solution::fraction_addition("-1/2+1/2+1/3".into()));
    }

    #[test]
    fn case3() {
        assert_eq!("-1/6", Solution::fraction_addition("1/3-1/2".into()));
    }
}
