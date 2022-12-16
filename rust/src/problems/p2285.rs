pub struct Solution;

impl Solution {
    pub fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
        let mut degrees = vec![0; n as usize];
        for r in roads {
            degrees[r[0] as usize] += 1;
            degrees[r[1] as usize] += 1;
        }
        let mut degrees = degrees
            .into_iter()
            .enumerate()
            .map(|(i, d)| (d, i))
            .collect::<Vec<_>>();
        degrees.sort_unstable();
        degrees
            .into_iter()
            .enumerate()
            .rev()
            .fold(0i64, |acc, (i, (d, _))| acc + d as i64 * (i + 1) as i64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_roads(roads: &[[i32; 2]]) -> Vec<Vec<i32>> {
        roads.iter().map(|r| r.to_vec()).collect()
    }

    #[test]
    fn case1() {
        let roads = parse_roads(&[[0, 1], [1, 2], [2, 3], [0, 2], [1, 3], [2, 4]]);
        assert_eq!(43, Solution::maximum_importance(5, roads));
    }
}
