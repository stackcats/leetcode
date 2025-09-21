-- Write your PostgreSQL query statement below
with cte as (select *,
case when extract(month from sale_date) BETWEEN 3 AND 5 THEN 'Spring'
        WHEN EXTRACT(MONTH FROM sale_date) BETWEEN 6 AND 8 THEN 'Summer'
        WHEN EXTRACT(MONTH FROM sale_date) BETWEEN 9 AND 11 THEN 'Fall' 
        ELSE 'Winter'
end season
from sales),
cte2 as (select season,
category,
sum(quantity) total_quantity,
sum(quantity * price) total_revenue,
rank() over (partition by season order by sum(quantity) desc, sum(quantity * price) desc) r
from cte
left join products p on cte.product_id = p.product_id
group by season, category
order by season)
select season, category, total_quantity, total_revenue  from cte2
where r = 1
