#!/usr/bin/env bash
set -euo pipefail

DB_CONTAINER=${DB_CONTAINER:-aevalo-db}
DB_USER=${DB_USER:-aevalo}
DB_NAME=${DB_NAME:-aevalo_db}

cat <<'SQL' | docker exec -i "$DB_CONTAINER" psql -v ON_ERROR_STOP=1 -U "$DB_USER" -d "$DB_NAME"
BEGIN;

TRUNCATE TABLE
  responses,
  questions,
  public_links,
  collaborators,
  analytics_results,
  evaluations,
  categories,
  templates,
  users
CASCADE;

-- 50 users
INSERT INTO users (email, password_hash, name)
SELECT
  'user' || lpad(gs::text, 2, '0') || '@aevalo.dev',
  crypt('Password123!', gen_salt('bf')),
  'User ' || lpad(gs::text, 2, '0')
FROM generate_series(1, 50) AS gs;

-- 10 categories
WITH u AS (
  SELECT id, row_number() OVER (ORDER BY created_at) AS rn
  FROM users
)
INSERT INTO categories (user_id, name, description, color)
SELECT
  u.id,
  format('Categoria %s', u.rn),
  format('Descrição da categoria %s', u.rn),
  '#3B82F6'
FROM u
WHERE u.rn <= 10;

-- 50 evaluations
WITH u AS (
  SELECT id, row_number() OVER (ORDER BY created_at) AS rn
  FROM users
),
 c AS (
  SELECT id, row_number() OVER (ORDER BY created_at) AS rn
  FROM categories
)
INSERT INTO evaluations (user_id, category_id, title, description, status, scale_type, created_at, updated_at, published_at)
SELECT
  u.id,
  c.id,
  format('Avaliação %s', gs),
  format('Descrição da avaliação %s', gs),
  CASE (gs % 4)
    WHEN 0 THEN 'CLOSED'
    WHEN 1 THEN 'OPEN'
    WHEN 2 THEN 'DRAFT'
    ELSE 'ARCHIVED'
  END::evaluation_status,
  CASE (gs % 4)
    WHEN 0 THEN 'LIKERT'
    WHEN 1 THEN 'FREQUENCY'
    WHEN 2 THEN 'PAIRED_COMPARISON'
    ELSE 'FIXED_SUM'
  END::scale_type,
  NOW() - (gs || ' days')::interval,
  NOW() - (gs || ' days')::interval,
  CASE WHEN (gs % 4) = 1 THEN NOW() - (gs || ' days')::interval ELSE NULL END
FROM generate_series(1, 50) AS gs
JOIN u ON u.rn = ((gs - 1) % 50) + 1
LEFT JOIN c ON c.rn = ((gs - 1) % 10) + 1;

-- 3 questions per evaluation
WITH e AS (
  SELECT id, row_number() OVER (ORDER BY created_at) AS rn, scale_type
  FROM evaluations
)
INSERT INTO questions (evaluation_id, "order", text, scale_type, metadata)
SELECT
  e.id,
  q,
  format('Pergunta %s da avaliação %s', q, e.rn),
  e.scale_type,
  CASE e.scale_type
    WHEN 'LIKERT' THEN jsonb_build_object('min_value', 1, 'max_value', 5, 'labels', jsonb_build_array('Discordo', 'Neutro', 'Concordo'))
    WHEN 'FREQUENCY' THEN jsonb_build_object('options', jsonb_build_array('Nunca', 'Raramente', 'Às vezes', 'Frequentemente', 'Sempre'))
    WHEN 'PAIRED_COMPARISON' THEN jsonb_build_object('items', jsonb_build_array('Opção A', 'Opção B', 'Opção C'))
    WHEN 'FIXED_SUM' THEN jsonb_build_object('total', 100, 'items', jsonb_build_array('Item 1', 'Item 2', 'Item 3'))
    ELSE '{}'::jsonb
  END
FROM e
CROSS JOIN generate_series(1, 3) AS q;

-- 2 responses per question
WITH q AS (
  SELECT id, scale_type, row_number() OVER (ORDER BY id) AS rn
  FROM questions
),
 e AS (
  SELECT id, row_number() OVER (ORDER BY created_at) AS rn
  FROM evaluations
)
INSERT INTO responses (question_id, evaluation_id, respondent_id, answer_value)
SELECT
  q.id,
  e.id,
  format('resp-%s-%s', q.rn, r),
  CASE q.scale_type
    WHEN 'LIKERT' THEN to_jsonb((r % 5) + 1)
    WHEN 'FREQUENCY' THEN to_jsonb((r % 5) + 1)
    WHEN 'PAIRED_COMPARISON' THEN jsonb_build_object('winner', 'Opção A')
    WHEN 'FIXED_SUM' THEN jsonb_build_object('Item 1', 40, 'Item 2', 35, 'Item 3', 25)
    ELSE '{}'::jsonb
  END
FROM q
JOIN e ON e.rn = ((q.rn - 1) % 50) + 1
CROSS JOIN generate_series(1, 2) AS r;

-- public links for OPEN evaluations
INSERT INTO public_links (evaluation_id, uuid, short_url, is_active)
SELECT
  id,
  gen_random_uuid()::text,
  format('https://aevalo.dev/e/%s', gen_random_uuid()::text),
  TRUE
FROM evaluations
WHERE status = 'OPEN';

-- collaborators (20)
WITH e AS (
  SELECT id, row_number() OVER (ORDER BY created_at) AS rn
  FROM evaluations
),
 u AS (
  SELECT id, row_number() OVER (ORDER BY created_at) AS rn
  FROM users
)
INSERT INTO collaborators (evaluation_id, user_id, role)
SELECT
  e.id,
  u.id,
  'EDITOR'::collaborator_role
FROM e
JOIN u ON u.rn = ((e.rn + 1) % 50) + 1
WHERE e.rn <= 20;

-- templates (5)
INSERT INTO templates (name, description, scale_type, structure)
SELECT
  format('Template %s', gs),
  format('Descrição do template %s', gs),
  CASE (gs % 4)
    WHEN 0 THEN 'LIKERT'
    WHEN 1 THEN 'FREQUENCY'
    WHEN 2 THEN 'PAIRED_COMPARISON'
    ELSE 'FIXED_SUM'
  END::scale_type,
  jsonb_build_object('items', jsonb_build_array('Q1', 'Q2', 'Q3'))
FROM generate_series(1, 5) AS gs;

-- analytics_results for CLOSED evaluations
INSERT INTO analytics_results (evaluation_id, total_responses, response_rate, metrics, insights)
SELECT
  id,
  20,
  0.4,
  jsonb_build_object('mean', 3.7, 'median', 4, 'std_dev', 0.8),
  'Resultados gerados automaticamente.'
FROM evaluations
WHERE status = 'CLOSED'
LIMIT 10;

COMMIT;
SQL

echo "Seed concluído com sucesso."
