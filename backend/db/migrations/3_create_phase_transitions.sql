-- Create lifecycle phase enum
CREATE TYPE lifecycle_phase AS ENUM (
    'proposal',
    'requirements',
    'design',
    'implementation',
    'testing',
    'deployment',
    'maintenance',
    'closed'
);

-- Add current_phase to projects table
ALTER TABLE projects
ADD COLUMN current_phase lifecycle_phase NOT NULL DEFAULT 'proposal';

-- Create phase transitions table
CREATE TABLE phase_transitions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    phase lifecycle_phase NOT NULL,
    description TEXT NOT NULL,
    attachments TEXT[],
    approved_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Add indexes
CREATE INDEX idx_phase_transitions_project ON phase_transitions(project_id);
CREATE INDEX idx_phase_transitions_approved_by ON phase_transitions(approved_by);