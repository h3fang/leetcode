pub struct Solution;

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut g = vec![(0, 0); n as usize + 1];
        g[0] = (-1, -1);
        for t in trust {
            g[t[0] as usize].1 += 1;
            g[t[1] as usize].0 += 1;
        }
        g.iter()
            .position(|p| *p == (n - 1, 0))
            .map(|p| p as i32)
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 3;
        let trust = [[1, 3], [2, 3], [3, 1]];
        let trust = trust.iter().map(|t| t.to_vec()).collect();
        assert_eq!(-1, Solution::find_judge(n, trust));
    }

    #[test]
    fn case2() {
        let n = 4;
        let trust = [[1, 3], [1, 4], [2, 3], [2, 4], [4, 3]];
        let trust = trust.iter().map(|t| t.to_vec()).collect();
        assert_eq!(3, Solution::find_judge(n, trust));
    }
}
