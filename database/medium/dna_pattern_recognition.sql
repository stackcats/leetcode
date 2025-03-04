SELECT sample_id,
       dna_sequence,
       species,
       CASE
           WHEN dna_sequence ~ '^ATG' THEN 1
           ELSE 0
           END has_start,
       CASE
           WHEN dna_sequence ~ '(TAA|TAG|TGA)$' THEN 1
           ELSE 0
           END has_stop,
       CASE
           WHEN dna_sequence ~ 'ATAT' THEN 1
           ELSE 0
           END has_atat,
       CASE
           WHEN dna_sequence ~ 'G{3}' THEN 1
           ELSE 0
           END has_ggg
FROM Samples
ORDER BY sample_id
