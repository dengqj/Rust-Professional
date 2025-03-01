/*
    Rotate Matrix 90 Degrees
    Given a 2D matrix, rotate it 90 degrees in place. 
    You need to perform the rotation without using any additional matrix storage.

    You need to implement the function `rotate_matrix_90_degrees(matrix: &mut Vec<Vec<i32>>)`.
    The function should rotate the input matrix in place.

    Hint: Consider rotating the matrix layer by layer, starting from the outermost layer and working your way inward.
*/

use std::fmt::{self, Display, Formatter};

pub fn rotate_matrix_90_degrees(matrix: &mut Vec<Vec<i32>>) {
    let n = matrix.len();
    if n == 0 { return; }
    let m = matrix[0].len();
    if m == 0 { return; }

    // Step 1: Convert to square matrix by padding with zeros
    let max_dim = n.max(m);
    for row in matrix.iter_mut() {
        row.resize(max_dim, 0);
    }
    while matrix.len() < max_dim {
        matrix.push(vec![0; max_dim]);
    }

    // Step 2: Rotate the square matrix
    for i in 0..max_dim / 2 {
        for j in 0..(max_dim + 1) / 2 {
            let temp = matrix[i][j];
            matrix[i][j] = matrix[max_dim - 1 - j][i];
            matrix[max_dim - 1 - j][i] = matrix[max_dim - 1 - i][max_dim - 1 - j];
            matrix[max_dim - 1 - i][max_dim - 1 - j] = matrix[j][max_dim - 1 - i];
            matrix[j][max_dim - 1 - i] = temp;
        }
    }

    // Step 3: Restore the original shape
    if n < m {
        // Remove extra columns
        for row in matrix.iter_mut() {
            row.truncate(n);
        }
    } else if m < n {
        // Remove extra rows
        matrix.truncate(m);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_matrix_1() {
        let mut matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![
            vec![7, 4, 1],
            vec![8, 5, 2],
            vec![9, 6, 3],
        ]);
    }

    #[test]
    fn test_rotate_matrix_2() {
        let mut matrix = vec![
            vec![1, 2],
            vec![3, 4],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![
            vec![3, 1],
            vec![4, 2],
        ]);
    }

    #[test]
    fn test_rotate_matrix_3() {
        let mut matrix = vec![
            vec![1],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![
            vec![1],
        ]);
    }

    #[test]
    fn test_rotate_matrix_4() {
        let mut matrix = vec![
            vec![1, 2],
            vec![3, 4],
            vec![5, 6],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![
            vec![5, 3, 1],
            vec![6, 4, 2],
        ]);
    }
}
