pub struct Solution;

impl Solution {
    pub fn hardest_worker(_n: i32, logs: Vec<Vec<i32>>) -> i32 {
        let mut last = 0;
        let mut max = -1;
        let mut result = -1;
        for l in logs {
            let t = l[1] - last;
            last = l[1];
            match t.cmp(&max) {
                std::cmp::Ordering::Equal => result = result.min(l[0]),
                std::cmp::Ordering::Greater => {
                    max = t;
                    result = l[0];
                }
                _ => {}
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 10;
        let logs = [[0, 3], [2, 5], [0, 9], [1, 15]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(1, Solution::hardest_worker(n, logs));
    }

    #[test]
    fn case2() {
        let n = 26;
        let logs = [[1, 1], [3, 7], [2, 12], [7, 17]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(3, Solution::hardest_worker(n, logs));
    }

    #[test]
    fn case3() {
        let n = 2;
        let logs = [[0, 10], [1, 20]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(0, Solution::hardest_worker(n, logs));
    }
}
