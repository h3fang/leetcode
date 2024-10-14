pub struct Solution;

impl Solution {
    pub fn super_egg_drop(k: i32, n: i32) -> i32 {
        let k = k as usize;
        let mut f = vec![0; k + 1];
        for i in 1.. {
            for j in (1..=k).rev() {
                f[j] = 1 + f[j] + f[j - 1];
            }
            if f[k] >= n {
                return i;
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::super_egg_drop(1, 2));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::super_egg_drop(2, 6));
    }

    #[test]
    fn case3() {
        assert_eq!(4, Solution::super_egg_drop(3, 14));
    }
}
