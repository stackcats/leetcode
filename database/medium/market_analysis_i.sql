# Write your MySQL query statement below
with cte as (
    select buyer_id, count(buyer_id) as orders_in_2019
    from Orders 
    where order_date >= '2019-01-01' and order_date < '2020-01-01'
    group by buyer_id
)
select u.user_id as buyer_id, u.join_date, coalesce(cte.orders_in_2019, 0) as orders_in_2019
from Users u
left join cte on u.user_id = cte.buyer_id
