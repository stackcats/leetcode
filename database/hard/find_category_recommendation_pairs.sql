-- Write your PostgreSQL query statement below
WITH cte AS (
    SELECT pp.user_id, pp.product_id, pi.category
    FROM ProductPurchases pp
             LEFT JOIN ProductInfo pi ON pp.product_id = pi.product_id)
SELECT c1.category                category1,
       c2.category                category2,
       COUNT(DISTINCT c1.user_id) customer_count
FROM cte c1
         LEFT JOIN cte c2 ON c1.user_id = c2.user_id
WHERE c1.category < c2.category
GROUP BY c1.category, c2.category
HAVING COUNT(DISTINCT c1.user_id) > 2
ORDER BY customer_count DESC, c1.category, c2.category
