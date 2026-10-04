SELECT customer_id,
       COUNT(*)                                                                       total_orders,
       ROUND(100.0 * COUNT(*) FILTER (WHERE order_timestamp::time BETWEEN '11:00:00' AND '14:00:00'
           OR order_timestamp::time BETWEEN '18:00:00' AND '21:00:00') / COUNT(*), 0) peak_hour_percentage,
       ROUND(AVG(order_rating), 2)                                                    average_rating
FROM restaurant_orders
GROUP BY customer_id
HAVING AVG(order_rating) >= 4.0
   AND 1.0 * COUNT(*) FILTER (WHERE order_rating NOTNULL) / COUNT(*) >= 0.5
   AND COUNT(*) >= 3
   AND 100.0 * COUNT(*) FILTER (WHERE order_timestamp::time BETWEEN '11:00:00' AND '14:00:00'
    OR order_timestamp::time BETWEEN '18:00:00' AND '21:00:00') / COUNT(*) >= 60
ORDER BY average_rating DESC, customer_id DESC
