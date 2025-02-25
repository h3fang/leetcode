pub struct Allocator {
    mem: Vec<i32>,
}

impl Allocator {
    pub fn new(n: i32) -> Self {
        Self {
            mem: vec![0; n as usize],
        }
    }

    pub fn allocate(&mut self, size: i32, m_id: i32) -> i32 {
        let mut c = 0;
        for (i, &x) in self.mem.iter().enumerate() {
            if x == 0 {
                c += 1;
            } else {
                c = 0;
            }
            if c == size {
                let r = i + 1 - size as usize;
                self.mem[r..=i].iter_mut().for_each(|x| *x = m_id);
                return r as i32;
            }
        }
        -1
    }

    pub fn free_memory(&mut self, m_id: i32) -> i32 {
        let mut result = 0;
        for x in self.mem.iter_mut() {
            if *x == m_id {
                *x = 0;
                result += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut a = Allocator::new(10);
        assert_eq!(0, a.allocate(1, 1));
        assert_eq!(1, a.allocate(1, 2));
        assert_eq!(2, a.allocate(1, 3));
        assert_eq!(1, a.free_memory(2));
        assert_eq!(3, a.allocate(3, 4));
        assert_eq!(1, a.allocate(1, 1));
        assert_eq!(6, a.allocate(1, 1));
        assert_eq!(3, a.free_memory(1));
        assert_eq!(-1, a.allocate(10, 2));
        assert_eq!(0, a.free_memory(7));
    }
}
