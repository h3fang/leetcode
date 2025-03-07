pub struct Solution;

impl Solution {
    pub fn minimum_total_price(
        n: i32,
        edges: Vec<Vec<i32>>,
        price: Vec<i32>,
        trips: Vec<Vec<i32>>,
    ) -> i32 {
        let n = n as usize;
        let mut g = vec![vec![]; n];
        for e in &edges {
            let x = e[0] as usize;
            let y = e[1] as usize;
            g[x].push(y);
            g[y].push(x);
        }

        fn dfs(x: usize, fa: usize, cnt: &mut Vec<i32>, g: &Vec<Vec<usize>>, end: usize) -> bool {
            if x == end {
                cnt[x] += 1;
                return true;
            }

            for &y in &g[x] {
                if y != fa && dfs(y, x, cnt, g, end) {
                    cnt[x] += 1;
                    return true;
                }
            }
            false
        }

        let mut cnt = vec![0; n];
        for t in &trips {
            dfs(t[0] as usize, n, &mut cnt, &g, t[1] as usize);
        }

        fn dp(
            x: usize,
            fa: usize,
            price: &Vec<i32>,
            cnt: &Vec<i32>,
            g: &Vec<Vec<usize>>,
        ) -> (i32, i32) {
            let mut not_halve = price[x] * cnt[x];
            let mut halve = not_halve / 2;
            for &y in &g[x] {
                if y != fa {
                    let (nh, h) = dp(y, x, price, cnt, g);
                    not_halve += nh.min(h);
                    halve += nh;
                }
            }
            (not_halve, halve)
        }
        let (nh, h) = dp(0, 0, &price, &cnt, &g);
        nh.min(h)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let edges = [[0, 1], [1, 2], [1, 3]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        let price = vec![2, 2, 10, 6];
        let trips = [[0, 3], [2, 1], [2, 3]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(23, Solution::minimum_total_price(4, edges, price, trips));
    }

    #[test]
    fn case2() {
        let edges = [[0, 1]].iter().map(|e| e.to_vec()).collect();
        let price = vec![2, 2];
        let trips = [[0, 0]].iter().map(|e| e.to_vec()).collect();
        assert_eq!(1, Solution::minimum_total_price(2, edges, price, trips));
    }
}
