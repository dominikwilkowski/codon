
CREATE TABLE IF NOT EXISTS samples
(
  id            SERIAL PRIMARY KEY,
  sample_type   VARCHAR(100),
  analyst       VARCHAR(100)
);
