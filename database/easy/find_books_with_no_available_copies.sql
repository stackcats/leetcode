WITH cte AS (SELECT book_id, COUNT(1) current_borrowers
             FROM borrowing_records
             WHERE return_date ISNULL
             GROUP BY book_id)
SELECT lb.book_id, title, author, genre, publication_year, current_borrowers
FROM library_books lb
         LEFT JOIN cte ON lb.book_id = cte.book_id
WHERE total_copies - current_borrowers = 0
ORDER BY current_borrowers DESC, title;
