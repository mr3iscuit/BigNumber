use std::io;
use crate::BigNumber;
use crate::Sign;

#[cfg(test)]
mod tests {
    use super::*;

    fn subtraction(a :&str, b :&str, c :&str) {
        let mut number1 = BigNumber::from_string(&a);
        let mut number2 = BigNumber::from_string(&b);
        let number3 = BigNumber::from_string(&c);

        number1.subtract(&mut number2);

        assert_eq!(number1.digits, number3.digits);
    }

    fn add(a :&str, b :&str, c :&str) {
        let mut number1 = BigNumber::from_string(&a);
        let mut number2 = BigNumber::from_string(&b);
        let number3 = BigNumber::from_string(&c);

        number1.print();
        number2.print();
        number1.add(&mut number2);
        number1.print();
        number3.print();

        assert_eq!(number1.digits, number3.digits);
    }

    #[test]
    fn test_from_string_positive() {
        let input = "123";
        let expected_digits = vec![3, 2, 1];
        let expected_sign = Sign::Positive;
        let num = BigNumber::from_string(input);

        assert_eq!(num.digits, expected_digits);
        assert_eq!(num.sign, expected_sign);
    }

    #[test]
    fn test_from_string_negative() {
        let input = "-123";
        let expected_digits = vec![3, 2, 1];
        let expected_sign = Sign::Negative;
        let num = BigNumber::from_string(input);

        assert_eq!(num.digits, expected_digits);
        assert_eq!(num.sign, expected_sign);
    }

    #[test]
    fn subtraction_1() {
        subtraction("11", "13", "-2");
    }

    #[test]
    fn subtraction_2() {
        subtraction("1000000000", "1000000000", "0");
    }

    #[test]
    fn subtraction_3() {
        subtraction("9999999999999999", "9999999999999999", "0");
    }

    #[test]
    fn subtraction_4() {
        subtraction("99999999999999999999999999999999", "99999999999999999999999999999998", "1");
    }

    #[test]
    fn subtraction_5() {
        subtraction("0", "0", "0");
    }

    #[test]
    fn add_1() {
        add("11", "13", "24");
    }

    #[test]
    fn add_2() {
        add("1000000000", "1000000000", "2000000000");
    }

    #[test]
    fn add_3() {
        add("5", "-5", "0");
    }

    #[test]
    fn add_4() {
        add("99999999999999999999999999999999", "-99999999999999999999999999999998", "1");
    }

    #[test]
    fn add_5() {
        add("0", "0", "0");
    }




    // Add more test functions for other methods and behaviors
}
