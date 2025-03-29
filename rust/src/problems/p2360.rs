pub struct Solution;

impl Solution {
    pub fn longest_cycle(edges: Vec<i32>) -> i32 {
        let n = edges.len();
        let mut vis = vec![0; n];
        let (mut ans, mut t) = (-1, 1);
        for mut node in 0..n as i32 {
            let t0 = t;
            while node != -1 && vis[node as usize] == 0 {
                vis[node as usize] = t;
                t += 1;
                node = edges[node as usize];
            }
            if node != -1 && vis[node as usize] >= t0 {
                ans = ans.max(t - vis[node as usize]);
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::longest_cycle(vec![3, 3, 4, 2, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(-1, Solution::longest_cycle(vec![2, -1, 3, 1]));
    }
}
