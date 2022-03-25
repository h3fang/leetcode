pub struct Solution;

impl Solution {
    pub fn two_city_sched_cost(mut costs: Vec<Vec<i32>>) -> i32 {
        let n = costs.len() / 2;
        costs.select_nth_unstable_by_key(n, |e| e[0] - e[1]);
        let mut result = 0;
        for i in 0..n {
            result += costs[i][0] + costs[i + n][1];
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let costs = [[10, 20], [30, 200], [400, 50], [30, 20]];
        let costs = costs.iter().map(|c| c.to_vec()).collect();
        assert_eq!(110, Solution::two_city_sched_cost(costs));
    }

    #[test]
    fn case2() {
        let costs = [
            [259, 770],
            [448, 54],
            [926, 667],
            [184, 139],
            [840, 118],
            [577, 469],
        ];
        let costs = costs.iter().map(|c| c.to_vec()).collect();
        assert_eq!(1859, Solution::two_city_sched_cost(costs));
    }

    #[test]
    fn case3() {
        let costs = [
            [515, 563],
            [451, 713],
            [537, 709],
            [343, 819],
            [855, 779],
            [457, 60],
            [650, 359],
            [631, 42],
        ];
        let costs = costs.iter().map(|c| c.to_vec()).collect();
        assert_eq!(3086, Solution::two_city_sched_cost(costs));
    }
}
