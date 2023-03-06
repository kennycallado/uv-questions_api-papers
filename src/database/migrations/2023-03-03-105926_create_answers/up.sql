CREATE TABLE IF NOT EXISTS answers (
    id SERIAL PRIMARY KEY,
    question_id SERIAL NOT NULL,
    answer VARCHAR NOT NULL
);

ALTER TABLE answers REPLICA IDENTITY FULL;

INSERT INTO answers VALUES
  (1, 1, 'answer 1'),
  (2, 1, 'answer 2'),
  (3, 2, 'answer 3'),
  (4, 2, 'answer 4'),
  (5, 1, 'answer 5') ON CONFLICT DO NOTHING
  ;

-- update the sequence to the max id
SELECT setval('answers_id_seq', (SELECT MAX(id) FROM answers));

CREATE PUBLICATION answers_pub FOR TABLE answers;
