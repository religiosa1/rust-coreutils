#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Alphabet {
    symbols: &'static [u8; 32],
    pub padding: Option<u8>,
}
impl Alphabet {
    pub fn symbol(&self, index: u8) -> u8 {
        self.symbols.get(index as usize).copied().unwrap()
    }
}

pub const RFC4648_ALPHABET: Alphabet = Alphabet {
    symbols: b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567",
    padding: Some(b'='),
};
