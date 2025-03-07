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
        if x != self.parent[x] {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let (px, py) = (self.find(x), self.find(y));
        if px == py {
            return;
        }
        match self.size[px].cmp(&self.size[py]) {
            std::cmp::Ordering::Less => {
                self.parent[px] = py;
                self.size[py] += self.size[px];
            }
            _ => {
                self.parent[py] = px;
                self.size[px] += self.size[py];
            }
        }
    }
}

impl Solution {
    pub fn can_traverse_all_pairs(nums: Vec<i32>) -> bool {
        if nums.len() == 1 {
            return true;
        }
        let mut max = 1;
        for &x in &nums {
            if x == 1 {
                return false;
            }
            max = max.max(x);
        }
        let mut dsu = Dsu::new(max as usize + 1);
        for &x in &nums {
            let mut y = 2;
            while y * y <= x {
                if x % y == 0 {
                    dsu.union(x as usize, y as usize);
                    dsu.union(x as usize, (x / y) as usize);
                }
                y += 1;
            }
        }
        let root = dsu.find(nums[0] as usize);
        nums.into_iter()
            .skip(1)
            .all(|x| dsu.find(x as usize) == root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::can_traverse_all_pairs(vec![2, 3, 6]));
    }

    #[test]
    fn case2() {
        assert!(!Solution::can_traverse_all_pairs(vec![3, 9, 5]));
    }

    #[test]
    fn case3() {
        assert!(Solution::can_traverse_all_pairs(vec![4, 3, 12, 8]));
    }
}
