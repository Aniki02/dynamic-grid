#[macro_use] extern crate assert_matches;

use std::fmt;
use std::fmt::Formatter;
use std::string::ToString;
use std::slice::{Iter, IterMut};

#[derive(Default, Debug, Clone)]
/// Dynamic Grid
pub struct DynamicGrid <T>{
    data: Vec<T>,
    elements_per_row: Vec<usize>
}

impl <T> DynamicGrid<T> where T: Clone{

    /// Constructor, Returns a dynamic grid
    ///
    /// # Arguments
    /// # Examples
    ///
    /// ```rust,editable
    ///     let grid = DynamicGrid::new();
    /// ```

    pub fn new () -> Self{
        DynamicGrid{ data: vec![], elements_per_row: vec![] }
    }

    /// Init a grid of size rows x columns with the given data element
    ///
    /// # Arguments
    /// * `row` - number of rows
    /// * `col` - number columns
    /// * `value` - default value
    pub fn init (row: usize, col: usize, value: T) -> Self{
        DynamicGrid{
            data: vec![value; row * col],
            elements_per_row: vec![col; row]
        }
    }

    /// Returns number of rows of the grid
    pub fn rows(&self) -> usize {
        self.elements_per_row.len()
    }

    /// Returns the size of the row indicate by the index
    /// # Arguments
    /// * `index` - rows index
    pub fn row_size(&self, index: usize) -> Option<&usize> {
        self.elements_per_row.get(index)
    }

    /// push value in the last position of last row
    /// * `value` - value to push
    pub fn push(&mut self, value: T){
        self.data.push(value);
        let len = self.elements_per_row.len();

        if len > 0 {
            self.elements_per_row[len - 1] += 1;
        } else {
            self.elements_per_row.push(1);
        }
    }

    /// push a new empty row
    pub fn push_row(&mut self){
        self.elements_per_row.push(0)
    }

    /// remove the last value of the last row
    pub fn remove(&mut self){
        if self.data.len() > 0 {
            *self.elements_per_row.last_mut().unwrap() -= 1;
            self.data.remove(self.data.len() -1 );
        }
    }

    /// remove the last row
    pub fn remove_row(&mut self) {
        if self.data.len() > 0 {
            let len_data = self.data.len();
            let nb_value_of_last_row = self.elements_per_row.remove(self.elements_per_row.len() - 1 );
            for i in  0..nb_value_of_last_row{
                self.data.remove(len_data - (i + 1));
            }
        }
    }

    /// Returns a reference to an element, without doing bound checking.
    /// # Arguments
    /// `index_row` - index of row
    /// `index_col` - index of column
    /// # Example
    pub unsafe fn get_unchecked(&self, index_row: usize, index_col: usize) -> &T{
        self.data.get_unchecked(index_row * (self.elements_per_row[index_row] - 1) + index_col)
    }

    /// Return a mutable reference to an element, without doing bound checking.
    /// # Arguments
    /// `index_row` - index of row
    /// `index_col` - index of column
    /// # Example
    pub unsafe fn get_unchecked_mut(&mut self, index_row: usize, index_col: usize) -> &mut T{
        self.data.get_unchecked_mut(index_row * (self.elements_per_row[index_row] - 1) + index_col)
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
            if index_col < self.elements_per_row[index_row] {
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
            if index_col < self.elements_per_row[index_row] {
                unsafe{ Some(self.get_unchecked_mut(index_row, index_col))}
            } else {
                None
            }
        }else {
            None
        }
    }

    /// Returns the dimension of the grid as vec which index represent index of row and value
    /// represent the number of elements in it
    /// # Example
    /// let g = DynamicGrid::init(3, 3, 0);
    /// let size = g.size();
    /// assert_matches!(vec![3, 3], size);
    ///
    pub fn size(&self) -> &Vec<usize>{
        &self.elements_per_row
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
            let cols = self.elements_per_row[index_row];
            let start = index_row * cols;
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
            let cols = self.elements_per_row[index_row];
            let start = index_row * cols;
            return self.data[start..(start + cols)].iter_mut()
        } else {
            panic!("Out of bounds. Row index must be less than {:?}, your index is {:?}", self.rows() - 1, index_row)
        }
    }


}

impl <T> fmt::Display for DynamicGrid<T> where T: Clone + ToString{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        if !self.elements_per_row.is_empty(){
            let mut current_row: usize = 0;
            let mut out_of_row: usize = self.elements_per_row[current_row];

            for (i, data) in self.iter().enumerate() {
                if i == out_of_row {
                    s.push_str("\n");
                    current_row += 1;
                    if current_row < self.rows(){
                        out_of_row += self.elements_per_row[current_row]
                    }
                }
                s.push_str(data.to_string().as_str())
            }
        }
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {

    use crate::DynamicGrid;

    #[test]
    fn test_new() {
        let g: DynamicGrid<i32> = DynamicGrid::new();

        assert_matches!(g.rows(), 0);
        assert_matches!(g.row_size(0), None);
        assert_matches!(g.row_size(10), None);
    }

    #[test]
    fn test_init() {
        let g = DynamicGrid::init(3, 2, 0);
        assert_matches!(g.rows(), 3);
        assert_matches!(g.row_size(0), Some(2));
        assert_matches!(g.row_size(1), Some(2));
        assert_matches!(g.row_size(10), None);
    }

    #[test]
    fn test_push() {
        let mut g = DynamicGrid::init(3, 2, 0);

        g.push(4);
        assert_matches!(g.row_size(0), Some(2));
        assert_matches!(g.row_size(2), Some(3));
    }

    #[test]
    fn test_push_row() {
        let mut g: DynamicGrid<i32> = DynamicGrid::new();
        g.push_row();
        assert_matches!(g.rows(), 1);
    }

    #[test]
    fn test_remove() {
        let mut g = DynamicGrid::init(3, 4, 0);
        g.remove();
        assert_matches!(g.row_size(2), Some(3))
    }

    #[test]
    fn test_remove_row() {
        let mut g = DynamicGrid::init(3, 4, 0);
        g.remove_row();
        assert_matches!(g.rows(), 2);
    }

    #[test]
    fn test_get() {
        let mut g = DynamicGrid::init(3, 3, 1);
        assert_matches!(g.get(0,0), Some(1));
        assert_matches!(g.get(10,0), None);
        assert_matches!(g.get(1, 1), Some(1));

        g.push(3);
        assert_matches!(g.get(2, 3), Some(3));
    }

    #[test]
    fn test_get_mut() {
        let mut g = DynamicGrid::init(3, 3, 1);
        assert_matches!(g.get_mut(0,0), Some(1));
        assert_matches!(g.get_mut(10,0), None);
        assert_matches!(g.get_mut(1, 1), Some(1));

        g.push(3);
        assert_matches!(g.get_mut(2, 3), Some(3));

        *g.get_mut(2, 3).unwrap() = 5;
        assert_matches!(g.get(2, 3), Some(5));
    }

    #[test]
    fn test_size() {
        let g = DynamicGrid::init(3, 3, 0);
        let _size = g.size();
        assert_matches!(vec![3, 3], _size);
    }

    #[test]
    fn test_iterator() {
        let g = DynamicGrid::init(2, 2, 0);
        let mut iter = g.iter();
        assert_matches!(iter.next(), Some(0));
        assert_matches!(iter.next(), Some(0));
        assert_matches!(iter.next(), Some(0));
        assert_matches!(iter.next(), Some(0));
        assert_matches!(iter.next(), None);
    }

    #[test]
    fn test_row_iterator() {
        let g = DynamicGrid::init(2, 2, 0);
        let mut iter = g.iter_row(1);
        assert_matches!(iter.next(), Some(0));
        assert_matches!(iter.next(), Some(0));
        assert_matches!(iter.next(), None);
    }

    #[test]
    #[should_panic]
    fn test_row_iterator_should_panic() {
        let g = DynamicGrid::init(2, 2, 0);
        let mut _iter = g.iter_row(10);
    }
}
