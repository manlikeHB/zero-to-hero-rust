use num_traits::Zero;
use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};
fn main() {}

#[derive(Debug, PartialEq, Clone)]
struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T> Matrix<T> {
    fn new(rows: usize, cols: usize, data: Vec<T>) -> Self {
        assert!(data.len() == rows * cols, "Data does not match dimensions");

        Matrix { rows, cols, data }
    }

    fn add(&self, other: &Matrix<T>) -> Matrix<T>
    where
        T: Add<Output = T> + Copy,
    {
        // check dimension
        assert!(
            self.rows == other.rows && self.cols == other.cols,
            "Dimensions don't match"
        );

        let res = self
            .data
            .iter()
            .zip(&other.data)
            .map(|(a, b)| *a + *b)
            .collect();

        Matrix::new(self.rows, self.cols, res)
    }

    fn sub(&self, other: &Matrix<T>) -> Matrix<T>
    where
        T: Copy + Sub<Output = T>,
    {
        // check dimension
        assert!(
            self.rows == other.rows && self.cols == other.cols,
            "Dimensions don't match"
        );

        let res = self
            .data
            .iter()
            .zip(&other.data)
            .map(|(a, b)| *a - *b)
            .collect();

        Matrix::new(self.rows, self.cols, res)
    }

    fn transpose(&self) -> Self
    where
        T: Copy,
    {
        let mut res = Vec::new();

        for i in 0..self.cols {
            for j in 0..self.rows {
                res.push(*self.get(j, i));
            }
        }

        Matrix::new(self.cols, self.rows, res)
    }

    fn multiply(&self, other: &Matrix<T>) -> Self
    where
        T: Mul<Output = T> + Add<Output = T> + Copy + Zero,
    {
        assert!(self.cols == other.rows, "Incompatible dimensions");

        let mut res = Vec::new();

        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = T::zero();
                for k in 0..self.cols {
                    sum = sum + *self.get(i, k) * *other.get(k, j);
                }

                res.push(sum);
            }
        }

        Matrix::new(self.rows, other.cols, res)
    }

    fn scalar_mul(&self, rhs: T) -> Matrix<T>
    where
        T: Mul<Output = T> + Copy,
    {
        let mut new_data = Vec::new();

        for i in 0..self.rows {
            for j in 0..self.cols {
                new_data.push(*self.get(i, j) * rhs);
            }
        }

        Matrix::new(self.rows, self.cols, new_data)
    }

    fn get(&self, i: usize, j: usize) -> &T {
        &self.data[i * self.cols + j]
    }

    fn set(&mut self, i: usize, j: usize, val: T) {
        self[(i, j)] = val;
    }
}

impl<T> Add for &Matrix<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Matrix<T>;

    fn add(self, other: &Matrix<T>) -> Matrix<T> {
        self.add(other)
    }
}

impl<T> Sub for &Matrix<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Matrix<T>;

    fn sub(self, other: &Matrix<T>) -> Matrix<T> {
        self.sub(other)
    }
}

impl<T> Mul for &Matrix<T>
where
    T: Mul<Output = T> + Copy + Zero,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: &Matrix<T>) -> Self::Output {
        self.multiply(rhs)
    }
}

impl<T> Mul<T> for &Matrix<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: T) -> Self::Output {
        self.scalar_mul(rhs)
    }
}

impl<T> Neg for Matrix<T>
where
    T: Neg<Output = T>,
{
    type Output = Matrix<T>;

    fn neg(self) -> Self::Output {
        let res = self.data.into_iter().map(|e| -e).collect();

        Matrix::new(self.rows, self.cols, res)
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (i, j) = index;
        self.get(i, j)
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (i, j) = index;
        &mut self.data[i * self.cols + j]
    }
}

impl<T> Div<T> for &Matrix<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Matrix<T>;

    fn div(self, rhs: T) -> Self::Output {
        let data = self.data.iter().map(|x| *x / rhs).collect();
        Matrix::new(self.rows, self.cols, data)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_ok() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let matrix = Matrix::new(2, 3, data.clone());

        assert_eq!(matrix.rows, 2);
        assert_eq!(matrix.cols, 3);
        assert_eq!(data.len(), 6);

        let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let matrix = Matrix::new(2, 4, data.clone());

        assert_eq!(matrix.rows, 2);
        assert_eq!(matrix.cols, 4);
        assert_eq!(matrix.data.len(), data.len());

        let m3 = Matrix::new(2, 2, vec!["a", "b", "c", "d"]);

        assert_eq!(m3.rows, 2);
        assert_eq!(m3.cols, 2);
        assert_eq!(m3.data.len(), 4);
    }

    #[test]
    #[should_panic(expected = "Data does not match dimensions")]
    fn test_new_fail() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        Matrix::new(3, 3, data);
    }

    #[test]
    fn test_add_matrix() {
        let matrix_1 = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let matrix_2 = Matrix::new(2, 2, vec![5.0, 6.0, 7.0, 8.0]);

        let expected_data = vec![6.0, 8.0, 10.0, 12.0];
        let res = matrix_1.add(&matrix_2);

        assert!(res.data == expected_data, "wrong matrix data");
    }

    #[test]
    #[should_panic(expected = "Dimensions don't match")]
    fn test_add_matrix_wrong_dimensions() {
        let matrix_1 = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let matrix_2 = Matrix::new(3, 2, vec![5.0, 6.0, 7.0, 8.0, 9.0, 10.0]);

        matrix_1.add(&matrix_2);
    }

    #[test]
    fn test_transpose() {
        let matrix = Matrix::new(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let expected_matrix = Matrix::new(3, 2, vec![1.0, 4.0, 2.0, 5.0, 3.0, 6.0]);

        assert!(matrix.transpose() == expected_matrix, "Wrongly transposed");
    }

    #[test]
    fn test_multiply() {
        let matrix_1 = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let matrix_2 = Matrix::new(2, 2, vec![5.0, 6.0, 7.0, 8.0]);

        let expected_matrix = Matrix::new(2, 2, vec![19.0, 22.0, 43.0, 50.0]);

        assert_eq!(matrix_1.multiply(&matrix_2), expected_matrix);

        assert_eq!(&matrix_1 * &matrix_2, expected_matrix);
    }

    #[test]
    fn test_add_trait() {
        let matrix_1 = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let matrix_2 = Matrix::new(2, 2, vec![5.0, 6.0, 7.0, 8.0]);

        let expected_data = vec![6.0, 8.0, 10.0, 12.0];

        assert!(
            &matrix_1 + &matrix_2 == Matrix::new(2, 2, expected_data),
            "wrong matrix data"
        );
    }

    #[test]
    fn test_mul_trait() {
        let matrix_1 = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let matrix_2 = Matrix::new(2, 2, vec![5.0, 6.0, 7.0, 8.0]);

        let expected_matrix = Matrix::new(2, 2, vec![19.0, 22.0, 43.0, 50.0]);

        assert_eq!(&matrix_1 * &matrix_2, expected_matrix);
    }

    #[test]
    fn test_scalar_mul() {
        let matrix = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);

        let expected_matrix = Matrix::new(2, 2, vec![2.0, 4.0, 6.0, 8.0]);

        assert_eq!(matrix.scalar_mul(2.0), expected_matrix);
        assert_eq!(&matrix * 2.0, expected_matrix);
    }

    #[test]
    fn test_sub_matrix() {
        let matrix_1 = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let matrix_2 = Matrix::new(2, 2, vec![5.0, 6.0, 7.0, 8.0]);

        let expected_data = vec![-4.0, -4.0, -4.0, -4.0];
        let res = matrix_1.sub(&matrix_2);

        assert!(res.data == expected_data, "wrong matrix data");
        assert_eq!(&matrix_1 - &matrix_2, Matrix::new(2, 2, expected_data));

        let m1 = Matrix::new(2, 2, vec![1, 2, 3, 4]);
        let m2 = Matrix::new(2, 2, vec![5, 6, 7, 8]);
        let exp_m = Matrix::new(2, 2, vec![-4, -4, -4, -4]);
        assert_eq!(&m1 - &m2, exp_m);
    }

    #[test]
    fn test_neg_trait() {
        let matrix_1 = Matrix::new(2, 2, vec![-1.0, 2.0, 3.0, -4.0]);

        let exp_matrix = Matrix::new(2, 2, vec![1.0, -2.0, -3.0, 4.0]);

        assert_eq!(-matrix_1, exp_matrix);
    }

    #[test]
    fn test_matrix_index() {
        let matrix: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);

        assert_eq!(matrix[(0, 0)], matrix.data[0]);
        assert_eq!(matrix[(0, 1)], matrix.data[1]);
        assert_eq!(matrix[(1, 0)], matrix.data[2]);
        assert_eq!(matrix[(1, 1)], matrix.data[3]);
    }

    #[test]
    fn test_matrix_index_mut() {
        let mut matrix: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);

        matrix[(0, 0)] = 6.0;
        assert_eq!(matrix[(0, 0)], 6.0);

        matrix.set(0, 1, 19.0);
        assert_eq!(matrix[(0, 1)], 19.0);
    }

    #[test]
    fn test_div_trait() {
        let m1 = Matrix::new(2, 2, vec![4, 4, 4, 4]);

        assert_eq!(&m1 / 2, Matrix::new(2, 2, vec![2, 2, 2, 2]));
    }
}
