pub struct Solution;

impl Solution {
    pub fn number_of_sets(n: i32, max_distance: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut result = 0;
        for m in 0..(1 << n) {
            let mut vertices = vec![false; n];
            for (i, v) in vertices.iter_mut().enumerate() {
                if m & (1 << i) > 0 {
                    *v = true;
                }
            }
            let mut dist = vec![vec![i32::MAX / 2; n]; n];
            for r in &roads {
                if !vertices[r[0] as usize] || !vertices[r[1] as usize] {
                    continue;
                }
                let d = dist[r[0] as usize][r[1] as usize].min(r[2]);
                dist[r[0] as usize][r[1] as usize] = d;
                dist[r[1] as usize][r[0] as usize] = d;
            }
            for k in 0..n {
                if !vertices[k] {
                    continue;
                }
                for i in 0..n {
                    if !vertices[i] {
                        continue;
                    }
                    for (j, v) in vertices.iter().enumerate().skip(i + 1) {
                        if !*v {
                            continue;
                        }
                        if dist[i][j] > dist[i][k] + dist[k][j] {
                            dist[i][j] = dist[i][k] + dist[k][j];
                            dist[j][i] = dist[i][j];
                        }
                    }
                }
            }
            let mut valid = true;
            for i in 0..n {
                if !valid {
                    break;
                }
                for j in i + 1..n {
                    if vertices[i] && vertices[j] && dist[i][j] > max_distance {
                        valid = false;
                        break;
                    }
                }
            }
            if valid {
                result += 1;
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
        let roads = [[0, 1, 2], [1, 2, 10], [0, 2, 10]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(5, Solution::number_of_sets(3, 5, roads));
    }

    #[test]
    fn case2() {
        let roads = [[0, 1, 20], [0, 1, 10], [1, 2, 2], [0, 2, 2]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(7, Solution::number_of_sets(3, 5, roads));
    }

    #[test]
    fn case3() {
        assert_eq!(2, Solution::number_of_sets(1, 10, vec![]));
    }
}
