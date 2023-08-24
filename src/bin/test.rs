use anyhow::{anyhow, Result};
use proyecto_bd::*;

fn main() -> Result<()> {
    #[cfg(feature = "dhat")]
    let _profiler = dhat::Profiler::new_heap();

    let file = std::env::args()
        .nth(1)
        .ok_or(anyhow!("Missing file path `cargo run <PATH>`"))?;

    // Primero borramos todas las lineas vacías (incluyendo \r)
    if !std::path::PathBuf::from("./out_temp.csv").exists() {
        let (file, out) = reader_writer(&file, "./out_temp.csv")?;
        remove_empty_lines(file, out)?;
    }

    // Obtenemos los valores únicos
    let (file, mut out) = reader_writer("./out_temp.csv", "out.csv")?;
    let uniques = {
        let mut lines = file.lines();

        let headers = lines.next().unwrap()?.to_owned();
        let headers = headers
            .split(',')
            .map(|a| a.to_owned())
            .collect::<Vec<String>>();

        extract_uniques(headers, lines)?
    };

    write!(out, "{:#?}", uniques)?;

    Ok(())
}
