mod matrix;

use matrix::*;

mod matrixany;

mod matrixnum;

fn main() {
    let a = Matrix::rand(3, 3);
    let b = Matrix::rand(3, 3);

    println!("おはようございます、みんなさん");

    a.sum_p(b.clone()).print();

    // am i really faster on workman or not ???
    a.product_p(b).print();
}
