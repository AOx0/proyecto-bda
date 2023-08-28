SELECT 
    MIN(latitud), 
    MAX(latitud), 
    mean 
FROM delitos 
    CROSS JOIN (
        SELECT 
            AVG(latitud) as mean, 
            STDDEV_POP(latitud) as std 
        FROM delitos 
    ) AS s
WHERE latitud 
    BETWEEN (s.mean - 3 * s.std) AND (s.mean + 3 * s.std);


SELECT 
    alcaldia_hecho, 
    Total 
FROM (
    SELECT 
        id_alcaldia_hecho, 
        COUNT(*) as Total 
    FROM delitos 
       WHERE YEAR(fecha_hecho) = 2023 
         AND id_categoria IN (1) 
    GROUP BY id_alcaldia_hecho
) AS res 
JOIN alcaldia_hecho 
ON (alcaldia_hecho.id_alcaldia_hecho = res.id_alcaldia_hecho) 
ORDER BY Total DESC; 