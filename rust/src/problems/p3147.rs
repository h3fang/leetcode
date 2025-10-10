pub struct Solution;

impl Solution {
    pub fn maximum_energy(energy: Vec<i32>, k: i32) -> i32 {
        let (k, n) = (k as usize, energy.len());
        let mut ans = i32::MIN;
        #[allow(clippy::needless_range_loop)]
        for mut i in n - k..n {
            let mut s = 0;
            loop {
                s += energy[i];
                ans = ans.max(s);
                if i < k {
                    break;
                }
                i -= k;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::maximum_energy(vec![5, 2, -10, -5, 1], 3));
    }

    #[test]
    fn case2() {
        assert_eq!(-1, Solution::maximum_energy(vec![-2, -3, -1], 2));
    }
}
