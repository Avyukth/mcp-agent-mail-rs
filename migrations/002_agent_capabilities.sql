CREATE TABLE agent_capabilities (
    id INTEGER PRIMARY KEY,
    agent_id INTEGER NOT NULL REFERENCES agents(id),
    capability TEXT NOT NULL,
    granted_at TEXT NOT NULL,
    granted_by INTEGER REFERENCES agents(id),
    expires_at TEXT,
    UNIQUE (agent_id, capability)
);
CREATE INDEX idx_agent_capabilities_agent_id ON agent_capabilities(agent_id);
