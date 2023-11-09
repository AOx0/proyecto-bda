-- Your SQL goes here
DROP VIEW IF EXISTS anio_mes_numero;
CREATE VIEW anio_mes_numero 
AS SELECT 
    id_anio_hecho + 1947, 
    id_mes_hecho, 
    COUNT(1) FROM delitos 
WHERE id_anio_hecho + 1947 BETWEEN 2016 AND 2023 
GROUP BY id_anio_hecho, id_mes_hecho;

DROP VIEW IF EXISTS anio_mes_numero_municipio;
CREATE VIEW anio_mes_numero_municipio
AS SELECT 
    id_anio_hecho + 1947, 
    id_mes_hecho, 
    id_alcaldia_hecho, 
    COUNT(1) FROM delitos 
WHERE id_anio_hecho + 1947 BETWEEN 2016 AND 2023 
GROUP BY id_anio_hecho, id_mes_hecho, id_alcaldia_hecho;
