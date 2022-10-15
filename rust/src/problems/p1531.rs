pub struct Solution;

impl Solution {
    pub fn get_length_of_optimal_compression(s: String, k: i32) -> i32 {
        fn calc(x: i32) -> i32 {
            if x == 1 {
                1
            } else if x < 10 {
                2
            } else if x < 100 {
                3
            } else {
                4
            }
        }

        let s = s.as_bytes();
        let n = s.len();
        let mut f = vec![vec![i32::MAX / 2; k as usize + 1]; n + 1];
        f[0][0] = 0;
        for i in 1..=n {
            for j in 0..=(k as usize).min(i) {
                if j > 0 {
                    f[i][j] = f[i - 1][j - 1];
                }
                let mut same = 0;
                let mut diff = 0;
                for i0 in (1..=i).rev() {
                    if diff > j {
                        break;
                    }
                    if s[i0 - 1] == s[i - 1] {
                        same += 1;
                        f[i][j] = f[i][j].min(f[i0 - 1][j - diff] + calc(same));
                    } else {
                        diff += 1;
                    }
                }
            }
        }
        f[n][k as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            4,
            Solution::get_length_of_optimal_compression("aaabcccd".to_string(), 2)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            2,
            Solution::get_length_of_optimal_compression("aabbaa".to_string(), 2)
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            3,
            Solution::get_length_of_optimal_compression("aaaaaaaaaaa".to_string(), 0)
        );
    }
}
