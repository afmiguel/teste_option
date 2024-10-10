fn test_divisor(number: i32, divisor: i32) -> i32 {
    if divisor == 0 {
        -1  // Return -1 if the divisor is zero, as division by zero is not allowed
    } else if number % divisor == 0 {
        number / divisor  // Return the result of number divided by divisor if it is a divisor
    } else {
        -1  // Return -1 if the divisor is not a divisor of the number
    }
}

fn main() {
    let number = 10;
    let divisor = 2;

    let result = test_divisor(number, divisor);

    if result != -1 {
        println!("{} divided by {} equals {}", number, divisor, result);
    } else {
        println!("{} is not divisible by {}", number, divisor);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divisor_zero() {
        assert_eq!(test_divisor(10, 0), -1);  // Test division by zero
    }

    #[test]
    fn test_valid_division() {
        assert_eq!(test_divisor(10, 2), 5);   // Test valid division
    }

    #[test]
    fn test_invalid_division() {
        assert_eq!(test_divisor(10, 3), -1);  // Test invalid divisor
    }

    #[test]
    fn test_negative_division() {
        assert_eq!(test_divisor(10, -2), -5);  // Test valid division with a negative divisor
    }
}
