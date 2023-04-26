use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};

/// reverse lines characterwise
#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
pub struct Args {
    pub file: Vec<String>,
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();
    let mut output = std::io::stdout();
    if args.file.len() == 0 {
        rev(std::io::stdin(), &mut output)?;
    } else {
        for filename in args.file {
            let file = File::open(filename)?;
            rev(file, &mut output)?;
        }
    }
    Ok(())
}

fn rev<R: Read, W: Write>(input: R, output: &mut W) -> Result<(), std::io::Error> {
    let reader = BufReader::new(input);
    let mut first = true;
    for line in reader.split(b'\n') {
        let mut line = line?;
        line.reverse();
        if !first {
            output.write(b"\n")?;
        }
        output.write(&line)?;
        first = false;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn reverses_stirngs() {
        let data = b"some data\nhere and there";
        let input = Cursor::new(data);
        let mut output = Cursor::new(Vec::<u8>::new());
        rev(input, &mut output).unwrap();
        assert_eq!(output.into_inner(), b"atad emos\nereht dna ereh".to_vec());
    }
}
