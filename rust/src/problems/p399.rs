use std::collections::HashMap;

pub struct Solution;

struct DisjointUnionSet {
    parent: Vec<usize>,
    weight: Vec<f64>,
}

impl DisjointUnionSet {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            weight: vec![1.0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let origin = self.parent[x];
            self.parent[x] = self.find(origin);
            self.weight[x] *= self.weight[origin];
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize, v: f64) {
        let px = self.find(x);
        let py = self.find(y);
        if px == py {
            return;
        }
        self.parent[px] = py;
        self.weight[px] = v * self.weight[y] / self.weight[x];
    }

    fn get_value(&mut self, x: usize, y: usize) -> f64 {
        let px = self.find(x);
        let py = self.find(y);
        if px == py {
            self.weight[x] / self.weight[y]
        } else {
            -1.0
        }
    }
}

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut map = HashMap::new();
        let mut id = 0;
        let mut dsu = DisjointUnionSet::new(2 * equations.len());
        for (s, &v) in equations.iter().zip(&values) {
            let x = *map.entry(s[0].to_string()).or_insert_with(|| {
                let i = id;
                id += 1;
                i
            });
            let y = *map.entry(s[1].to_string()).or_insert_with(|| {
                let i = id;
                id += 1;
                i
            });
            dsu.union(x, y, v);
        }

        queries
            .iter()
            .map(|q| {
                if let Some(&x) = map.get(&q[0])
                    && let Some(&y) = map.get(&q[1])
                {
                    return dsu.get_value(x, y);
                }
                -1.0
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_equations(eqns: &[[&str; 2]]) -> Vec<Vec<String>> {
        eqns.iter()
            .map(|e| e.iter().map(|s| s.to_string()).collect())
            .collect()
    }

    fn assert_close(a: &[f64], b: &[f64]) {
        println!("{a:?}");
        println!("{b:?}");
        assert_eq!(a.len(), b.len());
        for (a, b) in a.iter().zip(b) {
            assert!((a - b).abs() < 1e-5);
        }
    }

    #[test]
    fn case1() {
        let equations = parse_equations(&[["a", "b"], ["b", "c"]]);
        let values = vec![2.0, 3.0];
        let queries =
            parse_equations(&[["a", "c"], ["b", "a"], ["a", "e"], ["a", "a"], ["x", "x"]]);
        let expected = [6.00000, 0.50000, -1.00000, 1.00000, -1.00000];
        assert_close(
            &expected,
            &Solution::calc_equation(equations, values, queries),
        );
    }

    #[test]
    fn case2() {
        let equations = parse_equations(&[["a", "b"], ["b", "c"], ["bc", "cd"]]);
        let values = vec![1.5, 2.5, 5.0];
        let queries = parse_equations(&[["a", "c"], ["c", "b"], ["bc", "cd"], ["cd", "bc"]]);
        let expected = [3.75000, 0.40000, 5.00000, 0.20000];
        assert_close(
            &expected,
            &Solution::calc_equation(equations, values, queries),
        );
    }

    #[test]
    fn case3() {
        let equations = parse_equations(&[["a", "b"]]);
        let values = vec![0.5];
        let queries = parse_equations(&[["a", "b"], ["b", "a"], ["a", "c"], ["x", "y"]]);
        let expected = [0.50000, 2.00000, -1.00000, -1.00000];
        assert_close(
            &expected,
            &Solution::calc_equation(equations, values, queries),
        );
    }
}
