/// Utilities module.
mod utils {
    /// Returns the sum of two integers.
    ///
    /// # Arguments
    ///
    /// * `a` - The first integer.
    /// * `b` - The second integer.
    ///
    /// # Example
    /// ```
    /// let sum = utils::sum(4, 5);
    /// assert_eq!(sum, 9);
    /// ```
    pub fn sum(a: i32, b: i32) -> i32 {
        a + b
    }
    /// Returns the subtact of two floats.
     ///
    /// # Arguments
    ///
    /// * `a` - The first float.
    /// * `b` - The second float.
    ///
    /// # Example
    /// ```
    /// let sub = utils::subtract(6f64, 3f64);
    /// assert_eq!(subtract, 3f64);
    /// ```
    pub fn subtract(a: f64, b: f64) -> f64 {
        a - b
    }
    /// Returns the multiply of two floats.
     ///
    /// # Arguments
    ///
    /// * `a` - The first float.
    /// * `b` - The second float.
    ///
    /// # Example
    /// ```
    /// let mult = utils::multiply(-2f64, -5f64);
    /// assert_eq!(multiply, 10f64);
    /// ```
    pub fn multiply(a: f64, b: f64) -> f64 {
        a * b
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_sum() {
            assert_eq!(sum(2, 2), 4);
            assert_eq!(sum(-2, 2), 0);
            assert_eq!(sum(0, 0), 0);
        }
        #[test]
        fn test_subtract() {
            assert_eq!(subtract(5f64, 2f64), 3f64);
            assert_eq!(subtract(-2f64, -2f64), 0f64);
            assert_eq!(subtract(0f64, -2f64), 2f64);
            assert_eq!(subtract(0f64, 0f64), 0f64);
        }
        #[test]
        fn test_multiply() {
            assert_eq!(multiply(2f64, 3f64), 6f64);
            assert_eq!(multiply(-2f64, 2f64), -4f64);
            assert_eq!(multiply(-2f64, -2f64), 4f64);
            assert_eq!(multiply(-1f64, 0f64), 0f64);
        }
    }
}

fn main() {
    let sum = utils::sum(4, 5);
    let sub = utils::subtract(6f64, 3f64);
    let mult = utils::multiply(-2f64, -5f64);
    println!("Sum = {}", sum);
    println!("Substract = {}", sub);
    println!("Multiply = {}", mult);
}