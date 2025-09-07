-- Write your PostgreSQL query statement below
WITH cte AS (
    SELECT requester_id id
    FROM RequestAccepted
    UNION ALL
    SELECT accepter_id id
    FROM RequestAccepted)
SELECT id, COUNT(1) num
FROM cte
GROUP BY id
ORDER BY num DESC
LIMIT 1
