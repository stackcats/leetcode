-- Write your PostgreSQL query statement below
with c1 as (select user_id,
                   action_date,
                   action,
                   row_number() over (partition by user_id, action order by action_date) r
            from activity),
     c2 as (select *,
                   action_date + -r * interval '1 day' start_date
            from c1)
select user_id,
       action,
       count(1)         streak_length,
       min(action_date) start_date,
       max(action_date) end_date
from c2
group by user_id, start_date, action
having count(1) >= 5
order by streak_length desc, user_id
