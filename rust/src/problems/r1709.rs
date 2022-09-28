pub struct Solution;

impl Solution {
    pub fn get_kth_magic_number(k: i32) -> i32 {
        let mut f = vec![0; k as usize + 1];
        f[1] = 1;
        let mut p3 = 1;
        let mut p5 = 1;
        let mut p7 = 1;
        for i in 2..=k as usize {
            let min = (f[p3] * 3).min(f[p5] * 5).min(f[p7] * 7);
            f[i] = min;
            if min == f[p3] * 3 {
                p3 += 1;
            }
            if min == f[p5] * 5 {
                p5 += 1;
            }
            if min == f[p7] * 7 {
                p7 += 1;
            }
        }
        f[k as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(9, Solution::get_kth_magic_number(5));
    }
}
