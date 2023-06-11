pub struct Solution;

pub struct SnapshotArray {
    snap_id: i32,
    records: Vec<Vec<(i32, i32)>>,
}

impl SnapshotArray {
    pub fn new(length: i32) -> Self {
        Self {
            snap_id: 0,
            records: vec![vec![(0, 0)]; length as usize],
        }
    }

    pub fn set(&mut self, index: i32, val: i32) {
        self.records[index as usize].push((self.snap_id, val));
    }

    pub fn snap(&mut self) -> i32 {
        self.snap_id += 1;
        self.snap_id - 1
    }

    pub fn get(&self, index: i32, snap_id: i32) -> i32 {
        let rs = &self.records[index as usize];
        let i = rs.partition_point(|&(id, _)| id <= snap_id);
        rs[i - 1].1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut sa = SnapshotArray::new(3);
        sa.set(0, 5);
        assert_eq!(0, sa.snap());
        sa.set(0, 6);
        assert_eq!(5, sa.get(0, 0));
    }
}
