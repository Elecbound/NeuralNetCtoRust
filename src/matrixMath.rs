use crate::matrixNotMath::{Matrix, matrix_create};

fn check_dimension(matrix1: &Matrix, matrix2: &Matrix) -> bool {
    if (matrix1.rows == matrix2.rows && matrix1.columns == matrix2.columns){
        return true;
    }
    return false;
}

fn multiply(mut matrix1: Matrix, mut matrix2:Matrix) -> Matrix {
    let dm = check_dimension(&matrix1, &matrix2);
    let mut new_matrix = match dm {
        true =>
            matrix_create(matrix1.rows, matrix1.columns),

        false => panic!("Matrix dimensions are different fix it. 
            Matrix 1 is {},{} Matrix 2 is{}, {}", 
            matrix1.rows, matrix1.columns, matrix2.columns, matrix2.columns),
    };
        
    for i in 0..matrix1.rows {
        for j in 0..matrix2.columns {
            new_matrix.entries[i][j] = matrix1.entries[i][j] * matrix2.entries[i][j];
        }
    }

    new_matrix
}
fn add(mut matrix1: Matrix, mut matrix2:Matrix) -> Matrix {
    let dm = check_dimension(&matrix1, &matrix2);
    let mut new_matrix = match dm {
        true =>
            matrix_create(matrix1.rows, matrix1.columns),

        false => panic!("Matrix dimensions are different fix it. 
            Matrix 1 is {},{} Matrix 2 is{}, {}", 
            matrix1.rows, matrix1.columns, matrix2.columns, matrix2.columns),
    };
        
    for i in 0..matrix1.rows {
        for j in 0..matrix2.columns {
            new_matrix.entries[i][j] = matrix1.entries[i][j] + matrix2.entries[i][j];
        }
    }

    new_matrix
}

fn subtract(mut matrix1: Matrix, mut matrix2:Matrix) -> Matrix {
    let dm = check_dimension(&matrix1, &matrix2);
    let mut new_matrix = match dm {
        true =>
            matrix_create(matrix1.rows, matrix1.columns),

        false => panic!("Matrix dimensions are different fix it. 
            Matrix 1 is {},{} Matrix 2 is{}, {}", 
            matrix1.rows, matrix1.columns, matrix2.columns, matrix2.columns),
    };
        
    for i in 0..matrix1.rows {
        for j in 0..matrix2.columns {
            new_matrix.entries[i][j] = matrix1.entries[i][j] - matrix2.entries[i][j];
        }
    }

    new_matrix
}
fn apply(mut matrix1: Matrix, function: f64) -> Matrix {
    
    let mut new_matrix =matrix1.clone();
    for i in 0..matrix1.rows {
        for j in 0..matrix1.columns {
            new_matrix.entries[i][j] = function * matrix1.entries[i][j];
        }
    }
    new_matrix
}
fn dot(mut matrix1: Matrix, mut matrix2:Matrix) -> Matrix {
    let dm = check_dimension(&matrix1, &matrix2);
    let mut new_matrix = match dm {
        true =>
            matrix_create(matrix1.rows, matrix1.columns),

        false => panic!("Matrix dimensions are different fix it. 
            Matrix 1 is {},{} Matrix 2 is{}, {}", 
            matrix1.rows, matrix1.columns, matrix2.columns, matrix2.columns),
    };
        
    for i in 0..matrix1.rows {
        for j in 0..matrix2.columns {
            let mut dot = 0.0;
            for k in 0..matrix2.rows {
                dot += (matrix1.entries[i][k] * matrix2.entries[k][j]) as f64;
            }
            new_matrix.entries[i][j] = dot;
        }
    }

    new_matrix
}
fn scale(mut matrix1: Matrix, mut scale: f64) -> Matrix {
    
    let mut new_matrix =matrix1.clone();
    for i in 0..matrix1.rows {
        for j in 0..matrix1.columns {
            new_matrix.entries[i][j] *= scale;
        }
    }
    new_matrix
}
fn addScalar(mut matrix1: Matrix, mut scale: f64) -> Matrix {
    
    let mut new_matrix =matrix1.clone();
    for i in 0..matrix1.rows {
        for j in 0..matrix1.columns {
            new_matrix.entries[i][j] += scale;
        }
    }
    new_matrix
}
fn transpose(mut matrix1: Matrix){
let mut new_matrix = matrix_create(matrix1.rows, matrix1.columns);
    for i in 0..matrix1.rows {
        for j in 0..matrix1.columns {
            new_matrix.entries[j][i] = matrix1.entries[i][j];
        }
    }
}