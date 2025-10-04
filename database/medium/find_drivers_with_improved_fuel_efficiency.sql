-- Write your PostgreSQL query statement below
WITH cte AS (
    SELECT
        driver_id,
        AVG(distance_km / fuel_consumed) FILTER (WHERE EXTRACT(MONTH FROM trip_date) BETWEEN 1 AND 6)  first_half_avg,
        AVG(distance_km / fuel_consumed) FILTER (WHERE EXTRACT(MONTH FROM trip_date) BETWEEN 7 AND 12) second_half_avg
    FROM trips
    GROUP BY driver_id)
SELECT
    driver_id,
    driver_name,
    ROUND(first_half_avg, 2)                   first_half_avg,
    ROUND(second_half_avg, 2)                  second_half_avg,
    ROUND(second_half_avg - first_half_avg, 2) efficiency_improvement
FROM cte
         LEFT JOIN drivers USING (driver_id)
WHERE first_half_avg > 0
  AND second_half_avg > 0
  AND second_half_avg > first_half_avg
ORDER BY efficiency_improvement DESC, driver_name
