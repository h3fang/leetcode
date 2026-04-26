pub struct Solution;

fn check(a: &[i64], f: &mut [i64], end: &mut [usize], low: i64, k: i64, side: i64) -> bool {
    let mut j = a.len();
    for (i, &x) in a.iter().enumerate().rev() {
        while a[j - 1] >= x + low {
            j -= 1;
        }
        f[i] = f[j] + 1;
        end[i] = if f[i] > 1 { end[j] } else { i };
        if f[i] == k && a[end[i]] - a[i] <= 4 * side - low {
            return true;
        }
    }
    false
}

fn tranform(points: Vec<Vec<i32>>, side: i64) -> Vec<i64> {
    points
        .into_iter()
        .map(|p| {
            let (x, y) = (p[0] as i64, p[1] as i64);
            if x == 0 {
                y
            } else if y == side {
                side + x
            } else if x == side {
                3 * side - y
            } else {
                4 * side - x
            }
        })
        .collect()
}

impl Solution {
    pub fn max_distance(side: i32, points: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = points.len();
        let (side, k) = (side as i64, k as i64);

        let mut a = tranform(points, side);
        a.sort_unstable();

        let mut f = vec![0; n + 1];
        let mut end = vec![0; n];

        let (mut l, mut r) = (1, (4 * side / k) + 1);
        while l + 1 < r {
            let m = l + (r - l) / 2;
            if check(&a, &mut f, &mut end, m, k, side) {
                l = m;
            } else {
                r = m;
            }
        }

        l as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let points = [[0, 2], [2, 0], [2, 2], [0, 0]]
            .iter()
            .map(|p| p.to_vec())
            .collect();
        assert_eq!(2, Solution::max_distance(2, points, 4));
    }

    #[test]
    fn case2() {
        let points = [[0, 0], [1, 2], [2, 0], [2, 2], [2, 1]]
            .iter()
            .map(|p| p.to_vec())
            .collect();
        assert_eq!(1, Solution::max_distance(2, points, 4));
    }

    #[test]
    fn case3() {
        let points = [[0, 0], [0, 1], [0, 2], [1, 2], [2, 0], [2, 2], [2, 1]]
            .iter()
            .map(|p| p.to_vec())
            .collect();
        assert_eq!(1, Solution::max_distance(2, points, 5));
    }
}
