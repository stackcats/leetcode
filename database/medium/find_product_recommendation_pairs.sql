-- Write your PostgreSQL query statement below
WITH cte AS (
    SELECT pp.user_id, pp.product_id, pi.category
    FROM ProductPurchases pp
             LEFT JOIN ProductInfo pi ON pp.product_id = pi.product_id)
SELECT c1.product_id              product1_id,
       c2.product_id              product2_id,
       c1.category                product1_category,
       c2.category                product2_category,
       COUNT(DISTINCT c1.user_id) customer_count
FROM cte c1
         LEFT JOIN cte c2 ON c1.user_id = c2.user_id
WHERE c1.product_id < c2.product_id
GROUP BY c1.product_id, c2.product_id, c1.category, c2.category
HAVING COUNT(DISTINCT c1.user_id) > 2
ORDER BY customer_count DESC, c1.product_id, c2.product_id
