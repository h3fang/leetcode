pub struct Solution;

fn build(l: usize, r: usize, i: usize, heights: &Vec<i32>, tree: &mut Vec<i32>) {
    if l == r {
        tree[i] = heights[l - 1];
        return;
    }

    let mid = (l + r) / 2;
    build(l, mid, i * 2, heights, tree);
    build(mid + 1, r, i * 2 + 1, heights, tree);
    tree[i] = tree[i * 2].max(tree[i * 2 + 1]);
}

fn query(pos: usize, val: i32, l: usize, r: usize, i: usize, tree: &Vec<i32>) -> i32 {
    if val >= tree[i] {
        return 0;
    }

    if l == r {
        return l as i32;
    }

    let mid = (l + r) / 2;
    if pos <= mid {
        let ans = query(pos, val, l, mid, i * 2, tree);
        if ans != 0 {
            return ans;
        }
    }
    query(pos, val, mid + 1, r, i * 2 + 1, tree)
}

impl Solution {
    pub fn leftmost_building_queries(heights: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = heights.len();
        let mut tree = vec![0; n * 4];
        build(1, n, 1, &heights, &mut tree);

        queries
            .into_iter()
            .map(|q| {
                let (mut a, mut b) = (q[0] as usize, q[1] as usize);
                if a > b {
                    (a, b) = (b, a);
                }
                if a == b || heights[a] < heights[b] {
                    b as i32
                } else {
                    query(b + 1, heights[a], 1, n, 1, &tree) - 1
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let queries = [[0, 1], [0, 3], [2, 4], [3, 4], [2, 2]]
            .iter()
            .map(|q| q.to_vec())
            .collect();
        assert_eq!(
            vec![2, 5, -1, 5, 2],
            Solution::leftmost_building_queries(vec![6, 4, 8, 5, 2, 7], queries)
        );
    }

    #[test]
    fn case2() {
        let queries = [[0, 7], [3, 5], [5, 2], [3, 0], [1, 6]]
            .iter()
            .map(|q| q.to_vec())
            .collect();
        assert_eq!(
            vec![7, 6, -1, 4, 6],
            Solution::leftmost_building_queries(vec![5, 3, 8, 2, 6, 1, 4, 6], queries)
        );
    }
}
