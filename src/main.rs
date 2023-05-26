use std::io;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Sign {
    Positive,
    Negative,
}

#[derive(Debug, Clone)]
struct BigNumber {
    digits: Vec<u32>,
    sign: Sign,
}

#[inline]
fn swap_sign_with_other(a: &mut BigNumber, b: &mut BigNumber) {
    let temp_sign = a.sign;
    a.sign = b.sign;
    b.sign = temp_sign;
}

impl BigNumber {
    fn make_abs(&mut self) {
        self.sign = Sign::Positive;
    }

    fn shift_left(&mut self, n: usize) {
        for _ in 0..n {
            self.digits.insert(0, 0);
        }
    }

    fn shift_right(&mut self, n: usize) {
        for _ in 0..n {
            if !self.digits.is_empty() {
                self.digits.remove(0);
            }
        }

        if self.digits.is_empty() {
            self.digits.push(0);
        }
    }

    fn swap_digits(&mut self, other: &mut BigNumber) {
        std::mem::swap(&mut self.digits, &mut other.digits);
    }

    fn from_string(input: &str) -> Self {
        let (sign, number_str) = match input.chars().next() {
            Some('-') => (Sign::Negative, &input[1..]),
            _ => (Sign::Positive, input),
        };

        let digits: Vec<u32> = number_str
            .chars()
            .rev()
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        BigNumber { digits, sign }
    }

    fn is_greater_than_or_equal_to(&self, other: &BigNumber) -> bool {
        if self.digits.len() > other.digits.len() {
            return true;
        } else if self.digits.len() < other.digits.len() {
            return false;
        }

        for (self_digit, other_digit) in self.digits.iter().zip(other.digits.iter()) {
            if self_digit > other_digit {
                return true;
            } else if self_digit < other_digit {
                return false;
            }
        }

        false
    }

    fn subtract(&mut self, other: &mut BigNumber) {
        let self_is_greater = self.is_greater_than_or_equal_to(other);
        if !self_is_greater {
            self.swap_digits(other);
            swap_sign_with_other(self, other);
        }

        if self.sign == Sign::Positive && self.sign == Sign::Positive {
            self._subtract(other);
            return;
        }

        self._add(other);
    }

    fn _subtract(&mut self, other: &BigNumber) {
        let mut borrow = 0;
        for i in 0..self.digits.len() {
            let other_digit = if i < other.digits.len() {
                other.digits[i]
            } else {
                0
            };
            let mut diff: i32 = self.digits[i] as i32 - other_digit as i32 - borrow;
            if diff < 0 {
                diff += 10;
                borrow = 1;
            } else {
                borrow = 0;
            }
            self.digits[i] = diff as u32;
        }
        self.normalize();
        if self.digits.len() == 0 {
            self.digits = vec![0 as u32];
        }
    }

    fn add(&mut self, other: &mut BigNumber) {
        if self.sign != other.sign {
            if !self.is_greater_than_or_equal_to(other) {
                self.swap_digits(other);
            }

            self._subtract(other);
            return;
        }
        self._add(other);
    }

    fn _add(&mut self, other: &BigNumber) {
        let mut carry = 0;
        let max_len = self.digits.len().max(other.digits.len());

        // Extend the length of self.digits if necessary
        self.digits.resize(max_len, 0);

        for i in 0..max_len {
            let self_digit = if i < self.digits.len() {
                self.digits[i]
            } else {
                0
            };

            let other_digit = if i < other.digits.len() {
                other.digits[i]
            } else {
                0
            };

            let sum = self_digit + other_digit + carry;
            self.digits[i] = sum % 10;
            carry = sum / 10;
        }

        if carry > 0 {
            self.digits.push(carry);
        }
    }

    fn multiply_by_int(&mut self, other: i32) {
        let mut carry = 0;

        for digit in &mut self.digits {
            let product = *digit as i32 * other + carry;
            *digit = (product % 10) as u32;
            carry = product / 10;
        }

        while carry > 0 {
            self.digits.push((carry % 10) as u32);
            carry /= 10;
        }

        self.normalize();
    }

    fn multiply(&mut self, other: &mut BigNumber) -> BigNumber {
        let mut result = BigNumber {
            digits: vec![0; self.digits.len() + other.digits.len()],
            sign: Sign::Positive,
        };

        for (i, self_digit) in self.digits.iter().enumerate() {
            let mut carry = 0;

            for (j, other_digit) in other.digits.iter().enumerate() {
                let product = self_digit * other_digit + result.digits[i + j] + carry;
                result.digits[i + j] = product % 10;
                carry = product / 10;
            }

            if carry > 0 {
                result.digits[i + other.digits.len()] += carry;
            }
        }

        result.normalize();
        self.digits = result.digits;
        self.sign = result.sign;
        self.clone()
    }

    fn normalize(&mut self) {
        while let Some(&digit) = self.digits.last() {
            if digit == 0 {
                self.digits.pop();
            } else {
                break;
            }
        }
    }

    fn print(&self) {
        if self.digits.is_empty() {
            println!("0");
        } else {
            if self.sign == Sign::Negative {
                print!("-");
            }
            for &digit in self.digits.iter().rev() {
                print!("{}", digit);
            }
            println!();
        }
    }

    fn is_prime(&self) -> bool {
        if self.digits.len() == 1 && self.digits[0] <= 1 {
            return false;
        }

        let two = BigNumber::from_string("2");
        let mut divisor = two.clone();

        while divisor.is_less_than(&self.sqrt()) {
            if self.is_divisible_by(&mut divisor) {
                return false;
            }
            divisor.print();

            divisor.add(&mut BigNumber::from_string("1"));
        }

        true
    }

    // Helper method to calculate the square root of the number
    fn sqrt(&self) -> BigNumber {
        let mut x = self.clone();
        let mut y = BigNumber::from_string("1");

        while y.is_less_than_or_equal_to(&x) {
            x.shift_right(1);
            y.shift_left(1);
        }

        while y.is_greater_than_or_equal_to(&x) {
            y.subtract(&mut x);
            x.shift_right(1);
            y.shift_left(1);
            y.shift_left(1);
        }

        x
    }

    // Helper method to check if the number is divisible by another number
    fn is_divisible_by(&self, divisor: &mut BigNumber) -> bool {
        if divisor.is_zero() {
            panic!("Division by zero");
        }

        let mut dividend = self.clone();
        dividend.sign = Sign::Positive;

        while dividend.is_greater_than_or_equal_to(divisor) {
            let mut quotient = dividend.divide(divisor);
            let mut remainder = dividend.clone();
            remainder.subtract(&mut quotient.multiply(divisor));

            if remainder.is_zero() {
                return true;
            }

            dividend = remainder;
        }

        false
    }

    // Helper method to check if the number is zero
    fn is_zero(&self) -> bool {
        self.digits.len() == 1 && self.digits[0] == 0
    }

    // fn is_equal_to(&self, other: &BigNumber) -> bool {
    //     self.digits == other.digits && self.sign == other.sign
    // }

    // fn modulo(&self, divisor: &BigNumber) -> BigNumber {
    //     let mut quotient = self.divide(divisor);
    //     let mut remainder = self.clone();
    //     remainder.subtract(&mut quotient.multiply(&mut divisor.clone()));
    //     remainder
    // }

    fn is_positive(&self) -> bool {
        self.sign == Sign::Positive
    }

    fn is_negative(&self) -> bool {
        self.sign == Sign::Negative
    }

    fn is_less_than_or_equal_to(&self, other: &BigNumber) -> bool {
        if self.is_negative() && other.is_positive() {
            return true;
        } else if self.is_positive() && other.is_negative() {
            return false;
        }

        let self_len = self.digits.len();
        let other_len = other.digits.len();

        if self_len < other_len {
            return true;
        } else if self_len > other_len {
            return false;
        }

        for (&self_digit, &other_digit) in self.digits.iter().rev().zip(other.digits.iter().rev()) {
            if self_digit < other_digit {
                return true;
            } else if self_digit > other_digit {
                return false;
            }
        }

        true
    }

    fn is_less_than(&self, other: &BigNumber) -> bool {
        if self.sign != other.sign {
            return self.sign == Sign::Negative;
        }

        if self.digits.len() < other.digits.len() {
            return self.sign == Sign::Positive;
        } else if self.digits.len() > other.digits.len() {
            return self.sign == Sign::Negative;
        }

        for (&self_digit, &other_digit) in self.digits.iter().rev().zip(other.digits.iter().rev()) {
            if self_digit < other_digit {
                return self.sign == Sign::Positive;
            } else if self_digit > other_digit {
                return self.sign == Sign::Negative;
            }
        }

        false
    }

    fn divide(&self, divisor: &BigNumber) -> BigNumber {
        if divisor.is_zero() {
            panic!("Division by zero");
        }

        let mut quotient = BigNumber {
            digits: vec![0; self.digits.len()],
            sign: Sign::Positive,
        };

        let mut remainder = self.clone();
        remainder.sign = Sign::Positive;

        let divisor_is_negative = divisor.is_negative();
        let divisor_copy = divisor.clone();

        while remainder.is_greater_than_or_equal_to(divisor) {
            let mut count = BigNumber::from_string("1");
            let mut temp_divisor = divisor_copy.clone();

            while temp_divisor.is_less_than_or_equal_to(&remainder) {
                temp_divisor.shift_left(1);
                count.shift_left(1);
            }

            temp_divisor.shift_right(1);
            count.shift_right(1);

            remainder.subtract(&mut temp_divisor.clone());
            quotient.add(&mut count);
        }

        if divisor_is_negative {
            quotient.sign = match quotient.sign {
                Sign::Positive => Sign::Negative,
                Sign::Negative => Sign::Positive,
            };
        }

        quotient.normalize();
        quotient
    }
}

fn main() {
    let mut num = BigNumber::from_string("36");
    let mut num2 = BigNumber::from_string("6");
    // let is_prime = num.is_prime();
    // println!("Is prime? {}", is_prime);
    let is_divisible = num.is_divisible_by(&mut num2);
    println!("Is divisible by 6 {}", is_divisible);
}
