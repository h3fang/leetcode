pub struct Solution;

const INF: i32 = i32::MAX / 2;

impl Solution {
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<char>,
        changed: Vec<char>,
        cost: Vec<i32>,
    ) -> i64 {
        let mut dist = [[INF; 26]; 26];
        for (i, d) in dist.iter_mut().enumerate() {
            d[i] = 0;
        }
        for ((x, y), d) in original.into_iter().zip(changed).zip(cost) {
            let (i, j) = ((x as u8 - b'a') as usize, (y as u8 - b'a') as usize);
            dist[i][j] = dist[i][j].min(d);
        }
        for k in 0..26 {
            for i in 0..26 {
                for j in 0..26 {
                    dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                }
            }
        }
        let mut result = 0;
        for (x, y) in source.as_bytes().iter().zip(target.as_bytes()) {
            let (i, j) = ((x - b'a') as usize, (y - b'a') as usize);
            if dist[i][j] == INF {
                return -1;
            }
            result += dist[i][j] as i64;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let source = "abcd".to_string();
        let target = "acbe".to_string();
        let original = ["a", "b", "c", "c", "e", "d"]
            .iter()
            .map(|x| x.chars().next().unwrap())
            .collect();
        let changed = ["b", "c", "b", "e", "b", "e"]
            .iter()
            .map(|x| x.chars().next().unwrap())
            .collect();
        let cost = vec![2, 5, 5, 1, 2, 20];
        assert_eq!(
            28,
            Solution::minimum_cost(source, target, original, changed, cost)
        );
    }

    #[test]
    fn case2() {
        let source = "aaaa".to_string();
        let target = "bbbb".to_string();
        let original = ["a", "c"]
            .iter()
            .map(|x| x.chars().next().unwrap())
            .collect();
        let changed = ["c", "b"]
            .iter()
            .map(|x| x.chars().next().unwrap())
            .collect();
        let cost = vec![1, 2];
        assert_eq!(
            12,
            Solution::minimum_cost(source, target, original, changed, cost)
        );
    }

    #[test]
    fn case3() {
        let source = "abcd".to_string();
        let target = "abce".to_string();
        let original = ["a"].iter().map(|x| x.chars().next().unwrap()).collect();
        let changed = ["e"].iter().map(|x| x.chars().next().unwrap()).collect();
        let cost = vec![10000];
        assert_eq!(
            -1,
            Solution::minimum_cost(source, target, original, changed, cost)
        );
    }
}
