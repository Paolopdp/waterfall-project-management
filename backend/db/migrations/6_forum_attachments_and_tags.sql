-- Tags table
CREATE TABLE forum_tags (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(50) NOT NULL UNIQUE,
    description VARCHAR(200),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Thread tags junction table
CREATE TABLE thread_tags (
    thread_id UUID NOT NULL REFERENCES threads(id) ON DELETE CASCADE,
    tag_id UUID NOT NULL REFERENCES forum_tags(id) ON DELETE CASCADE,
    PRIMARY KEY (thread_id, tag_id)
);

-- Attachments table
CREATE TABLE forum_attachments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    filename VARCHAR(255) NOT NULL,
    content_type VARCHAR(100) NOT NULL,
    size_bytes BIGINT NOT NULL,
    storage_path TEXT NOT NULL,
    thread_id UUID REFERENCES threads(id) ON DELETE CASCADE,
    reply_id UUID REFERENCES replies(id) ON DELETE CASCADE,
    uploaded_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT thread_or_reply_attachment CHECK (
        (thread_id IS NOT NULL AND reply_id IS NULL) OR
        (thread_id IS NULL AND reply_id IS NOT NULL)
    )
);

-- Add full-text search columns
ALTER TABLE threads ADD COLUMN search_vector tsvector
    GENERATED ALWAYS AS (
        setweight(to_tsvector('english', coalesce(title, '')), 'A') ||
        setweight(to_tsvector('english', coalesce(content, '')), 'B')
    ) STORED;

CREATE INDEX threads_search_idx ON threads USING GIN (search_vector);
