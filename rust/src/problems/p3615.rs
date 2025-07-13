pub struct Solution;

fn dfs(
    g: &[Vec<u8>],
    label: &[u8],
    (x, y, vis): (u8, u8, u16),
    cache: &mut Vec<Vec<Vec<i32>>>,
) -> i32 {
    let r = cache[x as usize][y as usize][vis as usize];
    if r >= 0 {
        return r;
    }
    let mut ans = 0;
    for &v in &g[x as usize] {
        if (vis >> v) & 1 == 1 {
            continue;
        }
        for &w in &g[y as usize] {
            if (vis >> w) & 1 == 0 && v != w && label[v as usize] == label[w as usize] {
                let (v, w) = if v > w { (w, v) } else { (v, w) };
                let r = dfs(g, label, (v, w, vis | 1 << v | 1 << w), cache);
                ans = ans.max(r + 2);
            }
        }
    }
    cache[x as usize][y as usize][vis as usize] = ans;
    ans
}

impl Solution {
    pub fn max_len(n: i32, edges: Vec<Vec<i32>>, label: String) -> i32 {
        let label = label.as_bytes();
        let mut g = vec![vec![]; n as usize];
        for e in edges {
            g[e[0] as usize].push(e[1] as u8);
            g[e[1] as usize].push(e[0] as u8);
        }
        let mut ans = 0;
        let n = n as usize;
        let mut cache = vec![vec![vec![-1; 1 << n]; n]; n];
        for (x, next) in g.iter().enumerate() {
            let x = x as u8;
            let r = dfs(&g, label, (x, x, 1 << x), &mut cache);
            ans = ans.max(r + 1);
            for &y in next {
                if x < y && label[x as usize] == label[y as usize] {
                    let r = dfs(&g, label, (x, y, 1 << x | 1 << y), &mut cache);
                    ans = ans.max(r + 2);
                }
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
        let n = 3;
        let edges = [[0, 1], [1, 2]].iter().map(|e| e.to_vec()).collect();
        let label = "aba".to_string();
        assert_eq!(3, Solution::max_len(n, edges, label));
    }

    #[test]
    fn case2() {
        let n = 3;
        let edges = [[0, 1], [0, 2]].iter().map(|e| e.to_vec()).collect();
        let label = "abc".to_string();
        assert_eq!(1, Solution::max_len(n, edges, label));
    }

    #[test]
    fn case3() {
        let n = 4;
        let edges = [[0, 2], [0, 3], [3, 1]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        let label = "bbac".to_string();
        assert_eq!(3, Solution::max_len(n, edges, label));
    }
}
