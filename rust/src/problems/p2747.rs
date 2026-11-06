pub struct Solution;

impl Solution {
    pub fn count_servers(n: i32, mut logs: Vec<Vec<i32>>, x: i32, queries: Vec<i32>) -> Vec<i32> {
        logs.sort_unstable_by_key(|e| e[1]);
        let mut queries: Vec<(usize, i32)> = queries.into_iter().enumerate().collect();
        queries.sort_unstable_by_key(|e| e.1);
        let mut ans = vec![0; queries.len()];

        let (mut l, mut r, mut idle) = (0, 0, n);
        let mut f = vec![0; n as usize + 1];

        for (i, q) in queries {
            while r < logs.len() && logs[r][1] <= q {
                if f[logs[r][0] as usize] == 0 {
                    idle -= 1;
                }
                f[logs[r][0] as usize] += 1;
                r += 1;
            }

            while l < logs.len() && logs[l][1] < q - x {
                f[logs[l][0] as usize] -= 1;
                if f[logs[l][0] as usize] == 0 {
                    idle += 1;
                }
                l += 1;
            }

            ans[i] = idle;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 3;
        let logs = [[1, 3], [2, 6], [1, 5]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        let x = 5;
        let queries = vec![10, 11];
        assert_eq!(vec![1, 2], Solution::count_servers(n, logs, x, queries))
    }

    #[test]
    fn case2() {
        let n = 3;
        let logs = [[2, 4], [2, 1], [1, 2], [3, 1]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        let x = 2;
        let queries = vec![3, 4];
        assert_eq!(vec![0, 1], Solution::count_servers(n, logs, x, queries))
    }
}
