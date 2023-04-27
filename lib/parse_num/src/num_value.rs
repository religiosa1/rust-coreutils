use ibig::UBig;

#[derive(Debug, Clone, PartialEq)]
pub enum Multiplier {
    Numeric(u32),
    Pow(u32),
    PowB10(u32),
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct NumValue {
    pub prefix: Option<char>,
    pub value: usize,
    pub multiplier: Option<Multiplier>,
}

impl NumValue {
    pub fn to_ubig(&self) -> UBig {
        match self.multiplier {
            None => UBig::from(self.value),
            Some(Multiplier::Numeric(n)) => UBig::from(self.value) * n,
            Some(Multiplier::Pow(n)) => {
                UBig::from(self.value) * UBig::from(1024_usize).pow(n as usize)
            }
            Some(Multiplier::PowB10(n)) => {
                UBig::from(self.value) * UBig::from(1000_usize).pow(n as usize)
            }
        }
    }

    pub fn to_usize(&self) -> Option<usize> {
        match self.multiplier {
            None => Some(self.value),
            Some(Multiplier::Numeric(n)) => self.value.checked_mul(n as usize),
            Some(Multiplier::Pow(n)) => 1024_usize
                .checked_pow(n)
                .and_then(|m| self.value.checked_mul(m)),
            Some(Multiplier::PowB10(n)) => 1000_usize
                .checked_pow(n)
                .and_then(|mul| self.value.checked_mul(mul)),
        }
    }
}

impl From<usize> for NumValue {
    fn from(value: usize) -> Self {
        Self {
            prefix: None,
            value: value,
            multiplier: None,
        }
    }
}

impl From<i32> for NumValue {
    fn from(value: i32) -> Self {
        Self {
            prefix: if value < 0 { Some('-') } else { None },
            value: value.abs() as usize,
            multiplier: None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn numvalue_to_ubig() {
        assert_eq!(
            NumValue {
                prefix: None,
                value: 3,
                multiplier: None
            }
            .to_ubig(),
            UBig::from(3_u32)
        );
        assert_eq!(
            NumValue {
                prefix: None,
                value: 3,
                multiplier: Some(Multiplier::Numeric(3))
            }
            .to_ubig(),
            UBig::from(9_u32)
        );
        assert_eq!(
            NumValue {
                prefix: None,
                value: 3,
                multiplier: Some(Multiplier::Pow(3))
            }
            .to_ubig(),
            UBig::from(3_u32 * 1024_u32.pow(3))
        );
        assert_eq!(
            NumValue {
                prefix: None,
                value: 3,
                multiplier: Some(Multiplier::PowB10(3))
            }
            .to_ubig(),
            UBig::from(3_u32 * 1000_u32.pow(3))
        );
    }

    #[test]
    fn numvalue_to_usize() {
        assert_eq!(
            NumValue {
                prefix: None,
                value: 3,
                multiplier: None
            }
            .to_usize(),
            Some(3)
        );
        assert_eq!(
            NumValue {
                prefix: None,
                value: 3,
                multiplier: Some(Multiplier::Numeric(3))
            }
            .to_usize(),
            Some(9)
        );
        assert_eq!(
            NumValue {
                prefix: None,
                value: 3,
                multiplier: Some(Multiplier::Pow(3))
            }
            .to_usize(),
            Some(3 * 1024_usize.pow(3))
        );
        assert_eq!(
            NumValue {
                prefix: None,
                value: 3,
                multiplier: Some(Multiplier::PowB10(3))
            }
            .to_usize(),
            Some(3 * 1000_usize.pow(3))
        );
    }

    #[test]
    fn numvalue_to_usize_overflow() {
        assert_eq!(
            NumValue {
                prefix: None,
                value: 3,
                multiplier: Some(Multiplier::Pow(15))
            }
            .to_usize(),
            None
        );
    }

    #[test]
    fn from_usize() {
        assert_eq!(
            NumValue::from(123_usize),
            NumValue {
                prefix: None,
                value: 123,
                multiplier: None
            }
        );
    }

    #[test]
    fn from_i32() {
        assert_eq!(
            NumValue::from(123),
            NumValue {
                prefix: None,
                value: 123,
                multiplier: None
            }
        );
        assert_eq!(
            NumValue::from(-123),
            NumValue {
                prefix: Some('-'),
                value: 123,
                multiplier: None
            }
        );
    }
}
