pub struct Solution;

impl Solution {
    pub fn minimum_fuel_cost(roads: Vec<Vec<i32>>, seats: i32) -> i64 {
        let mut result = 0;
        let mut g = vec![vec![]; roads.len() + 1];
        for r in roads {
            g[r[0] as usize].push(r[1]);
            g[r[1] as usize].push(r[0]);
        }
        fn dfs(x: i32, p: i32, g: &[Vec<i32>], seats: i64, result: &mut i64) -> i64 {
            let mut size = 1;
            for &y in &g[x as usize] {
                if y != p {
                    size += dfs(y, x, g, seats, result);
                }
            }
            if x > 0 {
                *result += (size + seats - 1) / seats;
            }
            size
        }
        dfs(0, -1, &g, seats as i64, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let roads = [[0, 1], [0, 2], [0, 3]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(3, Solution::minimum_fuel_cost(roads, 5));
    }

    #[test]
    fn case2() {
        let roads = [[3, 1], [3, 2], [1, 0], [0, 4], [0, 5], [4, 6]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(7, Solution::minimum_fuel_cost(roads, 2));
    }

    #[test]
    fn case3() {
        let roads = vec![];
        assert_eq!(0, Solution::minimum_fuel_cost(roads, 1));
    }
}
