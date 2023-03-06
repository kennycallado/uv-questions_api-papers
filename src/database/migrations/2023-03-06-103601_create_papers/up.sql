CREATE TABLE IF NOT EXISTS papers (
    id SERIAL PRIMARY KEY,
    form_id SERIAL NOT NULL,
    user_id SERIAL NOT NULL
);

ALTER TABLE papers REPLICA IDENTITY FULL;

INSERT INTO papers VALUES
  (1, 1, 1),
  (2, 2, 1),
  (3, 1, 2),
  (4, 2, 2),
  (5, 1, 3),
  (6, 2, 3) ON CONFLICT DO NOTHING
  ;

-- update the sequence to the max id
SELECT setval('papers_id_seq', (SELECT MAX(id) FROM papers));

CREATE PUBLICATION papers_pub FOR TABLE papers;
