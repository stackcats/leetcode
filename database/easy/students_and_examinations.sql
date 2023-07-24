select st.student_id,
st.student_name,
sbj.subject_name,
count(ex.subject_name) as attended_exams
from Students st
join Subjects sbj
left join Examinations ex
on ex.student_id = st.student_id
and ex.subject_name = sbj.subject_name
group by st.student_id, sbj.subject_name
order by st.student_id, sbj.subject_name
