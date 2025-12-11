pub struct Solution;

impl Solution {
    pub fn count_covered_buildings(n: i32, buildings: Vec<Vec<i32>>) -> i32 {
        let mut row = vec![(n + 1, 0); n as usize + 1];
        let mut col = vec![(n + 1, 0); n as usize + 1];
        for b in &buildings {
            row[b[0] as usize].0 = row[b[0] as usize].0.min(b[1]);
            row[b[0] as usize].1 = row[b[0] as usize].1.max(b[1]);
            col[b[1] as usize].0 = col[b[1] as usize].0.min(b[0]);
            col[b[1] as usize].1 = col[b[1] as usize].1.max(b[0]);
        }
        let mut ans = 0;
        for b in &buildings {
            if row[b[0] as usize].0 < b[1]
                && row[b[0] as usize].1 > b[1]
                && col[b[1] as usize].0 < b[0]
                && col[b[1] as usize].1 > b[0]
            {
                ans += 1;
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
        let buildings = [[1, 2], [2, 2], [3, 2], [2, 1], [2, 3]]
            .iter()
            .map(|b| b.to_vec())
            .collect();
        assert_eq!(1, Solution::count_covered_buildings(3, buildings));
    }

    #[test]
    fn case2() {
        let buildings = [[1, 1], [1, 2], [2, 1], [2, 2]]
            .iter()
            .map(|b| b.to_vec())
            .collect();
        assert_eq!(0, Solution::count_covered_buildings(3, buildings));
    }

    #[test]
    fn case3() {
        let buildings = [[1, 3], [3, 2], [3, 3], [3, 5], [5, 3]]
            .iter()
            .map(|b| b.to_vec())
            .collect();
        assert_eq!(1, Solution::count_covered_buildings(5, buildings));
    }
}
