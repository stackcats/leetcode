-- Write your PostgreSQL query statement below
WITH cte AS (
    SELECT DISTINCT
        store_id,
        store_name,
        location,
        FIRST_VALUE(product_name) OVER (PARTITION BY store_id ORDER BY price DESC) most_exp_product,
        FIRST_VALUE(product_name) OVER (PARTITION BY store_id ORDER BY price)      cheapest_product,
        FIRST_VALUE(quantity) OVER (PARTITION BY store_id ORDER BY price)          cheapest_quantity,
        FIRST_VALUE(quantity) OVER (PARTITION BY store_id ORDER BY price DESC)     most_expensive_quantity
    FROM inventory
             LEFT JOIN stores USING (store_id)
    WHERE store_id IN (
        SELECT
            store_id
        FROM inventory
        GROUP BY store_id
        HAVING COUNT(1) >= 3))
SELECT
    store_id,
    store_name,
    location,
    most_exp_product,
    cheapest_product,
    ROUND(cheapest_quantity * 1.0 / most_expensive_quantity, 2) imbalance_ratio
FROM cte
WHERE most_expensive_quantity < cheapest_quantity
ORDER BY imbalance_ratio DESC, store_name
