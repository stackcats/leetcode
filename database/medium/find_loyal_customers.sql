-- Write your PostgreSQL query statement below
WITH cte AS (SELECT customer_id,
                    COUNT(1) FILTER (WHERE transaction_type = 'refund') refund,
                    COUNT(1)                                            total,
                    MAX(transaction_date) - MIN(transaction_date)       ds
             FROM customer_transactions
             GROUP BY customer_id)
SELECT customer_id
FROM cte
WHERE total >= 3
  AND ds >= 30
  AND refund * 1.0 / total < 0.2
ORDER BY customer_id;
