pub struct Solution;

pub struct TreeAncestor {
    ancestors: Vec<Vec<i32>>,
}

impl TreeAncestor {
    pub fn new(n: i32, parent: Vec<i32>) -> Self {
        let h = 32 - n.leading_zeros() as usize;
        let mut ancestors = vec![vec![-1; h]; n as usize];
        for (i, &p) in parent.iter().enumerate() {
            ancestors[i][0] = p;
        }
        for j in 1..h {
            for i in 0..n as usize {
                if ancestors[i][j - 1] != -1 {
                    ancestors[i][j] = ancestors[ancestors[i][j - 1] as usize][j - 1];
                }
            }
        }
        Self { ancestors }
    }

    pub fn get_kth_ancestor(&self, mut node: i32, k: i32) -> i32 {
        let n = 32 - k.leading_zeros() as usize;
        for j in 0..n {
            if (k >> j) & 1 == 1 {
                node = self.ancestors[node as usize][j];
                if node == -1 {
                    break;
                }
            }
        }
        node
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let ta = TreeAncestor::new(7, vec![-1, 0, 0, 1, 1, 2, 2]);
        assert_eq!(1, ta.get_kth_ancestor(3, 1));
        assert_eq!(0, ta.get_kth_ancestor(5, 2));
        assert_eq!(-1, ta.get_kth_ancestor(6, 3));
    }
}
