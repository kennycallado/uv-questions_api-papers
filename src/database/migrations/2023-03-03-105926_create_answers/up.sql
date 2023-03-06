CREATE TABLE IF NOT EXISTS answers (
    id SERIAL PRIMARY KEY,
    answer VARCHAR NOT NULL
);

ALTER TABLE answers REPLICA IDENTITY FULL;

INSERT INTO answers VALUES
  (1, 'answer 1'),
  (2, 'answer 2'),
  (3, 'answer 3'),
  (4, 'answer 4'),
  (5, 'answer 5') ON CONFLICT DO NOTHING
  ;

-- update the sequence to the max id
SELECT setval('answers_id_seq', (SELECT MAX(id) FROM answers));

CREATE PUBLICATION answers_pub FOR TABLE answers;
