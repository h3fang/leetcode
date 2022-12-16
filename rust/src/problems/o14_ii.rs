pub struct Solution;

const P: u64 = 1000000007;

impl Solution {
    pub fn cutting_rope(n: i32) -> i32 {
        fn reminder(mut a: i32) -> u64 {
            let mut r = 1;
            let mut x = 3u64;
            while a > 0 {
                if a % 2 == 1 {
                    r = (r * x) % P;
                }
                x = (x * x) % P;
                a /= 2;
            }
            r
        }
        match n {
            2 => 1,
            3 => 2,
            _ => {
                let a = n / 3;
                ((match n % 3 {
                    1 => reminder(a - 1) * 4,
                    2 => reminder(a) * 2,
                    _ => reminder(a),
                }) % P) as i32
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::cutting_rope(2));
    }

    #[test]
    fn case2() {
        assert_eq!(620946522, Solution::cutting_rope(1000));
    }
}
