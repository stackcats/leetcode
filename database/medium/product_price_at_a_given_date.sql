-- Write your PostgreSQL query statement below
WITH cte AS (
    SELECT product_id, MAX(change_date) change_date
    FROM products
    WHERE change_date <= '2019-08-16'
    GROUP BY product_id)
SELECT p.product_id, new_price price
FROM products p
         RIGHT JOIN cte ON p.product_id = cte.product_id AND p.change_date = cte.change_date
UNION
SELECT product_id, 10
FROM products
GROUP BY product_id
HAVING MIN(change_date) > '2019-08-16'

