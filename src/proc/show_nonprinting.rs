use crate::proc::Processor;

/*
 * Behavior for the upper part of a byte (beyond the ASCII range):
 *
 * Unix power tools. 25.7 Show Non-Printing Characters with cat -v or od -c.
 * "cat -v has its own symbol for characters outside the ASCII range with their
 * high bits set, also called metacharacters. cat -v prints those as M- followed
 * by another character. There are two of them in the cat -v output: M-^? and
 * M-a . To get a metacharacter, you add 200 octal. "Say what?" Let's look at
 * M-a first. The octal value of the letter a is 141. When cat -v prints M-a ,
 * it means the character you get by adding 141+200, or 341 octal. You can
 * decode the character cat prints as M-^? in the same way. The ^? stands for
 * the DEL character, which is octal 177. Add 200+177 to get 377 octal. "
 */

pub struct ShowNonprinting;
impl ShowNonprinting {
    pub fn new() -> ShowNonprinting {
        ShowNonprinting
    }
}
impl Processor for ShowNonprinting {
    fn proc(&mut self, line: Vec<u8>) -> Option<Vec<u8>> {
        let mut bytes = Vec::new();
        for b in line {
            let mut c = b;
            if c >= 0x80 {
                bytes.push(b'M');
                bytes.push(b'-');
                c -= 0x80;
            }
            match c {
                0x00..=0x1F if c != b'\t' && c != b'\n' => {
                    bytes.push(b'^');
                    bytes.push(c + 0x40);
                }
                0x7F => {
                    bytes.push(b'^');
                    bytes.push(b'?');
                }
                _ => bytes.push(c),
            }
        }
        Some(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn control_chars_are_carret_encoded() {
        let mut p = ShowNonprinting::new();
        assert_eq!(
            p.proc(vec![0x00, 0x01, 0x02, b'\t', 0x1D, 0x1E, 0x1F, b'\n', 0x7F]),
            Some(b"^@^A^B\t^]^^^_\n^?".to_vec())
        );
    }

    #[test]
    fn normal_chars_are_left_in_place() {
        let mut p = ShowNonprinting::new();
        let str = b"A\ts1\n".to_vec();
        assert_eq!(p.proc(str.clone()), Some(str.clone()));
    }

    #[test]
    fn chars_above_0x80_are_metaencoded() {
        let mut p = ShowNonprinting::new();

        let str = vec![0x80, 0x81, 0x82, b'\t', 0xFD, 0xFE, 0xFF];
        assert_eq!(
            p.proc(str.clone()),
            Some(b"M-^@M-^AM-^B\tM-}M-~M-^?".to_vec())
        );
    }
}
