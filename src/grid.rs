use anyhow::{Result, anyhow};
use itertools::Itertools;

pub struct Grid<T>
where
    T: Copy,
{
    grid: Vec<Vec<T>>,
    m: usize,
    n: usize,
}

impl<T> Grid<T>
where
    T: Copy,
{
    const EIGHT_DIRECTIONS: [[isize; 2]; 8] = [
        [-1, -1],
        [-1, 0],
        [-1, 1],
        [0, -1],
        [0, 1],
        [1, -1],
        [1, 0],
        [1, 1],
    ];

    pub fn new(grid: Vec<Vec<T>>) -> Self {
        let m = grid.len();
        let n = grid[0].len();

        Self { grid, m, n }
    }

    pub fn get_pos_val(&self, i: usize, j: usize) -> Option<T> {
        Some(*self.grid.get(i)?.get(j)?)
    }

    pub fn get_rel_pos_val(&self, i: usize, j: usize, di: isize, dj: isize) -> Option<T> {
        let ni = i.checked_add_signed(di)?;
        let nj = j.checked_add_signed(dj)?;

        self.get_pos_val(ni, nj)
    }

    pub fn set_pos_val(&mut self, i: usize, j: usize, val: T) -> Result<()> {
        *self
            .grid
            .get_mut(i)
            .ok_or(anyhow!("Bounds"))?
            .get_mut(j)
            .ok_or(anyhow!("Bounds"))? = val;
        Ok(())
    }

    pub fn iter_all_pos_in_grid(&self) -> impl Iterator<Item = (usize, usize)> {
        (0..self.m).cartesian_product(0..self.n)
    }

    pub fn iter_eight_neighbors_of_pos(&self, i: usize, j: usize) -> impl Iterator<Item = T> + '_ {
        Grid::<T>::EIGHT_DIRECTIONS
            .iter()
            .filter_map(move |&[di, dj]| self.get_rel_pos_val(i, j, di, dj))
    }
}
