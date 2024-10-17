mod matrix;

use matrix::*;

mod matrixany;

mod matrixnum;

fn main() {
    let a = Matrix::rand(3, 3);
    let b = Matrix::rand(3, 3);

    let c = Matrix::from(vec![
        vec!["a", "b", "c"],
        vec!["d", "e", "f"],
        vec!["g", "h", "i"],
    ]);

    c.print();
    c.transpose().print();

    a.product(b).print();

    a.inv_w().print();
}
