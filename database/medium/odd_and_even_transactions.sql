-- Write your PostgreSQL query statement below
select
transaction_date,
COALESCE(sum(amount) filter (where mod(amount, 2) = 1), 0) odd_sum,
COALESCE(sum(amount) filter (where mod(amount, 2) = 0), 0) even_sum
from transactions
group by transaction_date
order by transaction_date
