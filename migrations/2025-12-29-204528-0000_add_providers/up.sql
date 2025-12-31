CREATE TYPE provider_type AS ENUM ('openai', 'anthropic');

CREATE TABLE providers (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    provider_type provider_type NOT NULL,
    api_endpoint VARCHAR(255) NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Insert default providers
INSERT INTO providers (name, provider_type, api_endpoint) VALUES
    ('OpenAI', 'openai', 'https://api.openai.com/v1/chat/completions'),
    ('Anthropic', 'anthropic', 'https://api.anthropic.com/v1/messages');
