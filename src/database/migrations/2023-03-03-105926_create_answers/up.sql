-- create a type_enum
-- CREATE TYPE type_enum AS ENUM ('int', 'string');

CREATE TABLE IF NOT EXISTS answers (
    id SERIAL PRIMARY KEY,
    a_type VARCHAR(10) NOT NULL,
    answer VARCHAR NOT NULL
);

ALTER TABLE answers REPLICA IDENTITY FULL;

INSERT INTO answers VALUES
  (1, 'int', ''),
  (2, 'string', ''),
  (3, 'int', ''),
  (4, 'int', ''),
  (5, 'int', '') ON CONFLICT DO NOTHING
  ;

-- update the sequence to the max id
SELECT setval('answers_id_seq', (SELECT MAX(id) FROM answers));

CREATE PUBLICATION answers_pub FOR TABLE answers;
