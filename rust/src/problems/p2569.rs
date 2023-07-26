pub struct Solution;

fn build(i: usize, l: usize, r: usize, seg: &mut [i32], nums1: &[i32]) {
    if l == r {
        seg[i] = nums1[l - 1];
    } else {
        let m = (l + r) / 2;
        build(2 * i, l, m, seg, nums1);
        build(2 * i + 1, m + 1, r, seg, nums1);
        seg[i] = seg[2 * i] + seg[2 * i + 1];
    }
}

fn flip(i: usize, l: usize, r: usize, seg: &mut [i32], lazy: &mut [bool]) {
    seg[i] = (r - l + 1) as i32 - seg[i];
    lazy[i] = !lazy[i];
}

fn update(i: usize, l: usize, r: usize, lb: usize, ub: usize, seg: &mut [i32], lazy: &mut [bool]) {
    if lb <= l && ub >= r {
        flip(i, l, r, seg, lazy);
    } else {
        let m = (l + r) / 2;
        if lazy[i] {
            flip(2 * i, l, m, seg, lazy);
            flip(2 * i + 1, m + 1, r, seg, lazy);
            lazy[i] = false;
        }
        if m >= lb {
            update(2 * i, l, m, lb, ub, seg, lazy);
        }
        if m < ub {
            update(2 * i + 1, m + 1, r, lb, ub, seg, lazy);
        }
        seg[i] = seg[2 * i] + seg[2 * i + 1];
    }
}

impl Solution {
    pub fn handle_query(nums1: Vec<i32>, nums2: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i64> {
        let n = nums1.len();
        let mut seg = vec![0; 4 * n];
        let mut lazy = vec![false; 4 * n];

        build(1, 1, n, &mut seg, &nums1);

        let mut res = vec![];
        let mut sum = nums2.iter().fold(0_i64, |a, &b| a + b as i64);
        for q in queries {
            let (op, l, r) = (q[0], q[1], q[2]);
            match op {
                1 => update(1, 1, n, l as usize + 1, r as usize + 1, &mut seg, &mut lazy),
                2 => sum += l as i64 * seg[1] as i64,
                3 => res.push(sum),
                _ => (),
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let queries = [[1, 1, 1], [2, 1, 0], [3, 0, 0]]
            .iter()
            .map(|q| q.to_vec())
            .collect();
        assert_eq!(
            vec![3],
            Solution::handle_query(vec![1, 0, 1], vec![0, 0, 0], queries)
        );
    }

    #[test]
    fn case2() {
        let queries = [[2, 0, 0], [3, 0, 0]].iter().map(|q| q.to_vec()).collect();
        assert_eq!(vec![5], Solution::handle_query(vec![1], vec![5], queries));
    }
}
