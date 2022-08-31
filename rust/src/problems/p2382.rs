pub struct Solution;

struct Dsu {
    parent: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n + 1).collect(),
        }
    }

    fn find(&mut self, x: usize) -> usize {
        let p = self.parent[x];
        if x != p {
            self.parent[x] = self.find(p);
        }
        self.parent[x]
    }
}

impl Solution {
    pub fn maximum_segment_sum(nums: Vec<i32>, remove_queries: Vec<i32>) -> Vec<i64> {
        let n = nums.len();
        let mut dsu = Dsu::new(n);
        let mut sum = vec![0; n + 1];
        let mut result = vec![0; n];
        for (i, q) in remove_queries.into_iter().enumerate().skip(1).rev() {
            let x = q as usize;
            let r = dsu.find(x + 1);
            dsu.parent[x] = r;
            sum[r] += sum[x] + nums[x] as i64;
            result[i - 1] = result[i].max(sum[r]);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![14, 7, 2, 2, 0],
            Solution::maximum_segment_sum(vec![1, 2, 5, 6, 1], vec![0, 3, 2, 4, 1])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![16, 5, 3, 0],
            Solution::maximum_segment_sum(vec![3, 2, 11, 1], vec![3, 2, 1, 0])
        );
    }
}
