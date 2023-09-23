pub struct LockingTree {
    parent: Vec<i32>,
    g: Vec<Vec<i32>>,
    locked: Vec<i32>,
}

impl LockingTree {
    pub fn new(parent: Vec<i32>) -> Self {
        let n = parent.len();
        let mut g = vec![vec![]; n];
        for (i, &p) in parent.iter().enumerate() {
            if p != -1 {
                g[p as usize].push(i as i32);
            }
        }
        Self {
            parent,
            g,
            locked: vec![-1; n],
        }
    }

    pub fn lock(&mut self, num: i32, user: i32) -> bool {
        if self.locked[num as usize] != -1 {
            false
        } else {
            self.locked[num as usize] = user;
            true
        }
    }

    pub fn unlock(&mut self, num: i32, user: i32) -> bool {
        if self.locked[num as usize] == user {
            self.locked[num as usize] = -1;
            true
        } else {
            false
        }
    }

    pub fn upgrade(&mut self, num: i32, user: i32) -> bool {
        if self.locked[num as usize] != -1 {
            return false;
        }
        let mut p = self.parent[num as usize];
        while p != -1 {
            if self.locked[p as usize] != -1 {
                return false;
            }
            p = self.parent[p as usize];
        }
        let mut r = false;
        let mut q = self.g[num as usize].clone();
        while let Some(x) = q.pop() {
            if self.locked[x as usize] != -1 {
                r = true;
                self.locked[x as usize] = -1;
            }
            q.extend(&self.g[x as usize]);
        }
        if r {
            self.locked[num as usize] = user;
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut lt = LockingTree::new(vec![-1, 0, 0, 1, 1, 2, 2]);
        assert!(lt.lock(2, 2));
        assert!(!lt.unlock(2, 3));
        assert!(lt.unlock(2, 2));
        assert!(lt.lock(4, 5));
        assert!(lt.upgrade(0, 1));
        assert!(!lt.lock(0, 1));
    }
}
