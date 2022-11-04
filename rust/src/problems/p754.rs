pub struct Solution;

impl Solution {
    pub fn reach_number(target: i32) -> i32 {
        let mut t = target.abs();
        let mut k = 0;
        while t > 0 {
            k += 1;
            t -= k;
        }
        if t % 2 == 0 {
            k
        } else {
            k + 1 + k % 2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::reach_number(2));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::reach_number(3));
    }
}
