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
