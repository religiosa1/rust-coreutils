#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Alphabet {
    symbols: &'static [u8; 32],
    pub padding: Option<u8>,
}
impl Alphabet {
    pub fn symbol(&self, index: u8) -> u8 {
        self.symbols.get(index as usize).copied().unwrap()
    }
    pub fn value(&self, char: u8) -> Result<Option<u8>, String> {
        if let Some(c) = self.padding {
            if c == char {
                return Ok(None);
            }
        }
        for (i, c) in self.symbols.iter().enumerate() {
            if *c == char {
                return Ok(Some(i as u8));
            }
        }
        Err("Bad char".into())
    }
}

pub const RFC4648_ALPHABET: Alphabet = Alphabet {
    symbols: b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567",
    padding: Some(b'='),
};

/* some day later
pub const CROCKFORD: Alphabet = Alphabet {
    symbols: b"0123456789ABCDEFGHJKMNPQRSTVWXYZ",
    padding: Some(b'='),
    decoding_map: Some(CROCKFORD_REVERSE_MAP),
};
const CROCKFORD_REVERSE_MAP: &[(u8, u8)] = &[
    (b'o', b'0'),
    (b'O', b'0'),
    (b'i', b'1'),
    (b'I', b'1'),
    (b'l', b'1'),
    (b'L', b'1'),
];
 */
