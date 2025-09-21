use std::collections::{BTreeSet, HashMap};

pub struct MovieRentingSystem {
    movies: HashMap<(i32, i32), i32>,
    available: HashMap<i32, BTreeSet<(i32, i32)>>,
    rent: BTreeSet<(i32, i32, i32)>,
}

impl MovieRentingSystem {
    pub fn new(n: i32, entries: Vec<Vec<i32>>) -> Self {
        let mut movies = HashMap::with_capacity(entries.len() * 2);
        let mut available: HashMap<i32, BTreeSet<(i32, i32)>> =
            HashMap::with_capacity(n as usize * 2);
        for e in entries {
            movies.insert((e[0], e[1]), e[2]);
            available.entry(e[1]).or_default().insert((e[2], e[0]));
        }
        Self {
            movies,
            available,
            rent: BTreeSet::new(),
        }
    }

    pub fn search(&self, movie: i32) -> Vec<i32> {
        self.available
            .get(&movie)
            .map(|e| e.iter().take(5).map(|e| e.1).collect())
            .unwrap_or_default()
    }

    pub fn rent(&mut self, shop: i32, movie: i32) {
        let price = *self.movies.get(&(shop, movie)).unwrap();
        self.available
            .get_mut(&movie)
            .unwrap()
            .remove(&(price, shop));
        self.rent.insert((price, shop, movie));
    }

    pub fn drop(&mut self, shop: i32, movie: i32) {
        let price = *self.movies.get(&(shop, movie)).unwrap();
        self.available
            .get_mut(&movie)
            .unwrap()
            .insert((price, shop));
        self.rent.remove(&(price, shop, movie));
    }

    pub fn report(&self) -> Vec<Vec<i32>> {
        self.rent.iter().take(5).map(|e| vec![e.1, e.2]).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let entries = [
            [0, 1, 5],
            [0, 2, 6],
            [0, 3, 7],
            [1, 1, 4],
            [1, 2, 7],
            [2, 1, 5],
        ]
        .iter()
        .map(|e| e.to_vec())
        .collect();
        let mut s = MovieRentingSystem::new(3, entries);
        assert_eq!(vec![1, 0, 2], s.search(1));
        s.rent(0, 1);
        s.rent(1, 2);
        assert_eq!(vec![vec![0, 1], vec![1, 2]], s.report());
        s.drop(1, 2);
        assert_eq!(vec![0, 1], s.search(2));
    }
}
