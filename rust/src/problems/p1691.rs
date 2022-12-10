pub struct Solution;

impl Solution {
    pub fn max_height(mut cuboids: Vec<Vec<i32>>) -> i32 {
        cuboids.iter_mut().for_each(|c| c.sort_unstable());
        cuboids.sort_unstable();
        let n = cuboids.len();
        let mut dp = vec![0; n];
        for (i, c) in cuboids.iter().enumerate() {
            dp[i] = c[2];
            for j in 0..i {
                if cuboids[j][0] <= c[0] && cuboids[j][1] <= c[1] && cuboids[j][2] <= c[2] {
                    dp[i] = dp[i].max(dp[j] + c[2]);
                }
            }
        }
        dp.into_iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let cuboids = [[50, 45, 20], [95, 37, 53], [45, 23, 12]]
            .iter()
            .map(|c| c.to_vec())
            .collect();
        assert_eq!(190, Solution::max_height(cuboids));
    }

    #[test]
    fn case2() {
        let cuboids = [[38, 25, 45], [76, 35, 3]]
            .iter()
            .map(|c| c.to_vec())
            .collect();
        assert_eq!(76, Solution::max_height(cuboids));
    }

    #[test]
    fn case3() {
        let cuboids = [
            [7, 11, 17],
            [7, 17, 11],
            [11, 7, 17],
            [11, 17, 7],
            [17, 7, 11],
            [17, 11, 7],
        ]
        .iter()
        .map(|c| c.to_vec())
        .collect();
        assert_eq!(102, Solution::max_height(cuboids));
    }
}
