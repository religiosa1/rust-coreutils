// Implementation of coreutils NUM parsing, aka xdectoint()
// NUM may have a multiplier suffix: b 512, kB 1000, K 1024, MB 1000*1000, M 1024*1024,
// GB 1000*1000*1000, G 1024*1024*1024, and so on for T, P, E, Z, Y.

use super::ParseNumError;
use super::{Multiplier, NumValue};

pub fn parse_num(num: &str) -> Result<NumValue, ParseNumError> {
    let prefix = match num.chars().next() {
        Some('-') => Some('-'),
        Some('+') => Some('+'),
        Some(_) => None,
        None => return Err(ParseNumError::Empty),
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
        .map_err(|_| ParseNumError::BadNumericValue)?;
    let multiplier = parse_multiplier(&num[value_end_index..])?;

    Ok(NumValue {
        prefix: prefix,
        value: value,
        multiplier: multiplier,
    })
}

fn parse_multiplier(mult: &str) -> Result<Option<Multiplier>, ParseNumError> {
    if mult.len() == 0 {
        return Ok(None);
    }
    if mult.len() > 2 {
        return Err(ParseNumError::BadMultiplierValue(3));
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
                _ => Err(ParseNumError::BadMultiplierValue(1)),
            }?;
            match chars.next() {
                // lowercase 'b' isn't considered a valid suffix for some reason in the GNU implementation
                Some('B') => Ok(Some(Multiplier::PowB10(val))),
                Some(_) => Err(ParseNumError::BadMultiplierValue(2)),
                None => Ok(Some(Multiplier::Pow(val))),
            }
        }
        None => Err(ParseNumError::BadMultiplierValue(0)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_multiplier_returns_correct_result() {
        assert_eq!(
            parse_multiplier("zxc"),
            Err(ParseNumError::BadMultiplierValue(3))
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
        assert_eq!(parse_num("a"), Err(ParseNumError::BadNumericValue));
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
            parse_num("-123a"),
            Err(ParseNumError::BadMultiplierValue(1)),
        );
        assert_eq!(
            parse_num("-123Kb"),
            Err(ParseNumError::BadMultiplierValue(2)),
        );
        assert_eq!(
            parse_num("-123kBs"),
            Err(ParseNumError::BadMultiplierValue(3)),
        );
        assert_eq!(
            parse_num("-123kB").unwrap().multiplier,
            Some(Multiplier::PowB10(1))
        );
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
