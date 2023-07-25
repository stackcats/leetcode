with cte as (select num
from MyNumbers
group by num
having count(num) = 1)
select max(num) as num from cte;