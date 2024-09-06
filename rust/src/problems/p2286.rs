pub struct BookMyShow {
    n: i32,
    m: i32,
    min: Vec<i32>,
    sum: Vec<i64>,
}

impl BookMyShow {
    pub fn new(n: i32, m: i32) -> Self {
        let min = vec![0; n as usize * 4];
        let sum = vec![0; n as usize * 4];
        Self { m, n, min, sum }
    }

    fn add(&mut self, idx: i32, val: i32) {
        fn helper(st: &mut BookMyShow, idx: i32, val: i32, l: i32, r: i32, node: usize) {
            if l == r {
                st.min[node] += val;
                st.sum[node] += val as i64;
                return;
            }
            let mid = l + (r - l) / 2;
            if idx <= mid {
                helper(st, idx, val, l, mid, node * 2);
            } else {
                helper(st, idx, val, mid + 1, r, node * 2 + 1);
            }
            st.min[node] = st.min[node * 2].min(st.min[node * 2 + 1]);
            st.sum[node] = st.sum[node * 2] + st.sum[node * 2 + 1];
        }
        helper(self, idx, val, 0, self.n - 1, 1);
    }

    fn query(&self, l: i32, r: i32) -> i64 {
        fn helper(st: &BookMyShow, l: i32, r: i32, lb: i32, rb: i32, node: usize) -> i64 {
            if lb >= l && rb <= r {
                return st.sum[node];
            }
            let mut result = 0;
            let mid = lb + (rb - lb) / 2;
            if l <= mid {
                result += helper(st, l, r, lb, mid, node * 2);
            }
            if mid < r {
                result += helper(st, l, r, mid + 1, rb, node * 2 + 1);
            }
            result
        }
        helper(self, l, r, 0, self.n - 1, 1)
    }

    fn index(&self, max_row: i32, l: i32, r: i32, node: usize, val: i32) -> i32 {
        if self.min[node] > val {
            return -1;
        }
        if l == r {
            if l <= max_row {
                return l;
            } else {
                return -1;
            }
        }
        let mid = l + (r - l) / 2;
        if self.min[node * 2] <= val {
            return self.index(max_row, l, mid, node * 2, val);
        }
        if mid < max_row {
            return self.index(max_row, mid + 1, r, node * 2 + 1, val);
        }
        -1
    }

    pub fn gather(&mut self, k: i32, max_row: i32) -> Vec<i32> {
        let i = self.index(max_row, 0, self.n - 1, 1, self.m - k);
        if i == -1 {
            return vec![];
        }
        let seats = self.query(i, i);
        self.add(i, k);
        vec![i, seats as i32]
    }

    pub fn scatter(&mut self, mut k: i32, max_row: i32) -> bool {
        let total = self.m as i64 * (max_row as i64 + 1);
        if (total - self.query(0, max_row)) < (k as i64) {
            return false;
        }
        let mut i = self.index(max_row, 0, self.n - 1, 1, self.m - 1);
        loop {
            let empty = self.m - self.query(i, i) as i32;
            if k <= empty {
                self.add(i, k);
                return true;
            }
            k -= empty;
            self.add(i, empty);
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut bms = BookMyShow::new(5, 9);
        assert_eq!(vec![0; 0], bms.gather(10, 1));
        assert!(bms.scatter(3, 3));
        assert_eq!(vec![1, 0], bms.gather(9, 1));
        assert_eq!(vec![0; 0], bms.gather(10, 2));
        assert_eq!(vec![0, 3], bms.gather(2, 0));
    }

    #[test]
    fn case2() {
        let mut bms = BookMyShow::new(2, 5);
        assert_eq!(vec![0, 0], bms.gather(4, 0));
        assert_eq!(vec![0; 0], bms.gather(2, 0));
        assert!(bms.scatter(5, 1));
        assert!(!bms.scatter(5, 1));
    }

    #[test]
    fn case3() {
        let mut bms = BookMyShow::new(19, 40);
        assert!(bms.scatter(34, 14));
        assert!(bms.scatter(5, 5));
        assert_eq!(vec![1, 0], bms.gather(20, 6));
        assert_eq!(vec![1, 20], bms.gather(3, 3));
        assert_eq!(vec![0; 0], bms.gather(50, 7));
        assert_eq!(vec![1, 23], bms.gather(16, 5));
        assert_eq!(vec![0; 0], bms.gather(12, 0));
        assert!(bms.scatter(23, 14));
        assert!(!bms.scatter(36, 0));
        assert!(bms.scatter(25, 12));
    }
}
