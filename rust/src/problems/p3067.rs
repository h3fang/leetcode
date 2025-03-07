pub struct Solution;

fn dfs(speed: i32, g: &[Vec<(i32, i32)>], parent: i32, x: i32, dist: i32) -> i32 {
    let mut result = i32::from(dist == 0);
    for &(y, w) in &g[x as usize] {
        if y == parent {
            continue;
        }
        result += dfs(speed, g, x, y, (dist + w) % speed);
    }
    result
}

impl Solution {
    pub fn count_pairs_of_connectable_servers(edges: Vec<Vec<i32>>, signal_speed: i32) -> Vec<i32> {
        let n = edges.len() + 1;
        let mut g = vec![vec![]; n];
        for e in edges {
            g[e[0] as usize].push((e[1], e[2]));
            g[e[1] as usize].push((e[0], e[2]));
        }
        (0..n as i32)
            .map(|c| {
                let (mut result, mut sum) = (0, 0);
                for &(x, w) in &g[c as usize] {
                    let f = dfs(signal_speed, &g, c, x, w % signal_speed);
                    result += f * sum;
                    sum += f;
                }
                result
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let edges = [[0, 1, 1], [1, 2, 5], [2, 3, 13], [3, 4, 9], [4, 5, 2]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(
            vec![0, 4, 6, 6, 4, 0],
            Solution::count_pairs_of_connectable_servers(edges, 1)
        );
    }

    #[test]
    fn case2() {
        let edges = [
            [0, 6, 3],
            [6, 5, 3],
            [0, 3, 1],
            [3, 2, 7],
            [3, 1, 6],
            [3, 4, 2],
        ]
        .iter()
        .map(|e| e.to_vec())
        .collect();
        assert_eq!(
            vec![2, 0, 0, 0, 0, 0, 2],
            Solution::count_pairs_of_connectable_servers(edges, 3)
        );
    }
}
