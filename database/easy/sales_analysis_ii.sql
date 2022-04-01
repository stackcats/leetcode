# Write your MySQL query statement below
select product_id, product_name
from Product
where product_id in (select product_id from Sales where sale_date between 20190101000000 and 20190401000000)
and product_id not in (select product_id from Sales where sale_date < 20190101000000 or sale_date >= 20190401000000)
