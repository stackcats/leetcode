# Write your MySQL query statement below
select customer_id, sum(case when t.transaction_id is null then 1 else 0 end) as count_no_trans
from Visits v
left join Transactions t on v.visit_id = t.visit_id
group by customer_id
having sum(case when t.transaction_id is null then 1 else 0 end) > 0
