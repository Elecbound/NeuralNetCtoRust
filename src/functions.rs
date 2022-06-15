fn sigmindPrime(matrix: Matrix) -> Matrix{
    let ones = matrix_create(matrix.rows, matrix.columns);
    matrix_fill(ones,1);
    let subtracted = subtract(ones,matrix);
    let multiplied = multiply(matrix, subtracted);
    multiplied
}
fn softmax(matrix: Matrix) -> Matrix{
    let total = 0.0;
    for i in 0..matrix.rows {
        for j in 0..matrix.columns {
            total += (matrix.entries[i][j] as f64).exp();
        }
    }
    new_matrix = matrix_create(matrix.rows, matrix.columns);
    for  i in 0..matrix.rows {
        for j in 0..matrix.columns {
            matrix.entries[i][j] = (matrix.entries[i][j] as f64).exp() / total;
        }
    }
    new_matrix
}