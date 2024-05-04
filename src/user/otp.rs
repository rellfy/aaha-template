use rand::random;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Otp(u32);

impl Otp {
    pub fn random() -> Self {
        Self(random())
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

impl Display for Otp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let digits = self.decompose();
        let full = format!(
            "{}{}{}{}{}{}{}{}",
            digits[0], digits[1], digits[2], digits[3], digits[4], digits[5], digits[6], digits[7]
        );
        // The string above may be longer than 8 digits as 4 bit values go up to 16,
        // therefore it has to be contained into a length of 8.
        let contained = &full[0..8];
        write!(f, "{contained}")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn to_string() {
        let otp = Otp(133742069);
        assert_eq!(otp.to_string(), "07158111");
    }

    #[test]
    fn random() {
        let otp = Otp::random();
        assert_eq!(otp.to_string().len(), 8);
    }
}
