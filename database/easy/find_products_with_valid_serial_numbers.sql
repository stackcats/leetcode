SELECT *
FROM products
WHERE description ~ '(SN\d{4}-\d{4}\D+|SN\d{4}-\d{4}$)'
ORDER BY product_id;

