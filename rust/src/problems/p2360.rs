pub struct Solution;

impl Solution {
    pub fn longest_cycle(edges: Vec<i32>) -> i32 {
        let n = edges.len();
        let mut vis = vec![-1; n];
        let mut result = -1;
        for node in 0..n {
            if vis[node] >= 0 {
                continue;
            }
            let mut curr = node as i32;
            while curr != -1 {
                if vis[curr as usize] == node as i32 {
                    let mut size = 1;
                    let mut head = curr;
                    while edges[head as usize] != curr {
                        head = edges[head as usize];
                        size += 1;
                    }
                    result = result.max(size);
                    break;
                } else if vis[curr as usize] >= 0 {
                    break;
                }
                vis[curr as usize] = node as i32;
                curr = edges[curr as usize];
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
        assert_eq!(3, Solution::longest_cycle(vec![3, 3, 4, 2, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(-1, Solution::longest_cycle(vec![2, -1, 3, 1]));
    }
}
