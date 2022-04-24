use std::collections::HashMap;

#[derive(Default)]
pub struct UndergroundSystem {
    time: HashMap<(String, String), (i32, i32)>,
    checkins: HashMap<i32, (String, i32)>,
}

impl UndergroundSystem {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.checkins.insert(id, (station_name, t));
    }

    pub fn check_out(&mut self, id: i32, exit: String, t: i32) {
        if let Some((entry, t0)) = self.checkins.get(&id) {
            let e = self.time.entry((entry.clone(), exit)).or_default();
            e.0 += 1;
            e.1 += t - t0;
            self.checkins.remove(&id);
        }
    }

    pub fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        if let Some(&(n, total)) = self.time.get(&(start_station, end_station)) {
            total as f64 / n as f64
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        const EPSILON: f64 = 1e-5;
        let mut ugs = UndergroundSystem::new();
        ugs.check_in(45, "Leyton".into(), 3);
        ugs.check_in(32, "Paradise".into(), 8);
        ugs.check_in(27, "Leyton".into(), 10);
        ugs.check_out(45, "Waterloo".into(), 15);
        ugs.check_out(27, "Waterloo".into(), 20);
        ugs.check_out(32, "Cambridge".into(), 22);
        assert!(
            (ugs.get_average_time("Paradise".into(), "Cambridge".into()) - 14.0).abs() < EPSILON
        );
        assert!((ugs.get_average_time("Leyton".into(), "Waterloo".into()) - 11.0).abs() < EPSILON);
        ugs.check_in(10, "Leyton".into(), 24);
        assert!((ugs.get_average_time("Leyton".into(), "Waterloo".into()) - 11.0).abs() < EPSILON);
        ugs.check_out(10, "Waterloo".into(), 38);
        assert!((ugs.get_average_time("Leyton".into(), "Waterloo".into()) - 12.0).abs() < EPSILON);
    }
}
