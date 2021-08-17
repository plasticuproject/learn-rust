use anyhow::{Error, Result};
use std::fs::{remove_file, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

pub fn print_matches(
    content: &str,
    num: &i32,
    pattern: &str,
    mut writer: impl Write,
) -> Result<(), Error> {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "LINE# {}: {}", num, line)?;
        }
    }

    Ok(())
}

pub fn write_matches(
    content: &str,
    num: &i32,
    pattern: &str,
    outfile: &PathBuf,
) -> Result<(), Error> {
    let file_handler = OpenOptions::new()
        .create(true)
        .append(true)
        .open(outfile)
        .unwrap();
    let mut writer = BufWriter::new(file_handler);
    let num = num.to_string();
    for line in content.lines() {
        if line.contains(pattern) {
            let write_line = format!("LINE# {}: {}\n", num, line);
            writer.write(write_line.as_bytes())?;
        }
    }

    Ok(())
}

pub fn purge_file(outfile: &PathBuf) -> Result<(), Error> {
    if outfile.exists() {
        remove_file(outfile)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn print_a_match() -> Result<(), Error> {
        let mut result = Vec::new();
        let num = 1;
        print_matches("lorem ipsum\ndolor sit amet", &num, "lorem", &mut result)?;
        assert_eq!(result, b"LINE# 1: lorem ipsum\n");

        Ok(())
    }

    #[test]
    fn write_a_match() -> Result<(), Error> {
        let _file = File::create("test_write_file.txt")?;
        let outfile = PathBuf::from("test_write_file.txt");
        let num = 1;
        write_matches("lorem ipsum\ndolor sit amet", &num, "lorem", &outfile)?;
        let mut file = File::open("test_write_file.txt")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        remove_file(outfile)?;
        assert_eq!(contents, "LINE# 1: lorem ipsum\n");

        Ok(())
    }

    #[test]
    fn purge_a_file() -> Result<(), Error> {
        let mut file = File::create("test_purge_file.txt")?;
        writeln!(file, "A test\nActual content\nMore content\nAnother test")?;
        let outfile = PathBuf::from("test_purge_file.txt");
        purge_file(&outfile)?;
        match &outfile.exists() {
            false => Some(outfile),
            true => return Err(anyhow!("file was not purged")),
        };

        Ok(())
    }
}
