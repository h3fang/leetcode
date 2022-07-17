pub struct Solution;

impl Solution {
    pub fn minimum_score(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        let mut g = vec![vec![]; n];
        for e in &edges {
            g[e[0] as usize].push(e[1]);
            g[e[1] as usize].push(e[0]);
        }
        let mut xor = vec![0; n];
        let mut enter = vec![0; n];
        let mut leave = vec![0; n];

        #[allow(clippy::too_many_arguments)]
        fn dfs(
            nums: &[i32],
            g: &[Vec<i32>],
            xor: &mut [i32],
            enter: &mut [i32],
            leave: &mut [i32],
            i: i32,
            pa: i32,
            t: &mut i32,
        ) -> i32 {
            *t += 1;
            let mut r = nums[i as usize];
            enter[i as usize] = *t;
            for &c in &g[i as usize] {
                if c == pa {
                    continue;
                }
                r ^= dfs(nums, g, xor, enter, leave, c, i, t);
            }
            leave[i as usize] = *t;
            xor[i as usize] = r;
            r
        }

        dfs(&nums, &g, &mut xor, &mut enter, &mut leave, 0, -1, &mut 0);

        let mut result = i32::MAX;
        for i in 1..n {
            for j in i + 1..n {
                let (x, y, z) = if enter[i] < enter[j] && enter[j] <= leave[i] {
                    // i is parent of j
                    (xor[j], xor[i] ^ xor[j], xor[0] ^ xor[i])
                } else if enter[j] < enter[i] && enter[i] <= leave[j] {
                    // j is parent of i
                    (xor[i], xor[i] ^ xor[j], xor[0] ^ xor[j])
                } else {
                    // i and j are in different subtrees
                    (xor[i], xor[j], xor[0] ^ xor[i] ^ xor[j])
                };
                let min = x.min(y.min(z));
                let max = x.max(y.max(z));
                result = result.min(max - min);
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
        let nums = vec![1, 5, 5, 4, 11];
        let edges = [[0, 1], [1, 2], [1, 3], [3, 4]];
        let edges = edges.iter().map(|e| e.to_vec()).collect();
        assert_eq!(9, Solution::minimum_score(nums, edges));
    }

    #[test]
    fn case2() {
        let nums = vec![5, 5, 2, 4, 4, 2];
        let edges = [[0, 1], [1, 2], [5, 2], [4, 3], [1, 3]];
        let edges = edges.iter().map(|e| e.to_vec()).collect();
        assert_eq!(0, Solution::minimum_score(nums, edges));
    }
}
