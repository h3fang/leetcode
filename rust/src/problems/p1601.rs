pub struct Solution;

impl Solution {
    pub fn maximum_requests(n: i32, requests: Vec<Vec<i32>>) -> i32 {
        fn dfs(
            requests: &[Vec<i32>],
            delta: &mut [i32],
            mut zero: i32,
            count: i32,
            result: &mut i32,
        ) {
            if zero == delta.len() as i32 {
                *result = (*result).max(count);
            }
            if requests.is_empty() {
                return;
            }
            dfs(&requests[1..], delta, zero, count, result);

            let from = requests[0][0] as usize;
            if delta[from] == 0 {
                zero -= 1;
            }
            delta[from] -= 1;
            if delta[from] == 0 {
                zero += 1;
            }

            let to = requests[0][1] as usize;
            if delta[to] == 0 {
                zero -= 1;
            }
            delta[to] += 1;
            if delta[to] == 0 {
                zero += 1;
            }

            dfs(&requests[1..], delta, zero, count + 1, result);
            delta[from] += 1;
            delta[to] -= 1;
        }

        let mut result = 0;
        let mut delta = vec![0; n as usize];
        dfs(&requests, &mut delta, n, 0, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 5;
        let requests = [[0, 1], [1, 0], [0, 1], [1, 2], [2, 0], [3, 4]];
        let requests = requests.iter().map(|r| r.to_vec()).collect();
        assert_eq!(5, Solution::maximum_requests(n, requests));
    }

    #[test]
    fn case2() {
        let n = 3;
        let requests = [[0, 0], [1, 2], [2, 1]];
        let requests = requests.iter().map(|r| r.to_vec()).collect();
        assert_eq!(3, Solution::maximum_requests(n, requests));
    }

    #[test]
    fn case3() {
        let n = 4;
        let requests = [[0, 3], [3, 1], [1, 2], [2, 0]];
        let requests = requests.iter().map(|r| r.to_vec()).collect();
        assert_eq!(4, Solution::maximum_requests(n, requests));
    }
}
