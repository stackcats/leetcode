-- Write your PostgreSQL query statement below
WITH cte AS (
    SELECT employee_id, ARRAY_AGG(rating ORDER BY review_date DESC) AS ratings
    FROM performance_reviews
    GROUP BY employee_id
    HAVING COUNT(*) >= 3)
SELECT employee_id,
       name,
       ratings[1] - ratings[3] improvement_score
FROM employees
         JOIN cte USING (employee_id)
WHERE ratings[1] > ratings[2]
  AND ratings[2] > ratings[3]
ORDER BY improvement_score DESC, name
