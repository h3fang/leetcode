pub struct Solution;

impl Solution {
    pub fn max_energy_boost(energy_drink_a: Vec<i32>, energy_drink_b: Vec<i32>) -> i64 {
        let mut f = [[i64::MIN / 2; 2]; 2];
        f[0][0] = energy_drink_a[0] as i64;
        f[1][0] = energy_drink_b[0] as i64;
        for (a, b) in energy_drink_a.into_iter().zip(energy_drink_b).skip(1) {
            let fa = (f[0][0]).max(f[1][1]) + a as i64;
            let fb = (f[1][0]).max(f[0][1]) + b as i64;
            f[0] = [fa, f[0][0]];
            f[1] = [fb, f[1][0]];
        }
        f[0][0].max(f[1][0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::max_energy_boost(vec![1, 3, 1], vec![3, 1, 1]));
    }

    #[test]
    fn case2() {
        assert_eq!(7, Solution::max_energy_boost(vec![4, 1, 1], vec![1, 1, 3]));
    }
}
