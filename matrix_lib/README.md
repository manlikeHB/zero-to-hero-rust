# 🔢 Matrix Library

A generic matrix implementation with operator overloading and comprehensive mathematical operations.

## 🎯 Learning Objectives

- **Generics**: Creating types that work with any numeric type
- **Traits**: Implementing and using standard library traits
- **Operator Overloading**: Using `std::ops` traits (`Add`, `Mul`, etc.)
- **Trait Bounds**: Constraining generic types with trait requirements
- **Index Operators**: Implementing `Index` and `IndexMut` for ergonomic access
- **Testing**: Writing comprehensive unit tests

## ⚡ Features

- Generic over numeric types (`i32`, `f64`, etc.)
- Matrix addition and subtraction
- Matrix multiplication
- Scalar multiplication and division
- Transpose operation
- Negation
- Indexed access with `matrix[(row, col)]`
- Full operator overloading (`+`, `-`, `*`, `/`, `-`)

## 🚀 Usage

```rust
use matrix_lib::Matrix;

fn main() {
    // Create matrices
    let m1 = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    let m2 = Matrix::new(2, 2, vec![5.0, 6.0, 7.0, 8.0]);
    
    // Matrix operations
    let sum = &m1 + &m2;          // Addition
    let diff = &m1 - &m2;         // Subtraction
    let product = &m1 * &m2;      // Multiplication
    let scaled = &m1 * 2.0;       // Scalar multiplication
    let divided = &m1 / 2.0;      // Scalar division
    let transposed = m1.transpose();
    let negated = -m1;
    
    // Indexed access
    println!("Element at (0,1): {}", m1[(0, 1)]);
}
```

## 🔑 Key Concepts Demonstrated

### Generic Struct with Trait Bounds
```rust
#[derive(Debug, PartialEq, Clone)]
struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}
```

### Operator Overloading for Addition
```rust
impl<T> Add for &Matrix<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Matrix<T>;

    fn add(self, other: &Matrix<T>) -> Matrix<T> {
        assert!(self.rows == other.rows && self.cols == other.cols);
        
        let res = self.data.iter()
            .zip(&other.data)
            .map(|(a, b)| *a + *b)
            .collect();
        
        Matrix::new(self.rows, self.cols, res)
    }
}
```

### Matrix Multiplication
```rust
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
```

### Custom Indexing
```rust
impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (i, j) = index;
        &self.data[i * self.cols + j]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (i, j) = index;
        &mut self.data[i * self.cols + j]
    }
}
```

## 💡 What I Learned

1. **Generic Programming**: Writing code that works with multiple types
2. **Trait Bounds**: Using `where` clauses to constrain generic types
3. **Zero Trait**: Using `num_traits::Zero` for generic identity elements
4. **Reference Patterns**: Implementing operators for references (`&Matrix`)
5. **2D to 1D Mapping**: Converting `(row, col)` to flat array index
6. **Associated Types**: Using `type Output` in trait implementations
7. **Method vs Trait Implementation**: When to use inherent methods vs traits

## 🧪 Comprehensive Testing

```rust
#[test]
fn test_multiply() {
    let matrix_1 = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    let matrix_2 = Matrix::new(2, 2, vec![5.0, 6.0, 7.0, 8.0]);
    let expected = Matrix::new(2, 2, vec![19.0, 22.0, 43.0, 50.0]);
    
    assert_eq!(&matrix_1 * &matrix_2, expected);
}

#[test]
fn test_matrix_index_mut() {
    let mut matrix = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    matrix[(0, 0)] = 6.0;
    assert_eq!(matrix[(0, 0)], 6.0);
}
```

## 🔄 Possible Improvements

- [ ] Determinant calculation
- [ ] Matrix inversion
- [ ] LU decomposition
- [ ] Eigenvalues and eigenvectors
- [ ] Row and column operations
- [ ] Submatrix extraction
- [ ] Element-wise operations
- [ ] Support for complex numbers
- [ ] Sparse matrix optimization

## 📚 Relevant Rust Book Chapters

- [Chapter 10: Generic Types, Traits, and Lifetimes](https://doc.rust-lang.org/book/ch10-00-generics.html)
- [Rust by Example: Generics](https://doc.rust-lang.org/rust-by-example/generics.html)

## 📦 Dependencies

```toml
[dependencies]
num-traits = "0.2"
```

---

**Status**: ✅ Completed | **Difficulty**: Intermediate 