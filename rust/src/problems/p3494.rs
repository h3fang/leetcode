pub struct Solution;

impl Solution {
    pub fn min_time(skill: Vec<i32>, mana: Vec<i32>) -> i64 {
        let n = skill.len();
        let mut f = vec![0; n];
        for &m in &mana {
            let m = m as i64;
            let mut delay = 0;
            let mut t = f[0];
            for i in 0..n {
                t += skill[i] as i64 * m;
                if i + 1 < n {
                    delay = delay.max(f[i + 1] - t);
                }
            }
            f[0] += delay + skill[0] as i64 * m;
            for i in 1..n {
                f[i] = f[i - 1] + skill[i] as i64 * m;
            }
        }
        f[n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(110, Solution::min_time(vec![1, 5, 2, 4], vec![5, 1, 4, 2]))
    }

    #[test]
    fn case2() {
        assert_eq!(5, Solution::min_time(vec![1, 1, 1], vec![1, 1, 1]))
    }

    #[test]
    fn case3() {
        assert_eq!(21, Solution::min_time(vec![1, 2, 3, 4], vec![1, 2]))
    }
}
