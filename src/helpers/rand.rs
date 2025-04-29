use std::ops::Index;

use indexmap::IndexSet;
use rand::Rng;

pub trait RemoveRandom: Index<usize> {
    fn choose_remove<R>(&mut self, rng: &mut R) -> Self::Output
    where
        R: Rng + ?Sized;
}

impl<T> RemoveRandom for Vec<T> {
    fn choose_remove<R>(&mut self, rng: &mut R) -> Self::Output
    where
        R: Rng + ?Sized,
    {
        let index = rng.random_range(..self.len());
        self.swap_remove(index)
    }
}

impl<T> RemoveRandom for IndexSet<T> {
    fn choose_remove<R>(&mut self, rng: &mut R) -> Self::Output
    where
        R: Rng + ?Sized,
    {
        let index = rng.random_range(..self.len());
        self.swap_remove_index(index).unwrap()
    }
}
