CREATE TABLE IF NOT EXISTS paper_answers (
  id SERIAL PRIMARY KEY,
  paper_id SERIAL NOT NULL,
  answer_id SERIAL NOT NULL,
  CONSTRAINT fk_panswers_paper  FOREIGN KEY (paper_id) REFERENCES papers(id) ON DELETE CASCADE,
  CONSTRAINT fk_panswers_answer FOREIGN KEY (answer_id) REFERENCES answers(id) ON DELETE CASCADE
);

ALTER TABLE paper_answers REPLICA IDENTITY FULL;

INSERT INTO paper_answers (id, paper_id, answer_id) VALUES
  (1, 1, 1),
  (2, 1, 2),
  (3, 1, 3),
  (4, 2, 4),
  (5, 2, 5)
  ;

-- update the sequence to the max id
SELECT setval('paper_answers_id_seq', (SELECT MAX(id) FROM paper_answers));

CREATE PUBLICATION paper_answers_pub FOR TABLE paper_answers;
