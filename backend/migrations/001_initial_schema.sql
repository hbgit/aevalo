-- Initial database schema for Aevalo
-- Based on class diagram UML documentation

-- ==================== ENUMERATIONS ====================

CREATE TYPE evaluation_status AS ENUM ('DRAFT', 'OPEN', 'CLOSED', 'ARCHIVED');
CREATE TYPE scale_type AS ENUM ('LIKERT', 'FREQUENCY', 'PAIRED_COMPARISON', 'FIXED_SUM');
CREATE TYPE collaborator_role AS ENUM ('OWNER', 'EDITOR', 'VIEWER');

-- ==================== CORE ENTITIES ====================

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    color VARCHAR(7) NOT NULL DEFAULT '#3B82F6',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE evaluations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    category_id UUID REFERENCES categories(id) ON DELETE SET NULL,
    title VARCHAR(500) NOT NULL,
    description TEXT,
    status evaluation_status NOT NULL DEFAULT 'DRAFT',
    scale_type scale_type NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    published_at TIMESTAMPTZ,
    closed_at TIMESTAMPTZ
);

CREATE TABLE questions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    evaluation_id UUID NOT NULL REFERENCES evaluations(id) ON DELETE CASCADE,
    "order" INTEGER NOT NULL,
    text TEXT NOT NULL,
    scale_type scale_type NOT NULL,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    UNIQUE(evaluation_id, "order")
);

CREATE TABLE responses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    question_id UUID NOT NULL REFERENCES questions(id) ON DELETE CASCADE,
    evaluation_id UUID NOT NULL REFERENCES evaluations(id) ON DELETE CASCADE,
    respondent_id VARCHAR(255) NOT NULL,
    answer_value JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ==================== COLLABORATION & ACCESS ====================

CREATE TABLE public_links (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    evaluation_id UUID NOT NULL REFERENCES evaluations(id) ON DELETE CASCADE,
    uuid VARCHAR(255) UNIQUE NOT NULL,
    short_url VARCHAR(255) UNIQUE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ,
    is_active BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE TABLE collaborators (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    evaluation_id UUID NOT NULL REFERENCES evaluations(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role collaborator_role NOT NULL,
    added_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(evaluation_id, user_id)
);

-- ==================== TEMPLATES ====================

CREATE TABLE templates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    scale_type scale_type NOT NULL,
    structure JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ==================== ANALYTICS ====================

CREATE TABLE analytics_results (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    evaluation_id UUID NOT NULL REFERENCES evaluations(id) ON DELETE CASCADE,
    total_responses INTEGER NOT NULL,
    response_rate DOUBLE PRECISION NOT NULL,
    generated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    metrics JSONB NOT NULL DEFAULT '{}'::jsonb,
    insights TEXT,
    UNIQUE(evaluation_id)
);

-- ==================== INDEXES ====================

CREATE INDEX idx_categories_user_id ON categories(user_id);
CREATE INDEX idx_evaluations_user_id ON evaluations(user_id);
CREATE INDEX idx_evaluations_category_id ON evaluations(category_id);
CREATE INDEX idx_evaluations_status ON evaluations(status);
CREATE INDEX idx_questions_evaluation_id ON questions(evaluation_id);
CREATE INDEX idx_responses_question_id ON responses(question_id);
CREATE INDEX idx_responses_evaluation_id ON responses(evaluation_id);
CREATE INDEX idx_responses_respondent_id ON responses(respondent_id);
CREATE INDEX idx_public_links_evaluation_id ON public_links(evaluation_id);
CREATE INDEX idx_public_links_uuid ON public_links(uuid);
CREATE INDEX idx_collaborators_evaluation_id ON collaborators(evaluation_id);
CREATE INDEX idx_collaborators_user_id ON collaborators(user_id);

-- ==================== TRIGGERS ====================

-- Auto-update updated_at on users
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_users_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_evaluations_updated_at
    BEFORE UPDATE ON evaluations
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
