// Implementation of coreutils NUM parsing, aka xdectoint()
// NUM may have a multiplier suffix: b 512, kB 1000, K 1024, MB 1000*1000, M 1024*1024,
// GB 1000*1000*1000, G 1024*1024*1024, and so on for T, P, E, Z, Y.

use ibig::UBig;

#[derive(Debug, Clone, PartialEq)]
pub enum Multiplier {
    Numeric(u32),
    Pow(u32),
    PowB10(u32),
}

#[derive(Debug, Clone, PartialEq)]
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

    pub fn to_usize(&self) -> Result<usize, String> {
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
        .ok_or("Value too large for defined data type".to_string())
    }
}

pub fn parse_num(num: &str) -> Result<NumValue, String> {
    let prefix = match num.chars().next() {
        Some('-') => Some('-'),
        Some('+') => Some('+'),
        Some(_) => None,
        None => return Err("Empty string as num".to_string()),
    };
    let value_start_index = if let Some(_) = prefix { 1 } else { 0 };
    let value_end_index = num[value_start_index..]
        .chars()
        .enumerate()
        .find(|(_, c)| !c.is_digit(10))
        .map(|(i, _)| i + value_start_index)
        .unwrap_or(num.len());
    let value: usize = num[value_start_index..value_end_index]
        .parse()
        .map_err(|_| "bad numeric value".to_string())?;
    let multiplier = parse_multiplier(&num[value_end_index..])?;

    Ok(NumValue {
        prefix: prefix,
        value: value,
        multiplier: multiplier,
    })
}

fn parse_multiplier(mult: &str) -> Result<Option<Multiplier>, String> {
    if mult.len() == 0 {
        return Ok(None);
    }
    if mult.len() > 2 {
        return Err("bad multiplier value".to_string());
    }

    let mut chars = mult.chars();
    match chars.next() {
        Some('b') => Ok(Some(Multiplier::Numeric(512))),
        Some(m) => {
            let val = match m.to_ascii_uppercase() {
                'K' => Ok(1),
                'M' => Ok(2),
                'G' => Ok(3),
                'T' => Ok(4),
                'P' => Ok(5),
                'E' => Ok(6),
                'Z' => Ok(7),
                'Y' => Ok(8),
                _ => Err("bad multiplier value".to_string()),
            }?;
            match chars.next() {
                Some('B') => Ok(Some(Multiplier::PowB10(val))),
                Some(_) => Err("bad multiplier value".to_string()),
                None => Ok(Some(Multiplier::Pow(val))),
            }
        }
        None => Err("bad multiplier value".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numvalue_to_ubig() {
        todo!();
    }

    #[test]
    fn parse_multiplier_returns_correct_result() {
        assert_eq!(
            parse_multiplier("zxc"),
            Err("bad multiplier value".to_string())
        );
        assert_eq!(parse_multiplier("b"), Ok(Some(Multiplier::Numeric(512))));
        // should just "B" be equal to 512 too?..
        for (i, mult) in ["K", "M", "G", "T", "P", "E", "Z", "Y"].iter().enumerate() {
            let str = mult.to_string();
            assert_eq!(
                parse_multiplier(&str),
                Ok(Some(Multiplier::Pow((i + 1) as u32)))
            );
            assert_eq!(
                parse_multiplier(&(str + "B")),
                Ok(Some(Multiplier::PowB10((i + 1) as u32)))
            );
        }
    }

    #[test]
    fn parse_num_correctly_parses_prefix() {
        assert_eq!(parse_num("-123").unwrap().prefix, Some('-'));
        assert_eq!(parse_num("+123").unwrap().prefix, Some('+'));
        assert_eq!(parse_num("123").unwrap().prefix, None);
        assert_eq!(parse_num("a"), Err(String::from("bad numeric value")));
    }

    #[test]
    fn parse_num_correctly_parses_value() {
        assert_eq!(parse_num("-123").unwrap().value, 123);
        assert_eq!(parse_num("+321").unwrap().value, 321);
        assert_eq!(parse_num("67").unwrap().value, 67);
    }

    #[test]
    fn parse_num_correctly_parses_suffix() {
        assert_eq!(
            parse_num("-123kB").unwrap().multiplier,
            Some(Multiplier::PowB10(1))
        );
        // What about kb suffix?
        assert_eq!(
            parse_num("+321M").unwrap().multiplier,
            Some(Multiplier::Pow(2))
        );
        assert_eq!(
            parse_num("67b").unwrap().multiplier,
            Some(Multiplier::Numeric(512))
        );
    }
}
