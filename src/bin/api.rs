use askama::Template;
use axum::extract::{Path, State};
use axum::http::{header, HeaderName};
use axum::response::{AppendHeaders, IntoResponse};
use axum::routing::{get, get_service, post};
use axum::{Json, Router, Server};
use chrono::prelude::*;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};
use std::sync::Arc;
use tower_http::services::ServeDir;

// TODO: Esto es un mega haack exagerado. Deberia de ser el id directamente el año
const OFFSET: u16 = 1947;
const ACTUAL_CATEGORIES: usize = 16;

const SVG_HEADER: AppendHeaders<[(HeaderName, &str); 1]> =
    AppendHeaders([(header::CONTENT_TYPE, "image/svg+xml")]);
#[derive(Serialize)]
struct Test {
    nombre: String,
    resultado: String,
}

struct Content {
    name: &'static str,
    content: &'static str,
    desc: &'static str,
    method: &'static str,
}

struct Section {
    name: &'static str,
    href: &'static str,
}

struct Check {
    mensaje: &'static str,
    value: u8,
}

#[derive(Template)]
#[template(path = "hello.html")]
struct Hello<'a> {
    posts: &'a [Content],
    sects: &'a [Section],
    checks: &'a [Check],
}

async fn root() -> Hello<'static> {
    Hello {
        checks: &[
            Check {
                value: 1,
                mensaje: "Delito de bajo impacto",
            },
            Check {
                value: 3,
                mensaje: "Hecho no delictivo",
            },
            Check {
                value: 4,
                mensaje: "Homicidio doloso",
            },
            Check {
                value: 5,
                mensaje: "Lesiones dolosas por disparo de arma de fuego",
            },
            Check {
                value: 7,
                mensaje: "Robo a casa habitación con violencia",
            },
            Check {
                value: 8,
                mensaje: "Robo a cuentahabiente saliendo del cajero con violencia",
            },
            Check {
                value: 9,
                mensaje: "Robo a negocio con violencia",
            },
            Check {
                value: 10,
                mensaje: "Robo a pasajero a bordo de microbus con y sin violencia",
            },
            Check {
                value: 11,
                mensaje: "Robo a pasajero a bordo de taxi con violencia",
            },
            Check {
                value: 12,
                mensaje: "Robo a pasajero a bordo del metro con y sin violencia",
            },
            Check {
                value: 13,
                mensaje: "Robo a repartidor con y sin violencia",
            },
            Check {
                value: 14,
                mensaje: "Robo a transeunte en vía pública con y sin violencia",
            },
            Check {
                value: 15,
                mensaje: "Robo a transportista con y sin violencia",
            },
            Check {
                value: 16,
                mensaje: "Robo de vehículo con y sin violencia",
            },
            Check {
                value: 17,
                mensaje: "Secuestro",
            },
            Check {
                value: 18,
                mensaje: "Violación",
            },
        ],
        posts: &[
            Content {
                name: "Ultima actualización",
                content: " ",
                desc: "El registro más actual",
                method: "/ultimo_dato",
            },
            Content {
                name: "Robos",
                content: " ",
                desc: "En este año",
                method: "/numero_robos/2023",
            },
            Content {
                name: "Homicidios",
                content: " ",
                desc: "En este año",
                method: "/numero_homicidios/2023",
            },
            Content {
                name: "Carpetas",
                content: " ",
                desc: "En este año",
                method: "/date/upnow",
            },
        ],
        sects: &[
            Section {
                name: "Zonas calientes",
                href: "#zonas-calientes",
            },
            Section {
                name: "Top 15 delitos por tiempo",
                href: "#incidentes-por-anio",
            },
            Section {
                name: "Delitos de bajo y alto impacto",
                href: "#razones-por-anio",
            },
            Section {
                name: "Top 15 colonias por tiempo",
                href: "#incidentes-por-colonia",
            },
            Section {
                name: "Mapas de calor por Mes y Año",
                href: "#incidentes-por-mes",
            },
            Section {
                name: "Mapas de calor por Dia y Hora",
                href: "#incidentes-por-dia",
            },
        ],
    }
}

async fn mapa(Path(n): Path<usize>) -> impl IntoResponse {
    (
        SVG_HEADER,
        format!(
            include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/MXfmt.svg")),
            n
        ),
    )
}

async fn date(State(state): State<Shared>, Path(date): Path<String>) -> String {
    let invalid = date.chars().any(|a| !(a.is_ascii_digit() || a == '-'));

    if invalid {
        return format!("Invalid date '{date}'");
    }

    let row: Result<(String,), _> =
        sqlx::query_as("SELECT FORMAT((SELECT COUNT(1) FROM delitos WHERE fecha_hecho = ?), 0)")
            .bind(&date)
            .fetch_one(&state.db)
            .await;

    match row {
        Ok((row,)) => format!("+{}", row),
        Err(_) => "INTERR".to_string(),
    }
}

#[derive(Serialize, Debug, Default)]
struct MapaPorcetajes {
    total: u64,
    valores: Vec<u64>,
}

#[derive(Serialize, Debug, Default)]
struct CantidadesPorMes {
    total: u64,
    valores: Vec<Vec<u64>>,
    meses: Vec<String>,
}

#[derive(Serialize, Debug, Default)]
struct CantidadesAltoYBajo {
    alto: u64,
    bajo: u64,
}

#[derive(Debug, Deserialize)]
struct SolicitudCantidadesPorMes {
    #[serde(default = "min_year")]
    annio_inicio: u16,
    #[serde(default = "max_year")]
    annio_final: u16,
    #[serde(default)]
    categorias: Vec<u16>,
    #[serde(default)]
    alcaldias: Vec<u16>,
}

#[derive(Debug, Deserialize)]
struct SolicitudTopPorAnio {
    #[serde(default = "min_year")]
    annio_inicio: u16,
    #[serde(default = "max_year")]
    annio_final: u16,
    #[serde(default)]
    categorias: Vec<u16>,
}

#[derive(Serialize, Debug, Default)]
struct TopPorAnio {
    valores: Vec<u64>,
    nombres: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct SolicitudPorcentajePorDia {
    #[serde(default = "min_year")]
    annio_inicio: u16,
    #[serde(default = "max_year")]
    annio_final: u16,
    #[serde(default)]
    categorias: Vec<u16>,
}

#[derive(Debug, Deserialize)]
struct SolicitudPorcentajePorHora {
    #[serde(default = "min_year")]
    annio_inicio: u16,
    #[serde(default = "max_year")]
    annio_final: u16,
    #[serde(default)]
    categorias: Vec<u16>,
    #[serde(default, rename = "pinned")]
    dias: Vec<u16>,
}

#[derive(Debug, Deserialize)]
struct SolicitudPorcentajePorAnio {
    #[serde(default)]
    categorias: Vec<u16>,
}

#[derive(Debug, Deserialize)]
struct SolicitudPorcentajePorMesDeAnio {
    #[serde(default)]
    categorias: Vec<u16>,
    anio: u16,
}

#[derive(Serialize, Debug, Default)]
struct MesPorcetajesEnAnio {
    total: u64,
    anio: u16,
    valores: Vec<u64>,
}

#[derive(Serialize, Debug, Default)]
struct DiaPorcentajes {
    total: u64,
    valores: Vec<u64>,
}

#[derive(Serialize, Debug, Default)]
struct HoraPorcentajes {
    total: u64,
    valores: Vec<u64>,
}

#[derive(Serialize, Debug, Default)]
struct AnioPorcetajes {
    total: u64,
    valores: Vec<u64>,
}

#[derive(Debug, Deserialize)]
struct SolicitudMapaPorcentajes {
    #[serde(default = "min_year")]
    annio_inicio: u16,
    #[serde(default = "max_year")]
    annio_final: u16,
    #[serde(default)]
    categorias: Vec<u16>,
}

fn min_year() -> u16 {
    2016
}
fn max_year() -> u16 {
    2023
}

/// TODO: This seems expensive, benchmark and optimize
///
/// # Panics
///
/// Panics if .
async fn mapa_porcentajes(
    State(state): State<Shared>,
    Json(sol): Json<SolicitudMapaPorcentajes>,
) -> Json<MapaPorcetajes> {
    let SolicitudMapaPorcentajes {
        annio_inicio,
        annio_final,
        mut categorias,
    } = sol;

    categorias.sort();

    let (total,): (i64,) = sqlx::query_as(&format!(
        "SELECT COUNT(1) 
           FROM delitos 
          WHERE {}id_anio_hecho BETWEEN ? AND ?;
        ",
        if categorias.is_empty() || categorias.len() >= ACTUAL_CATEGORIES {
            format!("")
        } else {
            format!(
                "id_categoria IN ({0}) AND ",
                categorias
                    .iter()
                    .map(|id| format!("{id}"))
                    .collect::<Vec<_>>()
                    .join(",")
            )
        }
    ))
    .bind(annio_inicio - OFFSET)
    .bind(annio_final - OFFSET)
    .fetch_one(&state.db)
    .await
    .unwrap();

    let resultados: Vec<(i64,)> = sqlx::query_as(&format!(
        "SELECT COUNT(1) 
           FROM delitos 
          WHERE {}id_anio_hecho BETWEEN ? AND ? 
                AND id_alcaldia_hecho IS NOT NULL 
          GROUP BY id_alcaldia_hecho 
          ORDER BY id_alcaldia_hecho;
        ",
        if categorias.is_empty() || categorias.len() >= ACTUAL_CATEGORIES {
            format!("")
        } else {
            format!(
                "id_categoria IN ({0}) AND ",
                categorias
                    .iter()
                    .map(|id| format!("{id}"))
                    .collect::<Vec<_>>()
                    .join(",")
            )
        }
    ))
    .bind(annio_inicio - OFFSET)
    .bind(annio_final - OFFSET)
    .fetch_all(&state.db)
    .await
    .unwrap();

    MapaPorcetajes {
        total: u64::try_from(total).unwrap(),
        valores: resultados.into_iter().map(|(n,)| n as u64).collect(),
    }
    .into()
}

fn months_between(date1: (u64, u64), date2: (u64, u64)) -> u64 {
    let total_months1 = date1.0 * 12 + date1.1;
    let total_months2 = date2.0 * 12 + date2.1;

    if total_months1 > total_months2 {
        total_months1 - total_months2
    } else {
        total_months2 - total_months1
    }
}

async fn cantidades_por_mes(
    State(state): State<Shared>,
    Json(sol): Json<SolicitudCantidadesPorMes>,
) -> Json<CantidadesPorMes> {
    let SolicitudCantidadesPorMes {
        annio_inicio,
        annio_final,
        mut categorias,
        mut alcaldias,
    } = sol;

    categorias.sort();
    alcaldias.sort();

    let resultados: Vec<(u64, u64, u64, i64)> = sqlx::query_as(&format!(
        "SELECT id_anio_hecho + ?, 
                id_mes_hecho, 
                id_alcaldia_hecho, 
                COUNT(1) 
           FROM delitos 
          WHERE id_anio_hecho BETWEEN ? AND ? 
                AND {}id_alcaldia_hecho IN ({a}) 
          GROUP BY id_anio_hecho, id_mes_hecho, id_alcaldia_hecho;
        ",
        if categorias.is_empty() || categorias.len() >= ACTUAL_CATEGORIES {
            format!("")
        } else {
            format!(
                "id_categoria IN ({0}) AND ",
                categorias
                    .iter()
                    .map(|id| format!("{id}"))
                    .collect::<Vec<_>>()
                    .join(",")
            )
        },
        a = alcaldias
            .iter()
            .map(|id| format!("{id}"))
            .collect::<Vec<_>>()
            .join(",")
    ))
    .bind(OFFSET)
    .bind(annio_inicio - OFFSET)
    .bind(annio_final - OFFSET)
    .fetch_all(&state.db)
    .await
    .unwrap();

    // println!("{:?}", resultados);

    let mut primer_mes = (u64::MAX, u64::MAX);
    let mut ultimo_mes = (u64::MIN, u64::MIN);
    let mut total = 0;

    for (anio, mes, _, t) in resultados.iter().copied() {
        // println!("Comparando: ({anio}, {mes})");
        if anio <= primer_mes.0 && mes <= primer_mes.1 {
            primer_mes = (anio, mes);
        }
        if anio >= ultimo_mes.0 || mes >= ultimo_mes.1 {
            ultimo_mes = (anio, mes);
        }
        total += t;
    }

    // println!("PRIMERO: {:?}", primer_mes);
    // println!("ULTIMO: {:?}", ultimo_mes);

    let n_meses: usize = usize::try_from(months_between(primer_mes, ultimo_mes)).unwrap();

    // println!("Nmeses: {n_meses}");

    let mut valores = vec![vec![0; n_meses + 1]; alcaldias.len()];

    for (anio, mes, alcaldia, total) in resultados {
        let m: usize = usize::try_from(months_between(primer_mes, (anio, mes))).unwrap();
        // println!("{primer_mes:?} ({anio}, {mes}): {m} ({total})");
        // println!("{alcaldias:?}, {alcaldia}");
        valores[alcaldias
            .iter()
            .enumerate()
            .find_map(|(i, a)| (a == &u16::try_from(alcaldia).unwrap()).then_some(i))
            .unwrap()][m] = total as u64;
    }

    let mut meses = vec![String::new(); n_meses + 1];
    let mut i = 0;
    while i < n_meses + 1 {
        meses[i] = format!("{}-{}", primer_mes.0, primer_mes.1);
        primer_mes.1 += 1;
        i += 1;
        if primer_mes.1 == 13 {
            primer_mes.1 = 1;
            primer_mes.0 += 1;
        }
    }

    assert_eq!(valores.first().unwrap().len(), meses.len());

    CantidadesPorMes {
        valores,
        meses,
        total: u64::try_from(total).unwrap(),
    }
    .into()
}

async fn dias_porcentajes(
    State(state): State<Shared>,
    Json(sol): Json<SolicitudPorcentajePorDia>,
) -> Json<DiaPorcentajes> {
    let SolicitudPorcentajePorDia {
        annio_inicio,
        annio_final,
        mut categorias,
    } = sol;

    categorias.sort();

    let (total,): (i64,) = sqlx::query_as(&format!(
        "SELECT COUNT(1) FROM delitos WHERE {}id_anio_hecho BETWEEN ? AND ?;",
        if categorias.is_empty() || categorias.len() >= ACTUAL_CATEGORIES {
            format!("")
        } else {
            format!(
                "id_categoria IN ({0}) AND ",
                categorias
                    .iter()
                    .map(|id| format!("{id}"))
                    .collect::<Vec<_>>()
                    .join(",")
            )
        }
    ))
    .bind(annio_inicio - OFFSET)
    .bind(annio_final - OFFSET)
    .fetch_one(&state.db)
    .await
    .unwrap();

    let resultados: Vec<(i16, i64)> = sqlx::query_as(&format!(
        "SELECT WEEKDAY(fecha_hecho), 
                COUNT(1) FROM delitos
          WHERE {}id_anio_hecho BETWEEN ? AND ? 
                AND WEEKDAY(fecha_hecho) IS NOT NULL 
          GROUP BY WEEKDAY(fecha_hecho) 
          ORDER BY WEEKDAY(fecha_hecho);
        ",
        if categorias.is_empty() || categorias.len() >= ACTUAL_CATEGORIES {
            format!("")
        } else {
            format!(
                "id_categoria IN ({0}) AND ",
                categorias
                    .iter()
                    .map(|id| format!("{id}"))
                    .collect::<Vec<_>>()
                    .join(",")
            )
        }
    ))
    .bind(annio_inicio - OFFSET)
    .bind(annio_final - OFFSET)
    .fetch_all(&state.db)
    .await
    .unwrap();

    let mut res = vec![0; 7];

    for (i, v) in resultados {
        res[i as usize] = v as u64;
    }

    println!("Dias: {res:?}");

    DiaPorcentajes {
        total: u64::try_from(total).unwrap(),
        valores: res,
    }
    .into()
}

async fn horas_porcentajes(
    State(state): State<Shared>,
    Json(sol): Json<SolicitudPorcentajePorHora>,
) -> Json<HoraPorcentajes> {
    let SolicitudPorcentajePorHora {
        annio_inicio,
        annio_final,
        mut categorias,
        mut dias,
    } = dbg!(sol);

    dias.sort();
    categorias.sort();

    let (total,): (i64,) = sqlx::query_as(&format!(
        "SELECT COUNT(1) 
           FROM delitos 
          WHERE {}id_anio_hecho BETWEEN ? AND ? 
                AND WEEKDAY(fecha_hecho) IN ({d});
        ",
        if categorias.is_empty() || categorias.len() >= ACTUAL_CATEGORIES {
            format!("")
        } else {
            format!(
                "id_categoria IN ({0}) AND ",
                categorias
                    .iter()
                    .map(|id| format!("{id}"))
                    .collect::<Vec<_>>()
                    .join(",")
            )
        },
        d = dias
            .iter()
            .map(|id| format!("{}", id - 1))
            .collect::<Vec<_>>()
            .join(",")
    ))
    .bind(annio_inicio - OFFSET)
    .bind(annio_final - OFFSET)
    .fetch_one(&state.db)
    .await
    .unwrap();

    let resultados: Vec<(i16, i64)> = sqlx::query_as(&format!(
        "SELECT HOUR(hora_hecho), 
                COUNT(1)
           FROM delitos 
          WHERE {}id_anio_hecho BETWEEN ? AND ? 
                AND WEEKDAY(fecha_hecho) IS NOT NULL 
                AND WEEKDAY(fecha_hecho) IN ({d}) 
          GROUP BY HOUR(hora_hecho);
        ",
        if categorias.is_empty() || categorias.len() >= ACTUAL_CATEGORIES {
            format!("")
        } else {
            format!(
                "id_categoria IN ({0}) AND ",
                categorias
                    .iter()
                    .map(|id| format!("{id}"))
                    .collect::<Vec<_>>()
                    .join(",")
            )
        },
        d = dias
            .iter()
            .map(|id| format!("{}", id - 1))
            .collect::<Vec<_>>()
            .join(",")
    ))
    .bind(annio_inicio - OFFSET)
    .bind(annio_final - OFFSET)
    .fetch_all(&state.db)
    .await
    .unwrap();

    let mut res = vec![0; 24];

    for (i, v) in resultados {
        res[i as usize] = v as u64;
    }

    println!("Horas: {res:?}");

    HoraPorcentajes {
        total: u64::try_from(total).unwrap(),
        valores: res,
    }
    .into()
}

async fn anio_porcentajes(
    State(state): State<Shared>,
    Json(sol): Json<SolicitudPorcentajePorAnio>,
) -> Json<AnioPorcetajes> {
    let SolicitudPorcentajePorAnio { mut categorias } = sol;

    categorias.sort();

    let (total,): (i64,) = sqlx::query_as(&format!(
        "SELECT COUNT(1) FROM delitos WHERE {}id_anio_hecho BETWEEN ? AND ?;",
        if categorias.is_empty() || categorias.len() >= ACTUAL_CATEGORIES {
            format!("")
        } else {
            format!(
                "id_categoria IN ({0}) AND ",
                categorias
                    .iter()
                    .map(|id| format!("{id}"))
                    .collect::<Vec<_>>()
                    .join(",")
            )
        }
    ))
    .bind(min_year() - OFFSET)
    .bind(max_year() - OFFSET)
    .fetch_one(&state.db)
    .await
    .unwrap();

    let resultados: Vec<(i64,)> = sqlx::query_as(&format!(
        "SELECT COUNT(1) 
           FROM delitos 
          WHERE {}id_anio_hecho BETWEEN ? AND ? 
          GROUP BY id_anio_hecho 
          ORDER BY id_anio_hecho;
        ",
        if categorias.is_empty() || categorias.len() >= ACTUAL_CATEGORIES {
            format!("")
        } else {
            format!(
                "id_categoria IN ({0}) AND ",
                categorias
                    .iter()
                    .map(|id| format!("{id}"))
                    .collect::<Vec<_>>()
                    .join(",")
            )
        }
    ))
    .bind(min_year() - OFFSET)
    .bind(max_year() - OFFSET)
    .fetch_all(&state.db)
    .await
    .unwrap();

    // println!("Anio: {resultados:?}");

    AnioPorcetajes {
        total: u64::try_from(total).unwrap(),
        valores: resultados.into_iter().map(|(n,)| n as u64).collect(),
    }
    .into()
}

async fn mes_porcentajes(
    State(state): State<Shared>,
    Json(sol): Json<SolicitudPorcentajePorMesDeAnio>,
) -> Json<MesPorcetajesEnAnio> {
    let SolicitudPorcentajePorMesDeAnio {
        mut categorias,
        anio,
    } = sol;

    categorias.sort();

    let (total,): (i64,) = sqlx::query_as(&format!(
        "SELECT COUNT(1) FROM delitos WHERE {}id_anio_hecho = ?;",
        if categorias.is_empty() || categorias.len() >= ACTUAL_CATEGORIES {
            format!("")
        } else {
            format!(
                "id_categoria IN ({0}) AND ",
                categorias
                    .iter()
                    .map(|id| format!("{id}"))
                    .collect::<Vec<_>>()
                    .join(",")
            )
        }
    ))
    .bind(anio - OFFSET)
    .fetch_one(&state.db)
    .await
    .unwrap();

    let resultados: Vec<(u16, i64)> = sqlx::query_as(&format!(
        "SELECT id_mes_hecho, 
                COUNT(1) FROM delitos 
          WHERE {}id_anio_hecho = ? 
          GROUP BY id_mes_hecho;
        ",
        if categorias.is_empty() || categorias.len() >= ACTUAL_CATEGORIES {
            format!("")
        } else {
            format!(
                "id_categoria IN ({0}) AND ",
                categorias
                    .iter()
                    .map(|id| format!("{id}"))
                    .collect::<Vec<_>>()
                    .join(",")
            )
        }
    ))
    .bind(anio - OFFSET)
    .fetch_all(&state.db)
    .await
    .unwrap();

    // println!("{resultados:?}");

    let mut res = if anio == 2023 {
        vec![0; 9]
    } else {
        vec![0; 12]
    };

    resultados.iter().copied().for_each(|(m, v)| {
        res[m as usize - 1] = v;
    });

    assert!(res.len() >= 9);

    // println!("Meses en {}: {resultados:?}", anio + OFFSET);
    // println!("Meses en {}: {res:?}", anio + OFFSET);

    MesPorcetajesEnAnio {
        total: u64::try_from(total).unwrap(),
        valores: res.into_iter().map(|n| n as u64).collect(),
        anio,
    }
    .into()
}

fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

async fn delitos_por_anio(
    State(state): State<Shared>,
    Json(sol): Json<SolicitudTopPorAnio>,
) -> Json<TopPorAnio> {
    let SolicitudTopPorAnio {
        annio_inicio,
        annio_final,
        mut categorias,
    } = dbg!(sol);

    categorias.sort();

    let mut resultados: Vec<(String, i64)> = sqlx::query_as(&format!(
        "SELECT categoria, 
                COUNT(*) AS fre 
           FROM delitos 
           JOIN categoria USING(id_categoria) 
          WHERE {}id_anio_hecho BETWEEN ? AND ? 
          GROUP BY id_categoria;
        ",
        if categorias.is_empty() || categorias.len() >= ACTUAL_CATEGORIES {
            format!("")
        } else {
            format!(
                "id_categoria IN ({0}) AND ",
                categorias
                    .iter()
                    .map(|id| format!("{id}"))
                    .collect::<Vec<_>>()
                    .join(",")
            )
        }
    ))
    .bind(annio_inicio - OFFSET)
    .bind(annio_final - OFFSET)
    .fetch_all(&state.db)
    .await
    .unwrap();

    resultados.sort_unstable_by_key(|(_, v)| *v);
    resultados = resultados.into_iter().rev().into_iter().collect::<Vec<_>>();

    TopPorAnio {
        valores: resultados.iter().map(|(_, n)| *n as u64).collect(),
        nombres: resultados
            .into_iter()
            .map(|(n, _)| uppercase_first_letter(n.to_lowercase().as_str()))
            .collect(),
    }
    .into()
}

async fn top_por_anio(
    State(state): State<Shared>,
    Json(sol): Json<SolicitudTopPorAnio>,
) -> Json<TopPorAnio> {
    let SolicitudTopPorAnio {
        annio_inicio,
        annio_final,
        mut categorias,
    } = dbg!(sol);

    categorias.sort();

    let mut resultados: Vec<(String, i64)> = sqlx::query_as(&format!(
        "SELECT delito, fre
           FROM (SELECT id_delito,
                        COUNT(*) AS fre
                   FROM delitos 
                  WHERE {}id_anio_hecho BETWEEN ? AND ? 
                  GROUP BY id_delito
                  ORDER BY fre DESC
                   LIMIT 15) as R
           JOIN delito USING(id_delito);
        ",
        if categorias.is_empty() || categorias.len() >= ACTUAL_CATEGORIES {
            format!("")
        } else {
            format!(
                "id_categoria IN ({0}) AND ",
                categorias
                    .iter()
                    .map(|id| format!("{id}"))
                    .collect::<Vec<_>>()
                    .join(",")
            )
        }
    ))
    .bind(annio_inicio - OFFSET)
    .bind(annio_final - OFFSET)
    .fetch_all(&state.db)
    .await
    .unwrap();

    resultados.sort_unstable_by_key(|(_, v)| *v);
    resultados = resultados
        .into_iter()
        .rev()
        .take(15)
        .into_iter()
        .collect::<Vec<_>>();

    TopPorAnio {
        valores: resultados.iter().map(|(_, n)| *n as u64).collect(),
        nombres: resultados
            .into_iter()
            .map(|(n, _)| uppercase_first_letter(n.to_lowercase().as_str()))
            .collect(),
    }
    .into()
}

async fn top_por_colonia(
    State(state): State<Shared>,
    Json(sol): Json<SolicitudTopPorAnio>,
) -> Json<TopPorAnio> {
    let SolicitudTopPorAnio {
        annio_inicio,
        annio_final,
        mut categorias,
    } = dbg!(sol);

    categorias.sort();

    let mut resultados: Vec<(String, i64)> = sqlx::query_as(&format!(
        "SELECT IFNULL(colonia_hecho, 'Desconocido'), 
                COUNT(*) AS fre 
           FROM delitos 
          WHERE {}id_anio_hecho BETWEEN ? AND ? GROUP BY colonia_hecho
          ORDER BY fre DESC
          LIMIT 15;
        ",
        if categorias.is_empty() || categorias.len() >= ACTUAL_CATEGORIES {
            format!("")
        } else {
            format!(
                "id_categoria IN ({0}) AND ",
                categorias
                    .iter()
                    .map(|id| format!("{id}"))
                    .collect::<Vec<_>>()
                    .join(",")
            )
        }
    ))
    .bind(annio_inicio - OFFSET)
    .bind(annio_final - OFFSET)
    .fetch_all(&state.db)
    .await
    .unwrap();

    resultados.sort_unstable_by_key(|(_, v)| *v);
    resultados = resultados
        .into_iter()
        .rev()
        .take(15)
        .into_iter()
        .collect::<Vec<_>>();

    TopPorAnio {
        valores: resultados.iter().map(|(_, n)| *n as u64).collect(),
        nombres: resultados
            .into_iter()
            .map(|(n, _)| uppercase_first_letter(n.to_lowercase().as_str()))
            .collect(),
    }
    .into()
}

async fn cantidad_alto_y_bajo(
    State(state): State<Shared>,
    Json(sol): Json<SolicitudTopPorAnio>,
) -> Json<TopPorAnio> {
    let SolicitudTopPorAnio {
        annio_inicio,
        annio_final,
        ..
    } = dbg!(sol);

    let mut resultados: Vec<(String, i64)> = sqlx::query_as(&format!(
        "SELECT delito, fre
           FROM (SELECT id_delito,
                        COUNT(*) AS fre 
                   FROM delitos 
                  WHERE id_anio_hecho BETWEEN ? AND ? 
                    AND id_categoria = 1
                  GROUP BY id_delito) as R
           JOIN delito USING(id_delito); 
        ",
    ))
    .bind(annio_inicio - OFFSET)
    .bind(annio_final - OFFSET)
    .fetch_all(&state.db)
    .await
    .unwrap();

    resultados.sort_unstable_by_key(|(_, v)| *v);
    resultados = resultados.into_iter().rev().into_iter().collect::<Vec<_>>();

    let mut top_15 = resultados[0..15].to_vec();
    let resto = &resultados[15..];

    let total_resto = resto.iter().fold(0, |s, (_, v)| s + v);

    top_15.push(("Otros".to_string(), total_resto));

    TopPorAnio {
        valores: top_15.iter().map(|(_, n)| *n as u64).collect(),
        nombres: top_15
            .into_iter()
            .map(|(n, _)| uppercase_first_letter(n.to_lowercase().as_str()))
            .collect(),
    }
    .into()
}

async fn ultimo_dato(State(state): State<Shared>) -> String {
    let (resultado,): (NaiveDate,) =
        sqlx::query_as("SELECT fecha_hecho FROM delitos WHERE id_anio_hecho = ? ORDER BY fecha_hecho DESC LIMIT 1;")
            .bind(max_year() - OFFSET)
            .fetch_one(&state.db)
            .await
            .unwrap();

    resultado.format("%Y-%m-%d").to_string()
}

async fn numero_anio(State(state): State<Shared>, Path(anio): Path<u16>) -> String {
    let (resultado,): (String,) = sqlx::query_as(
        "SELECT FORMAT(COUNT(*), 0) 
           FROM delitos 
           JOIN categoria USING(id_categoria) 
          WHERE id_anio_hecho = ? 
                AND categoria = 'HOMICIDIO DOLOSO' 
          LIMIT 1;
        ",
    )
    .bind(anio - OFFSET)
    .fetch_one(&state.db)
    .await
    .unwrap();

    format!("+{resultado}")
}

async fn numero_robo(State(state): State<Shared>, Path(anio): Path<u16>) -> String {
    let (resultado,): (String,) =
        sqlx::query_as(
            "SELECT FORMAT(COUNT(*), 0) 
               FROM delitos 
               JOIN categoria USING(id_categoria) 
              WHERE id_anio_hecho = ? 
                    AND id_categoria IN (SELECT id_categoria FROM categoria WHERE categoria LIKE '%ROBO%') 
              LIMIT 1;
            ")
            .bind(anio - OFFSET)
            .fetch_one(&state.db)
            .await
            .unwrap();

    format!("+{resultado}")
}

async fn cantidad_alto_y_bajo2(
    State(state): State<Shared>,
    Json(sol): Json<SolicitudTopPorAnio>,
) -> Json<TopPorAnio> {
    let SolicitudTopPorAnio {
        annio_inicio,
        annio_final,
        ..
    } = dbg!(sol);

    let mut resultados: Vec<(String, i64)> = sqlx::query_as(&format!(
        "SELECT delito, fre
           FROM (SELECT id_delito,
                        COUNT(*) AS fre 
                   FROM delitos 
                  WHERE id_anio_hecho BETWEEN ? AND ? 
                    AND id_categoria != 1
                  GROUP BY id_delito) as R
           JOIN delito USING(id_delito); 
        ",
    ))
    .bind(annio_inicio - OFFSET)
    .bind(annio_final - OFFSET)
    .fetch_all(&state.db)
    .await
    .unwrap();

    resultados.sort_unstable_by_key(|(_, v)| *v);
    resultados = resultados.into_iter().rev().into_iter().collect::<Vec<_>>();

    let mut top_15 = resultados[0..15].to_vec();
    let resto = &resultados[15..];

    let total_resto = resto.iter().fold(0, |s, (_, v)| s + v);

    top_15.push(("Otros".to_string(), total_resto));

    TopPorAnio {
        valores: top_15.iter().map(|(_, n)| *n as u64).collect(),
        nombres: top_15
            .into_iter()
            .map(|(n, _)| uppercase_first_letter(n.to_lowercase().as_str()))
            .collect(),
    }
    .into()
}

async fn untilnow(State(state): State<Shared>) -> String {
    let utc: DateTime<Utc> = Utc::now();
    let year = utc.format("%Y").to_string();
    let year: u16 = year.parse().unwrap();

    let row: Result<(String,), _> =
        sqlx::query_as("SELECT FORMAT((SELECT COUNT(1) FROM delitos WHERE id_anio_hecho = ?), 0)")
            .bind(year - OFFSET)
            .fetch_one(&state.db)
            .await;

    match row {
        Ok((row,)) => format!("+{}", row),
        Err(err) => {
            println!("Error: {err}");
            "INTERR".to_string()
        }
    }
}

fn static_files() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./assets")))
}

#[derive(Clone)]
struct Shared(Arc<Inner>);

impl std::ops::Deref for Shared {
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct Inner {
    db: Pool<MySql>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let address = std::env::args().nth(1).unwrap_or("[::]:80".to_string());

    dotenv().ok();

    let db = dotenv::var("DATABASE_URL")?;

    let state = Shared(Arc::new(Inner {
        db: MySqlPoolOptions::new()
            .max_connections(5)
            .connect(&db)
            .await?,
    }));

    let router = Router::new()
        .route("/", get(root))
        .route("/mapa/:n", get(mapa))
        .route("/health", get(|| async { "alive" }))
        .route("/date/:date", get(date))
        .route("/map_percent", post(mapa_porcentajes))
        .route("/mes_percent", post(mes_porcentajes))
        .route("/anio_percent", post(anio_porcentajes))
        .route("/dias_percent", post(dias_porcentajes))
        .route("/horas_percent", post(horas_porcentajes))
        .route("/top_por_anio", post(top_por_anio))
        .route("/top_por_colonia", post(top_por_colonia))
        .route("/delitos_por_anio", post(delitos_por_anio))
        .route("/alto_y_bajo", post(cantidad_alto_y_bajo))
        .route("/alto_y_bajo2", post(cantidad_alto_y_bajo2))
        .route("/ultimo_dato", get(ultimo_dato))
        .route("/numero_homicidios/:anio", get(numero_anio))
        .route("/numero_robos/:anio", get(numero_robo))
        .route("/c_por_mes", post(cantidades_por_mes))
        .route("/date/upnow", get(untilnow))
        .with_state(state)
        .fallback_service(static_files());

    Server::bind(&address.parse().unwrap())
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
