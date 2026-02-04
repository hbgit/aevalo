-- Row Level Security (RLS) policies for Aevalo
-- Uses JWT subject claim: request.jwt.claim.sub

-- Helper to read current user id from JWT claims
CREATE OR REPLACE FUNCTION current_user_id()
RETURNS uuid
LANGUAGE sql
STABLE
AS $$
    SELECT NULLIF(current_setting('request.jwt.claim.sub', true), '')::uuid;
$$;

-- Required for password verification via crypt()
CREATE EXTENSION IF NOT EXISTS pgcrypto;

-- SECURITY DEFINER function for authentication
-- Returns minimal user info if credentials are valid
CREATE OR REPLACE FUNCTION auth_login(p_email text, p_password text)
RETURNS TABLE (
    id uuid,
    email varchar,
    name varchar
)
LANGUAGE sql
SECURITY DEFINER
SET search_path = public
AS $$
    SELECT u.id, u.email, u.name
    FROM users u
    WHERE u.email = p_email
      AND u.password_hash = crypt(p_password, u.password_hash)
    LIMIT 1;
$$;

-- ==================== RLS ENABLEMENT ====================

ALTER TABLE categories ENABLE ROW LEVEL SECURITY;
ALTER TABLE evaluations ENABLE ROW LEVEL SECURITY;
ALTER TABLE questions ENABLE ROW LEVEL SECURITY;
ALTER TABLE responses ENABLE ROW LEVEL SECURITY;
ALTER TABLE public_links ENABLE ROW LEVEL SECURITY;
ALTER TABLE collaborators ENABLE ROW LEVEL SECURITY;
ALTER TABLE analytics_results ENABLE ROW LEVEL SECURITY;
ALTER TABLE templates ENABLE ROW LEVEL SECURITY;

ALTER TABLE users ENABLE ROW LEVEL SECURITY;

-- ==================== CATEGORIES ====================

CREATE POLICY categories_select_own
ON categories
FOR SELECT
USING (user_id = current_user_id());

CREATE POLICY categories_insert_own
ON categories
FOR INSERT
WITH CHECK (user_id = current_user_id());

CREATE POLICY categories_update_own
ON categories
FOR UPDATE
USING (user_id = current_user_id())
WITH CHECK (user_id = current_user_id());

CREATE POLICY categories_delete_own
ON categories
FOR DELETE
USING (user_id = current_user_id());

-- ==================== EVALUATIONS ====================

CREATE POLICY evaluations_select_owner
ON evaluations
FOR SELECT
USING (user_id = current_user_id());

CREATE POLICY evaluations_select_collaborator
ON evaluations
FOR SELECT
USING (
    EXISTS (
        SELECT 1 FROM collaborators c
        WHERE c.evaluation_id = evaluations.id
          AND c.user_id = current_user_id()
    )
);

CREATE POLICY evaluations_select_public
ON evaluations
FOR SELECT
USING (
    status = 'OPEN'
    AND EXISTS (
        SELECT 1 FROM public_links pl
        WHERE pl.evaluation_id = evaluations.id
          AND pl.is_active = TRUE
          AND (pl.expires_at IS NULL OR pl.expires_at > NOW())
    )
);

CREATE POLICY evaluations_insert_owner
ON evaluations
FOR INSERT
WITH CHECK (user_id = current_user_id());

CREATE POLICY evaluations_update_owner
ON evaluations
FOR UPDATE
USING (user_id = current_user_id())
WITH CHECK (user_id = current_user_id());

CREATE POLICY evaluations_delete_owner
ON evaluations
FOR DELETE
USING (user_id = current_user_id());

-- ==================== QUESTIONS ====================

CREATE POLICY questions_select_owner
ON questions
FOR SELECT
USING (
    EXISTS (
        SELECT 1 FROM evaluations e
        WHERE e.id = questions.evaluation_id
          AND e.user_id = current_user_id()
    )
);

CREATE POLICY questions_select_collaborator
ON questions
FOR SELECT
USING (
    EXISTS (
        SELECT 1 FROM collaborators c
        WHERE c.evaluation_id = questions.evaluation_id
          AND c.user_id = current_user_id()
    )
);

CREATE POLICY questions_select_public
ON questions
FOR SELECT
USING (
    EXISTS (
        SELECT 1
        FROM evaluations e
        JOIN public_links pl ON pl.evaluation_id = e.id
        WHERE e.id = questions.evaluation_id
          AND e.status = 'OPEN'
          AND pl.is_active = TRUE
          AND (pl.expires_at IS NULL OR pl.expires_at > NOW())
    )
);

CREATE POLICY questions_insert_owner
ON questions
FOR INSERT
WITH CHECK (
    EXISTS (
        SELECT 1 FROM evaluations e
        WHERE e.id = questions.evaluation_id
          AND e.user_id = current_user_id()
    )
);

CREATE POLICY questions_update_owner
ON questions
FOR UPDATE
USING (
    EXISTS (
        SELECT 1 FROM evaluations e
        WHERE e.id = questions.evaluation_id
          AND e.user_id = current_user_id()
    )
)
WITH CHECK (
    EXISTS (
        SELECT 1 FROM evaluations e
        WHERE e.id = questions.evaluation_id
          AND e.user_id = current_user_id()
    )
);

CREATE POLICY questions_delete_owner
ON questions
FOR DELETE
USING (
    EXISTS (
        SELECT 1 FROM evaluations e
        WHERE e.id = questions.evaluation_id
          AND e.user_id = current_user_id()
    )
);

-- ==================== RESPONSES ====================

CREATE POLICY responses_select_owner
ON responses
FOR SELECT
USING (
    EXISTS (
        SELECT 1 FROM evaluations e
        WHERE e.id = responses.evaluation_id
          AND e.user_id = current_user_id()
    )
);

CREATE POLICY responses_insert_public
ON responses
FOR INSERT
WITH CHECK (
    EXISTS (
        SELECT 1 FROM evaluations e
        WHERE e.id = responses.evaluation_id
          AND e.status = 'OPEN'
    )
);

CREATE POLICY responses_delete_owner
ON responses
FOR DELETE
USING (
    EXISTS (
        SELECT 1 FROM evaluations e
        WHERE e.id = responses.evaluation_id
          AND e.user_id = current_user_id()
    )
);

-- ==================== PUBLIC LINKS ====================

CREATE POLICY public_links_select_public
ON public_links
FOR SELECT
USING (
    is_active = TRUE
    AND (expires_at IS NULL OR expires_at > NOW())
);

CREATE POLICY public_links_owner_all
ON public_links
FOR ALL
USING (
    EXISTS (
        SELECT 1 FROM evaluations e
        WHERE e.id = public_links.evaluation_id
          AND e.user_id = current_user_id()
    )
)
WITH CHECK (
    EXISTS (
        SELECT 1 FROM evaluations e
        WHERE e.id = public_links.evaluation_id
          AND e.user_id = current_user_id()
    )
);

-- ==================== COLLABORATORS ====================

CREATE POLICY collaborators_owner_all
ON collaborators
FOR ALL
USING (
    EXISTS (
        SELECT 1 FROM evaluations e
        WHERE e.id = collaborators.evaluation_id
          AND e.user_id = current_user_id()
    )
)
WITH CHECK (
    EXISTS (
        SELECT 1 FROM evaluations e
        WHERE e.id = collaborators.evaluation_id
          AND e.user_id = current_user_id()
    )
);

-- ==================== ANALYTICS RESULTS ====================

CREATE POLICY analytics_results_owner_all
ON analytics_results
FOR ALL
USING (
    EXISTS (
        SELECT 1 FROM evaluations e
        WHERE e.id = analytics_results.evaluation_id
          AND e.user_id = current_user_id()
    )
)
WITH CHECK (
    EXISTS (
        SELECT 1 FROM evaluations e
        WHERE e.id = analytics_results.evaluation_id
          AND e.user_id = current_user_id()
    )
);

-- ==================== TEMPLATES ====================

CREATE POLICY templates_select_public
ON templates
FOR SELECT
USING (TRUE);

-- ==================== USERS ====================

CREATE POLICY users_select_own
ON users
FOR SELECT
USING (id = current_user_id());

CREATE POLICY users_update_own
ON users
FOR UPDATE
USING (id = current_user_id())
WITH CHECK (id = current_user_id());

CREATE POLICY users_delete_own
ON users
FOR DELETE
USING (id = current_user_id());
