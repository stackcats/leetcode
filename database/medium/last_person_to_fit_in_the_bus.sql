-- Write your PostgreSQL query statement below
SELECT person_name
FROM (
         SELECT person_name, turn, SUM(weight) OVER (ORDER BY turn) acc_weight
         FROM queue
         ORDER BY turn)
WHERE acc_weight <= 1000
ORDER BY turn DESC
LIMIT 1

