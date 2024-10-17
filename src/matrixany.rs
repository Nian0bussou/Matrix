#![allow(dead_code)]
#![allow(unused_variables)]
use crate::Matrix;

impl<T> Matrix<T>
where
    T: Default + Copy,
{
    /// Make a Matrix from the provided vec<vec<T>>
    /// # Errors
    /// This function will return an error if:
    ///     data lengths are not the same as R(ows), C(ols)
    pub fn from(data: Vec<Vec<T>>) -> Self {
        let row = data.len();
        let col = data[0].len();
        Matrix(data, row, col)
    }

    /// Return a null matrix (fills the 2d array with T::default() )
    pub fn null((row, col): (usize, usize)) -> Self {
        let data = vec![vec![T::default(); col]; row];
        Matrix(data, row, col)
    }

    /// return transpose from a matrix (&self)
    pub fn transpose(&self) -> Self {
        let mut array = vec![vec![T::default(); self.rows()]; self.cols()];

        for i in (0..self.rows()).into_iter() {
            for j in (0..self.cols()).into_iter() {
                array[j][i] = self.0[i][j]
            }
        }

        Matrix(array, self.cols(), self.rows())
    }
}
