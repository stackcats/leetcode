-- Write your PostgreSQL query statement below
SELECT visited_on,
       SUM(amount) OVER (ROWS BETWEEN 6 PRECEDING AND CURRENT ROW)           amount,
       ROUND(AVG(amount) OVER (ROWS BETWEEN 6 PRECEDING AND CURRENT ROW), 2) average_amount
FROM (
         SELECT visited_on,
                SUM(amount) amount
         FROM customer
         GROUP BY visited_on
         ORDER BY visited_on)
OFFSET 6

