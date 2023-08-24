use std::{collections::HashMap, fs::OpenOptions};

use anyhow::{anyhow, Result};
use proyecto_bd::*;

const SPACING: usize = 12;

fn mes_a_int(mes: &str) -> usize {
    match mes {
        "NA" => 0,
        "Enero" => 1,
        "Febrero" => 2,
        "Marzo" => 3,
        "Abril" => 4,
        "Mayo" => 5,
        "Junio" => 6,
        "Julio" => 7,
        "Agosto" => 8,
        "Septiembre" => 9,
        "Octubre" => 10,
        "Noviembre" => 11,
        "Diciembre" => 12,
        _ => 13,
    }
}

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
        println!(
            "{:>SPACING$} File './results/out_temp.csv' already exists, skipping '{file}' cleanup",
            "Warning"
        )
    }

    // Obtenemos los valores únicos
    let (file, mut out) = reader_writer("./results/out_temp.csv", "./results/uniques.json")?;
    let (headers, mut uniques) = {
        let mut lines = file.lines();

        let headers = lines.next().unwrap()?.to_owned();
        let headers = headers
            .split(',')
            .map(|a| a.to_owned())
            .collect::<Vec<String>>();

        let u = extract_uniques(headers.as_slice(), lines)?;
        (headers, u)
    };

    // Hacer que ambos conjuntos de meses sean iguales, para despues tener un solo `mes.csv`
    uniques.iter_mut().for_each(|(a, b)| {
        if a.contains("mes") {
            b.insert("NA".to_string());
        }
    });

    // Es necesario iterar siempre en el mismo orden, asi que conjelamos en conjunto en un vector
    let mut uniques: HashMap<_, Vec<_>> = uniques
        .into_iter()
        .map(|(k, v)| (k, v.into_iter().collect()))
        .collect();

    // Ordenamos todos los vectores de valores alfabéticamente
    uniques.iter_mut().for_each(|(k, v)| {
        if k.contains("mes") {
            // Ordenamos manualmente los meses, esta chido tener 1 es enero, 12 diciembre
            v.sort_by(|a, b| mes_a_int(a).cmp(&mes_a_int(b)))
        } else {
            v.sort()
        }
    });

    write!(out, "{:#?}", uniques)?;
    println!(
        "{:>SPACING$} Wrote unique values to './results/uniques.json'",
        "Status"
    );

    // Reemplazamos todos los valores repetidos por el id
    let (file, mut out) = reader_writer("./results/out_temp.csv", "./results/out.csv")?;

    out.write_all("id,".as_bytes())?;
    out.write_all(
        headers
            .iter()
            .map(|h| {
                if uniques[h].is_empty().not() {
                    format!("id_{h}")
                } else {
                    h.to_owned()
                }
            })
            .collect::<Vec<_>>()
            .join(",")
            .as_bytes(),
    )?;
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

    println!(
        "{:>SPACING$} Wrote csv with replaced foreing keys to './results/out.csv'",
        "Status"
    );

    for header in headers.iter() {
        if uniques[header.as_str()].is_empty().not() {
            let header_f = if header.contains("mes") {
                "mes"
            } else {
                header
            };

            let path = format!("./results/{header_f}.csv");

            let mut file = OpenOptions::new()
                .truncate(true)
                .write(true)
                .create(true)
                .open(path.as_str())?;

            file.write_all(format!("id_{header_f},{header_f}\n").as_bytes())?;
            for (i, value) in uniques[header].iter().enumerate() {
                file.write_all(format!("{i},{value}\n").as_bytes())?
            }
            println!(
                "{:>SPACING$} Wrote to '{path}' header values {header_f} with fields (id_{header_f}, {header_f})",
                "Status"
            );
        }
    }

    Ok(())
}
