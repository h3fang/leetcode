pub struct Solution;

struct SparseTable {
    t: Vec<Vec<i32>>,
}
impl SparseTable {
    fn new(a: &[(i32, i32)]) -> Self {
        let n = a.len() - 1;
        let w = usize::BITS - n.leading_zeros();
        let mut t = vec![vec![0; n]; w as usize];

        for (j, t) in t[0].iter_mut().enumerate() {
            *t = a[j].1 - a[j].0 + a[j + 1].1 - a[j + 1].0;
        }

        for i in 1..t.len() {
            let mut j = 0;
            while j + (1 << i) <= n {
                t[i][j] = t[i - 1][j].max(t[i - 1][j + (1 << (i - 1))]);
                j += 1;
            }
        }

        Self { t }
    }

    fn query(&self, l: usize, r: usize) -> i32 {
        if l >= r {
            return 0;
        }

        let k = (usize::BITS - (r - l).leading_zeros() - 1) as usize;
        self.t[k][l].max(self.t[k][r - (1 << k)])
    }
}

impl Solution {
    pub fn max_active_sections_after_trade(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let s = s.as_bytes();
        let n = s.len();
        let mut ones = 0;
        let mut start = 0;
        let mut a = Vec::with_capacity(n);

        a.push((-1, -1));
        for (i, &b) in s.iter().enumerate() {
            if i + 1 == n || b != s[i + 1] {
                if b == b'1' {
                    ones += (i - start + 1) as i32;
                } else {
                    a.push((start as i32, i as i32 + 1));
                }
                start = i + 1;
            }
        }
        a.push((n as i32 + 1, n as i32 + 1));

        let merge = |x: i32, y: i32| {
            if x > 0 && y > 0 { x + y } else { 0 }
        };

        let st = SparseTable::new(&a);

        queries
            .into_iter()
            .map(|q| {
                let (ql, qr) = (q[0], q[1] + 1);
                let i = a.partition_point(|p| p.0 < ql);
                let j = a.partition_point(|p| p.1 <= qr) - 1;

                let max = if i <= j {
                    st.query(i, j)
                        .max(merge(a[i - 1].1 - ql, a[i].1 - a[i].0))
                        .max(merge(qr - a[j + 1].0, a[j].1 - a[j].0))
                } else if i == j + 1 {
                    merge(a[i - 1].1 - ql, qr - a[j + 1].0)
                } else {
                    0
                };

                ones + max
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let queries = [[0, 1]].iter().map(|q| q.to_vec()).collect();
        assert_eq!(
            vec![1],
            Solution::max_active_sections_after_trade("01".to_string(), queries)
        );
    }

    #[test]
    fn case2() {
        let queries = [[0, 3], [0, 2], [1, 3], [2, 3]]
            .iter()
            .map(|q| q.to_vec())
            .collect();
        assert_eq!(
            vec![4, 3, 1, 1],
            Solution::max_active_sections_after_trade("0100".to_string(), queries)
        );
    }

    #[test]
    fn case3() {
        let queries = [[1, 5], [0, 6], [0, 4]]
            .iter()
            .map(|q| q.to_vec())
            .collect();
        assert_eq!(
            vec![6, 7, 2],
            Solution::max_active_sections_after_trade("1000100".to_string(), queries)
        );
    }

    #[test]
    fn case4() {
        let queries = [[0, 3], [1, 4], [1, 3]]
            .iter()
            .map(|q| q.to_vec())
            .collect();
        assert_eq!(
            vec![4, 4, 2],
            Solution::max_active_sections_after_trade("01010".to_string(), queries)
        );
    }
}
