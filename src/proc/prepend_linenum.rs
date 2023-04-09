pub fn prepend_linenum(v: &Vec<u8>, num: usize) -> Vec<u8> {
    let mut retval = v.clone();
    let preface = Vec::from(format!("{: >6} ", num).as_bytes());
    retval.splice(..0, preface);
    retval
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_a_number_to_a_line() {
        let line = Vec::from("asdf".as_bytes());
        let result = Vec::from("     7 asdf".as_bytes());
        assert_eq!(prepend_linenum(&line, 7), result)
    }

    #[test]
    fn number_is_padded() {
        let line = Vec::from("asdf".as_bytes());
        let result = Vec::from("    23 asdf".as_bytes());
        assert_eq!(prepend_linenum(&line, 23), result)
    }
}
