-- Your SQL goes here

-- Creat tabla agencia
CREATE TABLE agencia (
    id_agencia INT UNSIGNED NOT NULL AUTO_INCREMENT,
    agencia CHAR(70) NOT NULL,
    CONSTRAINT pk_id_agencia PRIMARY KEY(id_agencia)
);

-- Crear tabla alcaldia_hecho
CREATE TABLE alcaldia_hecho (
    id_alcaldia_hecho INT UNSIGNED NOT NULL AUTO_INCREMENT,
    alcaldia_hecho CHAR(70) NOT NULL,
    CONSTRAINT pk_id_alcaldia_hecho PRIMARY KEY(id_alcaldia_hecho)
);

-- Crear tabla anio_hecho
CREATE TABLE anio_hecho (
    id_anio_hecho INT UNSIGNED NOT NULL AUTO_INCREMENT,
    anio_hecho CHAR(70) NOT NULL,
    CONSTRAINT pk_id_anio_hecho PRIMARY KEY(id_anio_hecho)
);

-- Crear tabla anio_inicio
CREATE TABLE anio_inicio (
    id_anio_inicio INT UNSIGNED NOT NULL AUTO_INCREMENT,
    anio_inicio CHAR(70) NOT NULL,
    CONSTRAINT pk_id_anio_inicio PRIMARY KEY(id_anio_inicio)
);

-- Crear tabla categoria
CREATE TABLE categoria (
    id_categoria INT UNSIGNED NOT NULL AUTO_INCREMENT,
    categoria CHAR(70) NOT NULL,
    CONSTRAINT pk_id_categoria PRIMARY KEY(id_categoria)
);

-- Crear tabla competencia
CREATE TABLE competencia (
    id_competencia INT UNSIGNED NOT NULL AUTO_INCREMENT,
    competencia CHAR(70) NOT NULL,
    CONSTRAINT pk_id_competencia PRIMARY KEY(id_competencia)
);

-- Crear tabla delito
CREATE TABLE delito (
    id_delito INT UNSIGNED NOT NULL AUTO_INCREMENT,
    delito CHAR(200) NOT NULL,
    CONSTRAINT pk_id_delito PRIMARY KEY(id_delito)
);

-- Crear tabla fiscalia
CREATE TABLE fiscalia (
    id_fiscalia INT UNSIGNED NOT NULL AUTO_INCREMENT,
    fiscalia CHAR(200) NOT NULL,
    CONSTRAINT pk_id_fiscalia PRIMARY KEY(id_fiscalia)
);

-- Crear tabla mes
CREATE TABLE mes (
    id_mes INT UNSIGNED NOT NULL AUTO_INCREMENT,
    mes CHAR(70) NOT NULL,
    CONSTRAINT pk_id_mes PRIMARY KEY(id_mes)
);

-- Crear tabla municipio_hecho
CREATE TABLE municipio_hecho (
    id_municipio_hecho INT UNSIGNED NOT NULL AUTO_INCREMENT,
    municipio_hecho CHAR(70) NOT NULL,
    CONSTRAINT pk_id_municipio_hecho PRIMARY KEY(id_municipio_hecho)
);


-- Crear tabla unidad_investigacion
CREATE TABLE unidad_investigacion (
    id_unidad_investigacion INT UNSIGNED NOT NULL AUTO_INCREMENT,
    unidad_investigacion CHAR(70) NOT NULL,
    CONSTRAINT pk_id_unidad_investigacion PRIMARY KEY(id_unidad_investigacion)
);

-- Crear tabla delitos
CREATE TABLE delitos (
    id INT UNSIGNED NOT NULL AUTO_INCREMENT,
    id_anio_hecho INT UNSIGNED DEFAULT NULL,
    id_mes_hecho INT UNSIGNED DEFAULT NULL,
    fecha_hecho DATE,
    hora_hecho TIME,
    id_delito INT UNSIGNED NOT NULL,
    id_categoria INT UNSIGNED NOT NULL,
    id_competencia INT UNSIGNED DEFAULT NULL,
    id_fiscalia INT UNSIGNED DEFAULT NULL,
    id_agencia INT UNSIGNED NOT NULL,
    id_unidad_investigacion INT UNSIGNED DEFAULT NULL,
    id_anio_inicio INT UNSIGNED NOT NULL,
    id_mes_inicio INT UNSIGNED NOT NULL,
    fecha_inicio DATE,
    hora_inicio TIME,
    colonia_catalogo CHAR(100),
    colonia_hecho CHAR(100),
    id_alcaldia_hecho INT UNSIGNED DEFAULT NULL,
    id_municipio_hecho INT UNSIGNED DEFAULT NULL,
    latitud FLOAT,
    longitud FLOAT,
    CONSTRAINT pk_id PRIMARY KEY (id)
);

ALTER TABLE delitos ADD CONSTRAINT fk_id_delito FOREIGN KEY (id_delito) REFERENCES delito (id_delito);
ALTER TABLE delitos ADD CONSTRAINT fk_id_mes_inicio FOREIGN KEY (id_mes_inicio) REFERENCES mes (id_mes);
ALTER TABLE delitos ADD CONSTRAINT fk_id_agencia FOREIGN KEY (id_agencia) REFERENCES agencia (id_agencia);
ALTER TABLE delitos ADD CONSTRAINT fk_id_categoria FOREIGN KEY (id_categoria) REFERENCES categoria (id_categoria);
ALTER TABLE delitos ADD CONSTRAINT fk_id_anio_inicio FOREIGN KEY (id_anio_inicio) REFERENCES anio_inicio (id_anio_inicio);
ALTER TABLE delitos ADD CONSTRAINT fk_id_mes_hecho FOREIGN KEY (id_mes_hecho) REFERENCES mes (id_mes);
ALTER TABLE delitos ADD CONSTRAINT fk_id_fiscalia FOREIGN KEY (id_fiscalia) REFERENCES fiscalia (id_fiscalia);
ALTER TABLE delitos ADD CONSTRAINT fk_id_anio_hecho FOREIGN KEY (id_anio_hecho) REFERENCES anio_hecho (id_anio_hecho);
ALTER TABLE delitos ADD CONSTRAINT fk_id_competencia FOREIGN KEY (id_competencia) REFERENCES competencia (id_competencia);
ALTER TABLE delitos ADD CONSTRAINT fk_id_alcaldia_hecho FOREIGN KEY (id_alcaldia_hecho) REFERENCES alcaldia_hecho (id_alcaldia_hecho);
ALTER TABLE delitos ADD CONSTRAINT fk_id_municipio_hecho FOREIGN KEY (id_municipio_hecho) REFERENCES municipio_hecho (id_municipio_hecho);
ALTER TABLE delitos ADD CONSTRAINT fk_id_unidad_investigacion FOREIGN KEY (id_unidad_investigacion) REFERENCES unidad_investigacion (id_unidad_investigacion);


-- Insert with:
/*
LOAD DATA LOCAL INFILE './results/agencia.csv' INTO TABLE agencia FIELDS TERMINATED BY ',' ENCLOSED BY '"' IGNORE 1 LINES;
LOAD DATA LOCAL INFILE './results/alcaldia_hecho.csv' INTO TABLE alcaldia_hecho FIELDS TERMINATED BY ',' ENCLOSED BY '"' IGNORE 1 LINES;
LOAD DATA LOCAL INFILE './results/anio_hecho.csv' INTO TABLE anio_hecho FIELDS TERMINATED BY ',' ENCLOSED BY '"' IGNORE 1 LINES;
LOAD DATA LOCAL INFILE './results/anio_inicio.csv' INTO TABLE anio_inicio FIELDS TERMINATED BY ',' ENCLOSED BY '"' IGNORE 1 LINES;
LOAD DATA LOCAL INFILE './results/categoria.csv' INTO TABLE categoria FIELDS TERMINATED BY ',' ENCLOSED BY '"' IGNORE 1 LINES;
LOAD DATA LOCAL INFILE './results/competencia.csv' INTO TABLE competencia FIELDS TERMINATED BY ',' ENCLOSED BY '"' IGNORE 1 LINES;
LOAD DATA LOCAL INFILE './results/delito.csv' INTO TABLE delito FIELDS TERMINATED BY ',' ENCLOSED BY '"' IGNORE 1 LINES;
LOAD DATA LOCAL INFILE './results/fiscalia.csv' INTO TABLE fiscalia FIELDS TERMINATED BY ',' ENCLOSED BY '"' IGNORE 1 LINES;
LOAD DATA LOCAL INFILE './results/mes.csv' INTO TABLE mes FIELDS TERMINATED BY ',' ENCLOSED BY '"' IGNORE 1 LINES;
LOAD DATA LOCAL INFILE './results/municipio_hecho.csv' INTO TABLE municipio_hecho FIELDS TERMINATED BY ',' ENCLOSED BY '"' IGNORE 1 LINES;
LOAD DATA LOCAL INFILE './results/unidad_investigacion.csv' INTO TABLE unidad_investigacion FIELDS TERMINATED BY ',' ENCLOSED BY '"' IGNORE 1 LINES;
LOAD DATA LOCAL INFILE './results/out.csv' INTO TABLE delitos FIELDS TERMINATED BY ',' ENCLOSED BY '"' IGNORE 1 LINES;
*/