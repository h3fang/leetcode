pub struct Solution;

impl Solution {
    pub fn find_integers(n: i32) -> i32 {
        let mut f = [0; 32];
        f[0] = 1;
        f[1] = 2;
        for i in 2..32 {
            f[i] = f[i - 1] + f[i - 2];
        }
        let (mut pre, mut result) = (0, 0);
        for i in (0..31).rev() {
            if n & (1 << i) > 0 {
                result += f[i];
                if pre == 1 {
                    break;
                }
                pre = 1;
            } else {
                pre = 0;
            }
            if i == 0 {
                result += 1;
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
        assert_eq!(5, Solution::find_integers(5));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::find_integers(1));
    }

    #[test]
    fn case3() {
        assert_eq!(3, Solution::find_integers(2));
    }
}
