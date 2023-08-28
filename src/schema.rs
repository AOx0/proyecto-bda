// @generated automatically by Diesel CLI.

diesel::table! {
    agencia (id_agencia) {
        id_agencia -> Unsigned<Integer>,
        #[max_length = 70]
        agencia -> Char,
    }
}

diesel::table! {
    alcaldia_hecho (id_alcaldia_hecho) {
        id_alcaldia_hecho -> Unsigned<Integer>,
        #[max_length = 70]
        alcaldia_hecho -> Char,
    }
}

diesel::table! {
    anio_hecho (id_anio_hecho) {
        id_anio_hecho -> Unsigned<Integer>,
        #[max_length = 70]
        anio_hecho -> Char,
    }
}

diesel::table! {
    anio_inicio (id_anio_inicio) {
        id_anio_inicio -> Unsigned<Integer>,
        #[max_length = 70]
        anio_inicio -> Char,
    }
}

diesel::table! {
    categoria (id_categoria) {
        id_categoria -> Unsigned<Integer>,
        #[max_length = 70]
        categoria -> Char,
    }
}

diesel::table! {
    competencia (id_competencia) {
        id_competencia -> Unsigned<Integer>,
        #[max_length = 70]
        competencia -> Char,
    }
}

diesel::table! {
    delito (id_delito) {
        id_delito -> Unsigned<Integer>,
        #[max_length = 200]
        delito -> Char,
    }
}

diesel::table! {
    delitos (id, fecha_hecho, hora_hecho) {
        id -> Unsigned<Integer>,
        id_anio_hecho -> Nullable<Unsigned<Integer>>,
        id_mes_hecho -> Nullable<Unsigned<Integer>>,
        fecha_hecho -> Date,
        hora_hecho -> Time,
        id_delito -> Unsigned<Integer>,
        id_categoria -> Unsigned<Integer>,
        id_competencia -> Nullable<Unsigned<Integer>>,
        id_fiscalia -> Nullable<Unsigned<Integer>>,
        id_agencia -> Unsigned<Integer>,
        id_unidad_investigacion -> Nullable<Unsigned<Integer>>,
        id_anio_inicio -> Unsigned<Integer>,
        id_mes_inicio -> Unsigned<Integer>,
        fecha_inicio -> Nullable<Date>,
        hora_inicio -> Nullable<Time>,
        #[max_length = 100]
        colonia_catalogo -> Nullable<Char>,
        #[max_length = 100]
        colonia_hecho -> Nullable<Char>,
        id_alcaldia_hecho -> Nullable<Unsigned<Integer>>,
        id_municipio_hecho -> Nullable<Unsigned<Integer>>,
        latitud -> Nullable<Float>,
        longitud -> Nullable<Float>,
    }
}

diesel::table! {
    fiscalia (id_fiscalia) {
        id_fiscalia -> Unsigned<Integer>,
        #[max_length = 200]
        fiscalia -> Char,
    }
}

diesel::table! {
    mes (id_mes) {
        id_mes -> Unsigned<Integer>,
        #[max_length = 70]
        mes -> Char,
    }
}

diesel::table! {
    municipio_hecho (id_municipio_hecho) {
        id_municipio_hecho -> Unsigned<Integer>,
        #[max_length = 70]
        municipio_hecho -> Char,
    }
}

diesel::table! {
    unidad_investigacion (id_unidad_investigacion) {
        id_unidad_investigacion -> Unsigned<Integer>,
        #[max_length = 70]
        unidad_investigacion -> Char,
    }
}

diesel::joinable!(delitos -> agencia (id_agencia));
diesel::joinable!(delitos -> alcaldia_hecho (id_alcaldia_hecho));
diesel::joinable!(delitos -> anio_hecho (id_anio_hecho));
diesel::joinable!(delitos -> anio_inicio (id_anio_inicio));
diesel::joinable!(delitos -> categoria (id_categoria));
diesel::joinable!(delitos -> competencia (id_competencia));
diesel::joinable!(delitos -> delito (id_delito));
diesel::joinable!(delitos -> fiscalia (id_fiscalia));
diesel::joinable!(delitos -> municipio_hecho (id_municipio_hecho));
diesel::joinable!(delitos -> unidad_investigacion (id_unidad_investigacion));

diesel::allow_tables_to_appear_in_same_query!(
    agencia,
    alcaldia_hecho,
    anio_hecho,
    anio_inicio,
    categoria,
    competencia,
    delito,
    delitos,
    fiscalia,
    mes,
    municipio_hecho,
    unidad_investigacion,
);
