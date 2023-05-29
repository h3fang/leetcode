pub struct Solution;

pub struct ParkingSystem {
    slots: [i32; 3],
}

impl ParkingSystem {
    pub fn new(big: i32, medium: i32, small: i32) -> Self {
        Self {
            slots: [big, medium, small],
        }
    }

    pub fn add_car(&mut self, car_type: i32) -> bool {
        if self.slots[(car_type - 1) as usize] > 0 {
            self.slots[(car_type - 1) as usize] -= 1;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut ps = ParkingSystem::new(1, 1, 0);
        assert!(ps.add_car(1));
        assert!(ps.add_car(2));
        assert!(!ps.add_car(3));
        assert!(!ps.add_car(1));
    }
}
