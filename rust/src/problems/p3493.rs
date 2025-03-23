pub struct Solution;

struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let (px, py) = (self.find(x), self.find(y));
        if px == py {
            return false;
        }
        match self.size[px].cmp(&self.size[py]) {
            std::cmp::Ordering::Less => {
                self.size[py] += self.size[px];
                self.parent[px] = py;
            }
            _ => {
                self.size[px] += self.size[py];
                self.parent[py] = px;
            }
        }
        true
    }
}

impl Solution {
    pub fn number_of_components(properties: Vec<Vec<i32>>, k: i32) -> i32 {
        let properties = properties
            .into_iter()
            .map(|p| p.into_iter().fold(0u128, |acc, x| acc | (1 << x)))
            .collect::<Vec<_>>();
        let n = properties.len();
        let mut dsu = Dsu::new(n);
        let mut ans = n as i32;
        for (i, a) in properties.iter().enumerate() {
            for (j, b) in properties.iter().enumerate().skip(i + 1) {
                if (a & b).count_ones() as i32 >= k && dsu.union(i, j) {
                    ans -= 1;
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
        let properties = [[1, 2], [1, 1], [3, 4], [4, 5], [5, 6], [7, 7]]
            .iter()
            .map(|p| p.to_vec())
            .collect();
        let k = 1;
        assert_eq!(3, Solution::number_of_components(properties, k));
    }

    #[test]
    fn case2() {
        let properties = [[1, 2, 3], [2, 3, 4], [4, 3, 5]]
            .iter()
            .map(|p| p.to_vec())
            .collect();
        let k = 2;
        assert_eq!(1, Solution::number_of_components(properties, k));
    }

    #[test]
    fn case3() {
        let properties = [[1, 1], [1, 1]].iter().map(|p| p.to_vec()).collect();
        let k = 2;
        assert_eq!(2, Solution::number_of_components(properties, k));
    }
}
