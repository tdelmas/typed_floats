use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum InvalidNumber {
    NaN,
    Zero,
    Negative,
    Positive,
    Infinite,
}

impl Error for InvalidNumber {}

impl fmt::Display for InvalidNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid number")
    }
}

#[derive(Debug)]
struct NumberIsNaN;

impl Error for NumberIsNaN {}

impl fmt::Display for NumberIsNaN {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Number is NaN")
    }
}

#[derive(Debug)]
struct NumberIsZero;

impl Error for NumberIsZero {}

impl fmt::Display for NumberIsZero {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Number is zero")
    }
}

#[derive(Debug)]
struct NumberIsNegative;

impl Error for NumberIsNegative {}

impl fmt::Display for NumberIsNegative {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Number is negative")
    }
}

#[derive(Debug)]
struct NumberIsPositive;

impl Error for NumberIsPositive {}

impl fmt::Display for NumberIsPositive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Number is positive")
    }
}

#[derive(Debug)]
struct NumberIsInfinite;

impl Error for NumberIsInfinite {}

impl fmt::Display for NumberIsInfinite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Number is infinite")
    }
}
