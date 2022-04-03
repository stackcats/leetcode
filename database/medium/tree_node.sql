with cte as (
    select p_id, count(1) as ct
    from Tree
    group by p_id
)
select t.id, (case when t.p_id is null then 'Root' when cte.ct > 0 then 'Inner' else 'Leaf' end) as type
from Tree t
