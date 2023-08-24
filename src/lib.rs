//! # `proyecto_bd`
//!

#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![deny(rust_2018_idioms, unsafe_code)]
use std::collections::{HashMap, HashSet};

pub use anyhow::Result;
pub use std::io::{BufRead, BufReader, BufWriter, Read, Write};
pub use std::ops::Not;

#[cfg(feature = "dhat")]
#[global_allocator]
pub static ALLOC: dhat::Alloc = dhat::Alloc;

/// Returns a `HashMap` with each columns and its unique values if the number of unique values is under 1000
///
/// # Errors
///
/// This function will return an error if the `BufferReader` reaturns an error while reading the file lines
pub fn extract_uniques(
    mut headers: Vec<String>,
    lines: std::io::Lines<BufReader<std::fs::File>>,
) -> Result<HashMap<String, HashSet<String>>, anyhow::Error> {
    let mut uniques: Vec<HashSet<String>> = (0..headers.len()).map(|_| HashSet::new()).collect();
    for line in lines {
        let line = line?;

        extract_values(&line)
            .iter()
            .enumerate()
            .for_each(|(col, val)| {
                if uniques[col].capacity() <= 1000 {
                    uniques[col].insert(val.to_string());
                } else {
                    uniques[col].clear();
                }
            });
    }
    let uniques = uniques
        .into_iter()
        .enumerate()
        .map(|(col, vals)| (std::mem::take(&mut headers[col]), vals))
        .collect::<HashMap<_, _>>();
    Ok(uniques)
}

/// Retrieves the values of a given comma separated line.
/// The function skips commas that are inside of string objetcs '"'
#[must_use]
pub fn extract_values(line: &str) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut cur = String::new();
    let mut in_str = false;
    for c in line.chars() {
        if c == '"' {
            in_str = !in_str;
            cur.push(c);
        } else if c == ',' && in_str {
            cur.push(c);
        } else if c == ',' && !in_str {
            chunks.push(std::mem::take(&mut cur));
        } else {
            cur.push(c);
        }
    }
    chunks.push(cur);
    chunks
}

/// Returns a `BufReader` and a `BufWriter` of the given paths, useful when we are
/// intending to process the contents of the reader and writing the result on the writer.
///
/// If the given writer already exists it overwrites it, else it creates it.
///
/// # Errors
///
/// This function will return an error if the given file path to read does not exist or if
/// there is any problem opening any of the write and read files
pub fn reader_writer(
    file: &str,
    write_to: &str,
) -> Result<(BufReader<std::fs::File>, BufWriter<std::fs::File>), anyhow::Error> {
    let file = BufReader::new(std::fs::OpenOptions::new().read(true).open(file)?);
    let out = BufWriter::new(
        std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(write_to)?,
    );
    Ok((file, out))
}

/// Remove carriage returns and empty lines
///
/// # Panics
///
/// Panics if there is not a single
///
/// # Errors
///
/// This function will return an error if there is a problem reading/writing to any of the files
pub fn remove_empty_lines(
    file: std::io::BufReader<std::fs::File>,
    mut out: BufWriter<std::fs::File>,
) -> Result<()> {
    let mut lines = file.lines();
    out.write_all(lines.next().unwrap()?.trim().as_bytes())?;
    out.write_all(&[b'\n'])?;
    let mut n = 2;
    let mut fin = 2;
    let mut line = lines.next();
    while let Some(line_res) = std::mem::take(&mut line) {
        let line_str = line_res?;
        let mut line_str = line_str.replace(['¢', '½'], "ó").replace('\u{a0}', " ");

        while line_str.contains("  ") {
            // println!("Removing duplicated space: {:?}", line_str);
            line_str = line_str.replace("  ", " ");
        }

        if line_str.contains(" ,") {
            // println!("Removing ' ,': {:?}", line_str);
            line_str = line_str.replace(" ,", ",");
        }

        line = if line_str.trim().is_empty().not() {
            out.write_all(line_str.trim().as_bytes())?;

            loop {
                line = lines.next();
                if let Some(Ok(ref line)) = line {
                    if line.trim().is_empty() {
                        // println!("I {n:>8}: Skipping empty line");
                        n += 1;
                        continue;
                    }
                }
                break;
            }

            if let Some(Ok(ref ref_line)) = line {
                // println!("Next: {:?}", line);

                if ref_line.trim_start().starts_with('"').not() {
                    out.write_all(&[b'\n'])?;
                    fin += 1;
                } else {
                    // println!("O {fin:>8}: Appending next line: {ref_line:?}");
                    out.write_all(ref_line.trim().as_bytes())?;
                    out.write_all(&[b'\n'])?;
                    fin += 1;
                    line = lines.next();
                }
            }
            line
        } else {
            // println!("I {n:>8}: Skipping empty line");
            lines.next()
        };

        n += 1;
    }
    println!("Initial: {n}");
    println!("  Final: {fin} (-{})", n - fin);
    Ok(())
}

#[must_use]
pub fn test() -> &'static str {
    "Hello World"
}
