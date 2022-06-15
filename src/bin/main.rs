use NeuralNetTest::matrixNotMath::*;
use NeuralNetTest::imageProcess::*;
use std::ffi::OsString;
use std::path::Path;
use std::ffi::OsStr;
use std::path;

fn main() {
    let mut v = matrix_create(5,5);
    dbg!(&v);
    matrix_fill(&mut v, 6.0);
    dbg!(&v);
    matrix_save(&mut v,"test.txt");
    dbg!(&v.entries[0][2]);
    let b = matrix_load("test.txt");
    dbg!(b);
    let csv_file_1 = csv_to_imgs("mnist_train.csv");
    dbg!("csv" ,csv_file_1);
}
