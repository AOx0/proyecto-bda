use std::{collections::HashMap, fs::OpenOptions};

use anyhow::{anyhow, Result};
use proyecto_bd::*;

fn main() -> Result<()> {
    #[cfg(feature = "dhat")]
    let _profiler = dhat::Profiler::new_heap();

    let file = std::env::args()
        .nth(1)
        .ok_or(anyhow!("Missing file path `cargo run <PATH>`"))?;

    std::fs::create_dir_all("./results")?;

    // Primero borramos todas las lineas vacías (incluyendo \r)
    if !std::path::PathBuf::from("./results/out_temp.csv").exists() {
        let (file, out) = reader_writer(&file, "./results/out_temp.csv")?;
        remove_empty_lines(file, out)?;
    } else {
        println!("Warning: File './results/out_temp.csv' already exists, skipping '{file}' cleanup")
    }

    // Obtenemos los valores únicos
    let (file, mut out) = reader_writer("./results/out_temp.csv", "./results/uniques.json")?;
    let (headers, uniques) = {
        let mut lines = file.lines();

        let headers = lines.next().unwrap()?.to_owned();
        let headers = headers
            .split(',')
            .map(|a| a.to_owned())
            .collect::<Vec<String>>();

        let u = extract_uniques(headers.as_slice(), lines)?;
        (headers, u)
    };

    let mut uniques: HashMap<_, Vec<_>> = uniques
        .into_iter()
        .map(|(k, v)| (k, v.into_iter().collect()))
        .collect();

    uniques.values_mut().for_each(|v| v.sort());

    write!(out, "{:#?}", uniques)?;
    println!("Wrote unique values to './results/uniques.json'");

    // Reemplazamos todos los valores repetidos por el id
    let (file, mut out) = reader_writer("./results/out_temp.csv", "./results/out.csv")?;

    out.write_all("id,".as_bytes())?;
    out.write_all(headers.join(",").as_bytes())?;
    out.write_all("\n".as_bytes())?;
    for (n, line) in file.lines().skip(1).enumerate() {
        let line = line?;
        let mut values = extract_values(&line);
        values.insert(0, format!("{n}"));

        for (col, value) in values.iter_mut().skip(1).enumerate() {
            for (id, vals) in uniques
                .get(headers[col].as_str())
                .unwrap()
                .iter()
                .enumerate()
                .rev()
            {
                if value == vals {
                    *value = format!("{id}");
                }
            }
        }

        out.write_all(values.join(",").as_bytes())?;
        out.write_all(&[b'\n'])?;
    }

    println!("Wrote csv with replaced foreing keys to './results/out.csv'");

    for header in headers.iter() {
        if uniques[header.as_str()].is_empty().not() {
            let path = format!("./results/{header}.csv");
            let mut file = OpenOptions::new()
                .truncate(true)
                .write(true)
                .create(true)
                .open(path.as_str())?;

            file.write_all(format!("id_{header},{header}\n").as_bytes())?;
            for (i, value) in uniques[header.as_str()].iter().enumerate() {
                file.write_all(format!("{i},{value}\n").as_bytes())?
            }
            println!("Wrote to {path} header values {header} with fields (id_{header}, {header})");
        }
    }

    Ok(())
}
