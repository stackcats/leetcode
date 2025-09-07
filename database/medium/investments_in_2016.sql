with cte as (
    select *,
    count (1) over (partition by tiv_2015) ct1,
    count (1) over (PARTITION BY lat, lon) ct2
    from Insurance
)
select round(sum(tiv_2016)::numeric, 2) as tiv_2016 from cte
where ct1 > 1 and ct2 = 1

