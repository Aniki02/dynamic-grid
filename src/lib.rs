#[macro_use] extern crate assert_matches;

use std::fmt;
use std::fmt::Formatter;
use std::string::ToString;

#[derive(Default, Debug, Clone)]
/// Dynamic Grid
pub struct DynamicGrid <T>{
    data: Vec<T>,
    nb_value_per_row: Vec<usize>
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
        DynamicGrid{ data: vec![], nb_value_per_row: vec![] }
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
            nb_value_per_row: vec![col; row]
        }
    }

    /// Returns number of rows of the grid
    pub fn rows(&self) -> usize {
        self.nb_value_per_row.len()
    }

    /// Returns the size of the row indicate by the index
    /// # Arguments
    /// * `index` - rows index
    pub fn row_size(&self, index: usize) -> Option<&usize> {
        self.nb_value_per_row.get(index)
    }

    /// push value in the last position of last row
    /// * `value` - value to push
    pub fn push(&mut self, value: T){
        self.data.push(value);
        let len = self.nb_value_per_row.len();

        if len > 0 {
            self.nb_value_per_row[len - 1] += 1;
        } else {
            self.nb_value_per_row.push(1);
        }
    }

    pub fn push_row(&mut self){
        self.nb_value_per_row.push(0)
    }
}

impl <T> fmt::Display for DynamicGrid<T> where T: Clone + ToString{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        if !self.nb_value_per_row.is_empty(){
            let mut current_row: usize = 0;
            let mut out_of_row: usize = self.nb_value_per_row[current_row];

            for (i, data) in self.data.iter().enumerate() {
                if i == out_of_row {
                    s.push_str("\n");
                    current_row += 1;
                    if current_row < self.rows(){
                        out_of_row += self.nb_value_per_row[current_row]
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
}
