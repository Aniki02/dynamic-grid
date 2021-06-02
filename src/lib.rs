#[cfg(test)]
#[macro_use] extern crate assert_matches;

use std::fmt;
use std::fmt::Formatter;
use std::string::ToString;
use std::slice::{Iter, IterMut};

#[derive(Default, Debug, Clone)]
/// Dynamic Grid
pub struct DynamicGrid <T>{
    data: Vec<T>,
    line_start_index: Vec<usize>
}

impl <T> DynamicGrid<T> where T: Clone{

    /// Constructor, Returns a dynamic grid
    pub fn new () -> Self{
        DynamicGrid{ data: vec![], line_start_index: vec![] }
    }

    /// Init a grid of size rows x columns with the given data element
    ///
    /// # Arguments
    /// * `row` - number of rows
    /// * `col` - number columns
    /// * `value` - default value
    pub fn init (row: usize, col: usize, value: T) -> Self{
        let mut v = vec![0, row];
        let mut index_row = 0;
        v.iter_mut().for_each(| value| {
            *value = index_row;
            index_row += col;
        });

        DynamicGrid{
            data: vec![value; row * col],
            line_start_index: v
        }
    }

    ///Returns a grid from a vector of vector
    /// # Arguments
    /// * vec - Vector which represent a grid
    pub fn from_vec(vec: Vec<Vec<T>>) -> Self{
        let mut g = DynamicGrid::new();
        let mut start_index = 0;
        for row  in vec.iter() {
            g.line_start_index.push(start_index);
            for item in row.iter(){
                g.data.push(item.clone());
                start_index+=1;
            }
        }
        g
    }

    /// Returns number of rows of the grid
    pub fn rows(&self) -> usize {
        self.line_start_index.len()
    }

    /// Returns the size of the row indicate by the index
    /// # Arguments
    /// * `index` - rows index
    pub fn row_size(&self, index_row: usize) -> Option<usize> {
        if index_row < self.rows() {
            Some(self.row_size_unchecked(index_row))
        } else {
            None
        }
    }

    /// Returns the size of the row indicate by the index, without bound checking
    /// # Arguments
    /// * `index` - rows index
    pub fn row_size_unchecked(&self, index_row: usize) -> usize{
        let end = if index_row < self.rows() - 1 {self.line_start_index[index_row + 1]}
        else {self.data.len()};
        end - self.line_start_index[index_row]
    }

    /// push value in the last position of last row
    /// * `value` - value to push
    pub fn push(&mut self, value: T) -> (usize, usize){
        self.data.push(value);
        (self.rows() - 1, self.row_size_unchecked(self.rows() - 1) - 1 )

    }

    /// push value in the last position at row mentioned
    /// # Argument
    /// * index_row - index of row
    /// * value - value to push
    pub fn push_at_row(&mut self, index_row: usize, value: T) -> Option<(usize, usize)> {
        if index_row < self.rows() {
            let position = (index_row, self.row_size_unchecked(index_row));
            self.insert(position.0, position.1, value);
            return Some(position)
        }
        return None
    }

    /// insert value at position
    /// # Argument
    /// * index_row - index of row
    /// * index_col - index of col
    /// * value - value to insert
    ///
    /// # Panics
    /// Panics if the row and the col index are out of bounds.
    pub fn insert(&mut self, index_row: usize, index_col:usize, value: T){
        if index_row < self.rows(){
            if index_col <= self.row_size_unchecked(index_row){
                self.data.insert(self.line_start_index[index_row] + index_col, value);
                if index_row < self.rows() - 1 {self.line_start_index[index_row + 1] += 1}
            }else {
                panic!("Out of bounds. Col index must be less than {:?}, your index is {:?}", self.row_size_unchecked(index_row) - 1, index_col)

            }
        } else {
            panic!("Out of bounds. Row index must be less than {:?}, your index is {:?}", self.rows() - 1, index_row)
        }
    }

    /// swap two element in the grid
    /// # Argument
    /// * first_position - position of the first element
    /// * second_position - position of the second element
    /// # Panics
    /// Panics if the row and the col index are out of bounds.
    pub fn swap(&mut self, first_position: (usize, usize), second_position: (usize, usize)) {
        if first_position.0 < self.rows() && second_position.0 < self.rows() {
            if first_position.1 < self.row_size_unchecked(first_position.0)
                && second_position.1 < self.row_size_unchecked(second_position.0){
                let first_index = self.line_start_index[first_position.0] + first_position.1;
                let second_index = self.line_start_index[second_position.0] + second_position.1;

                self.data.swap(first_index, second_index);
            } else {
                panic!("Out of bounds");
            }
        } else {
            panic!("Out of bounds");
        }
    }


    /// push a new empty row
    pub fn push_new_row(&mut self, value: T) -> (usize, usize){
        self.line_start_index.push(self.data.len());
        self.push(value);
        (self.rows() - 1, self.row_size_unchecked(self.rows() - 1) - 1 )
    }

    /// remove the last value of the last row
    pub fn remove(&mut self){
        if self.data.len() > 0 {
            self.data.remove(self.data.len() -1 );
            if *self.line_start_index.last().unwrap() >= self.data.len(){
                self.remove_row(self.rows() - 1 )
            }
        }
    }

    /// remove the last row
    pub fn remove_row(&mut self, index_row: usize) {
        if !self.data.is_empty() && index_row < self.rows(){
            let start = self.line_start_index[index_row];
            let end = start + self.row_size_unchecked(index_row);

            self.data = self.data.iter()
                .enumerate()
                .filter(|(i, _)| !(start..end).contains(i))
                .map(|(_, v)| v.clone())
                .collect();

            self.line_start_index.remove(index_row);
        }
    }

    /// Returns a reference to an element, without doing bound checking.
    /// # Arguments
    /// `index_row` - index of row
    /// `index_col` - index of column
    /// # Example
    pub unsafe fn get_unchecked(&self, index_row: usize, index_col: usize) -> &T{
        self.data.get_unchecked(self.line_start_index[index_row] + index_col)
    }

    /// Return a mutable reference to an element, without doing bound checking.
    /// # Arguments
    /// `index_row` - index of row
    /// `index_col` - index of column
    /// # Example
    pub unsafe fn get_unchecked_mut(&mut self, index_row: usize, index_col: usize) -> &mut T{
        self.data.get_unchecked_mut(self.line_start_index[index_row] + index_col)
    }

    ///Returns a reference to an element.
    ///
    /// # Arguments
    /// `index_row` - index of row
    /// `index_col` - index of column
    /// # Example
    ///
    pub fn get (&self, index_row: usize, index_col: usize) -> Option<&T>{
        if index_row < self.rows() {
            if index_col < self.row_size_unchecked(index_row) {
                unsafe{ Some(self.get_unchecked(index_row, index_col))}
            } else {
                None
            }
        }else {
            None
        }
    }

    ///Returns a reference to an element.
    ///
    /// # Arguments
    /// `index_row` - index of row
    /// `index_col` - index of column
    /// # Example
    ///
    pub fn get_mut (&mut self, index_row: usize, index_col: usize) -> Option<&mut T>{
        if index_row < self.rows() {
            if index_col < self.row_size_unchecked(index_row) {
                unsafe{ Some(self.get_unchecked_mut(index_row, index_col))}
            } else {
                None
            }
        }else {
            None
        }
    }

    /// Returns an iterator over the whole grid, starting from the first row and column.
    pub fn iter(&self) -> Iter<T> {
        self.data.iter()
    }

    /// Returns an mutable iterator over the whole grid that allows modifying each value.
    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.data.iter_mut()
    }

    /// Returns a row Iterator
    ///
    /// # Panics
    /// Panics if the row index is out of bounds.
    pub fn iter_row(&self, index_row: usize) -> Iter<T> {
        if index_row < self.rows() {
            let cols = self.row_size_unchecked(index_row);
            let start = self.line_start_index[index_row];
            return self.data[start..(start + cols)].iter()
        } else {
            panic!("Out of bounds. Row index must be less than {:?}, your index is {:?}", self.rows() - 1, index_row)
        }
    }

    /// Returns a mutable row Iterator
    ///
    /// # Panics
    /// Panics if the row index is out of bounds.
    pub fn iter_row_mut(&mut self, index_row: usize) -> IterMut<T> {
        if index_row < self.rows() {
            let cols = self.row_size_unchecked(index_row);
            let start = self.line_start_index[index_row];
            return self.data[start..(start + cols)].iter_mut()
        } else {
            panic!("Out of bounds. Row index must be less than {:?}, your index is {:?}", self.rows() - 1, index_row)
        }
    }


}

impl <T> fmt::Display for DynamicGrid<T> where T: Clone + ToString{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut s = String::new();

        for row in 0..self.rows(){
            for data in self.iter_row(row) {
                s.push_str(data.to_string().as_str());
                s.push_str(",")
            }
            s.push_str("\n");
        }

        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {

    use crate::DynamicGrid;

    // 10, 5, 4
    // 3, 9
    // 1
    // 7, 6, 2, 8
    fn init() -> DynamicGrid<usize>{
        /*let mut g = DynamicGrid::new();
        g.push_new_row(10);
        g.push(5);
        g.push(4);

        g.push_new_row(3);
        g.push(9);

        g.push_new_row(1);

        g.push_new_row(7);
        g.push(6);
        g.push(2);
        g.push(8);*/

        let g =
            DynamicGrid::from_vec(
                vec![
                        vec![10, 5, 4],
                        vec![3, 9],
                        vec![1],
                        vec![7, 6, 2, 8]
    ]);

        g
    }

    #[test]
    fn test_new() {
        let g: DynamicGrid<i32> = DynamicGrid::new();

        assert_matches!(g.rows(), 0);
        assert_matches!(g.row_size(0), None);
        assert_matches!(g.row_size(10), None);
    }

    #[test]
    fn test_init() {
        let g = init();
        assert_matches!(g.rows(), 4);
        assert_matches!(g.row_size(0), Some(3));
        assert_matches!(g.row_size(1), Some(2));
        assert_matches!(g.row_size(2), Some(1));
        assert_matches!(g.row_size(3), Some(4));
        assert_matches!(g.row_size(10), None);
    }

    #[test]
    fn test_push() {
        let mut g = init();

        let position = g.push(4);
        assert_matches!(position, (3, 4));
        assert_matches!(g.row_size(0), Some(3));
        assert_matches!(g.row_size(3), Some(5));
    }

    #[test]
    fn test_push_new_row(){
        let mut g = init();
        let position = g.push_new_row(4);
        assert_matches!(position, (4, 0));
        assert_matches!(g.row_size(0), Some(3));
        assert_matches!(g.row_size(4), Some(1));
    }

    #[test]
    fn test_push_at_row() {
        let mut g = init();

        let position = g.push_at_row(2, 4);
        println!("{}", g);
        assert_matches!(position, Some((2, 1)));
        assert_matches!(g.get(2, 1), Some(4));
        assert_matches!(g.row_size(0), Some(3));
        assert_matches!(g.row_size(2), Some(2));

    }

    #[test]
    fn test_swap() {
        let mut g = init();

        g.swap((0, 1), (3, 2));

        assert_matches!(g.get(0, 1), Some(2));
        assert_matches!(g.get(3, 2), Some(5));

    }

    #[test]
    fn test_remove() {
        let mut g = init();
        g.remove();
        assert_matches!(g.row_size(3), Some(3))
    }

    #[test]
    fn test_remove_row() {
        let mut g = init();
        g.remove_row(0);
        assert_matches!(g.rows(), 3);
    }

    #[test]
    fn test_get() {
        let mut g = init();
        assert_matches!(g.get(0,0), Some(10));
        assert_matches!(g.get(10,0), None);
        assert_matches!(g.get(1, 1), Some(9));
        assert_matches!(g.get(3, 3), Some(8));
        assert_matches!(g.get(3, 4), None);


        g.push(11);
        assert_matches!(g.get(3, 4), Some(11));
    }

    #[test]
    fn test_get_mut() {
        let mut g = init();
        assert_matches!(g.get_mut(0,0), Some(10));
        assert_matches!(g.get_mut(10,0), None);
        assert_matches!(g.get_mut(1, 1), Some(9));

        g.push(3);
        assert_matches!(g.get_mut(3, 4), Some(3));

        *g.get_mut(3, 4).unwrap() = 5;
        assert_matches!(g.get(3, 4), Some(5));
    }

    #[test]
    fn test_iterator() {
        let g = init();
        let mut iter = g.iter();
        assert_matches!(iter.next(), Some(10));
        assert_matches!(iter.next(), Some(5));
        assert_matches!(iter.next(), Some(4));
        assert_matches!(iter.next(), Some(3));
        assert_matches!(iter.next(), Some(9));
        assert_matches!(iter.next(), Some(1));
        assert_matches!(iter.next(), Some(7));
        assert_matches!(iter.next(), Some(6));
        assert_matches!(iter.next(), Some(2));
        assert_matches!(iter.next(), Some(8));
        assert_matches!(iter.next(), None);
    }

    #[test]
    fn test_row_iterator() {
        let g = init();
        let mut iter = g.iter_row(1);
        assert_matches!(iter.next(), Some(3));
        assert_matches!(iter.next(), Some(9));
        assert_matches!(iter.next(), None);

        let mut iter = g.iter_row(2);
        assert_matches!(iter.next(), Some(1));
        assert_matches!(iter.next(), None);

        let mut iter = g.iter_row(3);
        assert_matches!(iter.next(), Some(7));
        assert_matches!(iter.next(), Some(6));
        assert_matches!(iter.next(), Some(2));
        assert_matches!(iter.next(), Some(8));
        assert_matches!(iter.next(), None);

    }

    #[test]
    #[should_panic]
    fn test_row_iterator_should_panic() {
        let g = init();
        let mut _iter = g.iter_row(10);

        // Seconde way to assert panic
        // let should_panic = std::panic::catch_unwind(||{let mut _iter = g.iter_row(10);});
        // assert!(should_panic.is_err());

    }
}
