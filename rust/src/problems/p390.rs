pub struct Solution;

impl Solution {
    pub fn last_remaining(n: i32) -> i32 {
        let mut a1 = 1;
        let mut k = 0;
        let mut cnt = n;
        let mut step = 1;
        while cnt > 1 {
            if k % 2 == 0 {
                a1 += step;
            } else {
                a1 = if cnt % 2 == 0 { a1 } else { a1 + step };
            }
            k += 1;
            cnt /= 2;
            step *= 2;
        }
        a1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(6, Solution::last_remaining(9));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::last_remaining(1));
    }
}
