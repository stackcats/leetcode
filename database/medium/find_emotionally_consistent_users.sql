-- Write your PostgreSQL query statement below
with c1 as (select user_id,
                   count(*) total
            from reactions
            group by user_id
            having count(*) >= 5),
     c2 as (select user_id,
                   reaction                                                         dominant_reaction,
                   rank() over (partition by user_id order by count(reaction) desc) rk,
                   count(*)                                                         ct
            from reactions
            where user_id in (select user_id from c1)
            group by user_id, reaction)
select c1.user_id                   user_id,
       dominant_reaction,
       round((ct + 0.0) / total, 2) reaction_ratio
from c1
         left join c2 using (user_id)
where rk = 1
  and (ct + 0.0) / total >= 0.6
order by reaction_ratio desc, user_id
