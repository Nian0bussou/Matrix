#![allow(dead_code)]
#![allow(unused_variables)]

use crate::matrix::*;
use num::pow;
use rand::Rng;

use rayon::prelude::*;

impl Matrix<Alpha> {
    /// generate the Identity matrix, only works with numbers

    pub fn rand(rows: usize, cols: usize) -> Self {
        let mut rng = rand::thread_rng();

        let mut array = vec![vec![0.; cols]; rows];

        for i in (0..rows).into_iter() {
            for j in (0..cols).into_iter() {
                array[i][j] = rng.gen_range(0..101) as Alpha
            }
        }

        Matrix(array, rows, cols)
    }

    pub fn sum_p(&self, other: Self) -> Self {
        assert!(self.rows() == other.rows(), "self row != other col");
        assert!(self.cols() == other.cols(), "self col != other col");

        // Create the output array, but instead of using a regular iterator, use a parallel iterator.
        let array: Vec<Vec<Alpha>> = (0..self.rows())
            .into_par_iter() // Parallel iterator for rows
            .map(|i| {
                (0..self.cols())
                    .into_par_iter() // Parallel iterator for columns
                    .map(|j| (self.0[i][j] + other.0[i][j]) as Alpha)
                    .collect()
            })
            .collect();

        Matrix(array, self.rows(), self.cols())
    }

    pub fn sum(&self, other: Self) -> Self {
        assert!(self.rows() == other.rows(), "self row != other col");
        assert!(self.cols() == other.cols(), "self col != other col");

        let mut array = vec![vec![0.; self.cols()]; self.rows()];

        for i in (0..self.rows()).into_iter() {
            for j in (0..self.cols()).into_iter() {
                array[i][j] = (self.0[i][j] + other.0[i][j]) as Alpha
            }
        }

        Matrix(array, self.rows(), self.cols())
    }

    pub fn id(n: usize) -> Matrix<Alpha> {
        let mut array = vec![vec![0.; n]; n];

        for i in (0..n).into_iter() {
            array[i][i] = 1.;
        }

        Matrix(array, n, n)
    }

    pub fn mult_k(&self, k: Alpha) -> Self {
        let mut array = vec![vec![0.; self.cols()]; self.rows()];
        for i in (0..self.rows()).into_iter() {
            for j in (0..self.cols()).into_iter() {
                array[i][j] = self.0[i][j] * k
            }
        }
        Matrix(array, self.rows(), self.cols())
    }

    pub fn product(
        // what is this mess of generics jesus christ
        &self,
        other: Matrix<Alpha>,
    ) -> Matrix<Alpha> {
        assert!(self.cols() == other.rows(), "cant be multiplied");

        let mut tmp: Alpha;

        let mut array = vec![vec![0.; other.cols()]; self.rows()];

        for i in (0..self.rows()).into_iter() {
            for j in (0..other.cols()).into_iter() {
                tmp = 0.;
                for k in (0..self.cols()).into_iter() {
                    tmp += self.0[i][k] * other.0[k][j]
                }
                array[i][j] = tmp
            }
        }

        Matrix(array, self.rows(), other.cols())
    }

    pub fn product_p(
        // what is this mess of generics jesus christ
        &self,
        other: Matrix<Alpha>,
    ) -> Matrix<Alpha> {
        assert!(self.cols() == other.rows(), "cant be multiplied");

        let array: Vec<Vec<Alpha>> = (0..self.rows())
            .into_par_iter()
            .map(|i| {
                (0..self.cols())
                    .into_par_iter()
                    .map(|j| {
                        let tmp: Alpha = (0..self.cols())
                            .into_iter()
                            .map(|k| self.0[i][k] * other.0[k][j])
                            .fold(0., |acc, x| acc + x);
                        tmp
                    })
                    .collect()
            })
            .collect();
        //for i in (0..self.rows()).into_iter() {
        //    for j in (0..other.cols()).into_iter() {
        //        tmp = 0.;
        //        for k in (0..self.cols()).into_iter() {
        //            tmp += self.0[i][k] * other.0[k][j]
        //        }
        //        array[i][j] = tmp
        //    }
        //}

        Matrix(array, self.rows(), other.cols())
    }

    pub fn pow(&self, k: Alpha) -> Self {
        let rows = self.rows();
        let cols = self.cols();

        if k == 0. {
            panic!("k should not be 0")
        } else if k == 1. {
            Matrix(self.0.clone(), rows, cols)
        } else {
            self.product(self.pow(k - 1.))
        }
    }

    /// uses recursion, hence 'w' for weak
    pub fn det_w(&self) -> Alpha {
        let n = self.rows();

        if n == 1 {
            self.0[0][0]
        } else if n == 2 {
            let va = self.0[0][0] * self.0[1][1];

            let tmp = self.0[0][1] * self.0[1][0];
            let vb = -tmp;

            va + vb
        } else {
            let mut det: Alpha = 0.;
            for j in (0..n).into_iter() {
                let sign = if j % 2 == 0 { 1.0 } else { 0.0 };
                let v1 = self.0[0][j];
                det += sign * v1 * self.sous_mat(0, j).det_w()
            }
            det
        }
    }

    pub fn sous_mat(&self, rowtoremove: usize, coltoremove: usize) -> Self {
        let r1 = self.rows();
        let c1 = self.cols();

        let r2 = r1 - 1;
        let c2 = c1 - 1;

        let mut arr = vec![vec![0.; c2]; r2];

        let mut im = 0;
        for i in (0..r1).into_iter() {
            let mut jm = 0;
            if i != rowtoremove {
                for j in (0..c1).into_iter() {
                    if j != coltoremove {
                        arr[im][jm] = self.0[i][j];
                        jm += 1
                    }
                }
                im += 1
            }
        }
        Matrix(arr, r2, c2)
    }

    pub fn inv_w(&self) -> Self {
        let r = self.rows();
        let c = self.cols();

        assert!(r == c, "need a square");
        assert!(self.det_w() != 0., "not inversible");

        let mut ma: Matrix<Alpha> = Matrix::null((r, c));

        for i in (0..r).into_iter() {
            for j in (0..c).into_iter() {
                ma.0[i][j] = pow(-1., i + j) * self.sous_mat(i, j).det_w();
            }
        }

        self.transpose().mult_k(1. / self.det_w())
    }
}
