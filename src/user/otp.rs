use rand::random;
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq)]
pub struct Otp(u32);

impl Otp {
    pub fn random() -> Self {
        let mut values: [u8; 8] = [10, 10, 10, 10, 10, 10, 10, 10];
        for value in values.iter_mut() {
            while *value > 9 {
                *value = random();
            }
        }
        Self(compose(values))
    }

    pub fn from_str(value: &str) -> Option<Self> {
        let mut digits = value
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as u8))
            .collect::<Vec<u8>>();
        if digits.len() < 8 {
            digits.reverse();
            while digits.len() < 8 {
                digits.push(0);
            }
            digits.reverse();
        }
        let Ok(array) = digits.try_into() else {
            return None;
        };
        Some(Self(compose(array)))
    }

    pub fn as_u32(&self) -> u32 {
        self.0
    }

    /// Decomposes the OTP into 8 4-bit values.
    fn decompose(&self) -> [u8; 8] {
        let n = self.0;
        [
            ((n >> 28) & 0xF) as u8,
            ((n >> 24) & 0xF) as u8,
            ((n >> 20) & 0xF) as u8,
            ((n >> 16) & 0xF) as u8,
            ((n >> 12) & 0xF) as u8,
            ((n >> 8) & 0xF) as u8,
            ((n >> 4) & 0xF) as u8,
            (n & 0xF) as u8,
        ]
    }
}

fn compose(digits: [u8; 8]) -> u32 {
    let mut packed: u32 = 0;
    for (i, digit) in digits.into_iter().enumerate() {
        let shifted = (digit as u32) << (28 - (i * 4));
        packed |= shifted;
    }
    packed
}

impl Display for Otp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let digits = self.decompose();
        let string = format!(
            "{}{}{}{}{}{}{}{}",
            digits[0], digits[1], digits[2], digits[3], digits[4], digits[5], digits[6], digits[7]
        );
        if string.chars().into_iter().collect::<Vec<_>>().len() > 8 {
            unreachable!("OTP was generated with invalid u32 seed (a digit exceeded 9)");
        }
        write!(f, "{string}")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn to_string() {
        let otp = Otp(1649418903);
        assert_eq!(otp.to_string(), "62502297");
    }

    #[test]
    fn random() {
        let otp = Otp::random();
        print!("random is: {otp}");
        assert_eq!(otp.to_string().len(), 8);
    }

    #[test]
    fn from_str_8_digits() {
        let otp = Otp::from_str("62502297").unwrap();
        assert_eq!(otp.to_string(), "62502297");
        assert_eq!(otp.0, 1649418903);
    }

    #[test]
    fn from_str_7_digits() {
        let otp = Otp::from_str("2502297").unwrap();
        assert_eq!(otp.to_string(), "02502297");
        assert_eq!(otp.0, 38806167);
    }

    #[test]
    fn from_str_1_digit() {
        let otp = Otp::from_str("1").unwrap();
        assert_eq!(otp.to_string(), "00000001");
        assert_eq!(otp.0, 1);
    }

    #[test]
    fn from_str_0_digits() {
        let otp = Otp::from_str("").unwrap();
        assert_eq!(otp.to_string(), "00000000");
        assert_eq!(otp.0, 0);
    }
}
