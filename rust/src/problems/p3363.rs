pub struct Solution;

impl Solution {
    pub fn max_collected_fruits(fruits: Vec<Vec<i32>>) -> i32 {
        let n = fruits.len();
        let mut mid: i32 = fruits.iter().enumerate().map(|(i, r)| r[i]).sum();
        let mut f = vec![0; n + 1];
        f[n - 1] = fruits[0][n - 1];
        for (i, r) in fruits.iter().enumerate().skip(1).take(n - 2) {
            let mut g = vec![0; n + 1];
            for j in (n - 1 - i).max(i + 1)..n {
                g[j] = f[j - 1].max(f[j]).max(f[j + 1]) + r[j];
            }
            f = g;
        }
        mid += f[n - 1];

        f.iter_mut().for_each(|e| *e = 0);
        f[n - 1] = fruits[n - 1][0];
        for i in 1..n - 1 {
            let mut g = vec![0; n + 1];
            for j in (n - 1 - i).max(i + 1)..n {
                g[j] = f[j - 1].max(f[j]).max(f[j + 1]) + fruits[j][i];
            }
            f = g;
        }

        mid + f[n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let fruits = [
            [1, 2, 3, 4],
            [5, 6, 8, 7],
            [9, 10, 11, 12],
            [13, 14, 15, 16],
        ]
        .iter()
        .map(|r| r.to_vec())
        .collect();
        assert_eq!(100, Solution::max_collected_fruits(fruits));
    }

    #[test]
    fn case2() {
        let fruits = [[1, 1], [1, 1]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(4, Solution::max_collected_fruits(fruits));
    }
}
