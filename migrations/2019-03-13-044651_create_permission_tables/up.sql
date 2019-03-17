CREATE TABLE roles (
id SERIAL PRIMARY KEY,
name VARCHAR NOT NULL
);

CREATE TYPE action_type AS ENUM ('create', 'read', 'update', 'delete');
CREATE TYPE action_modifier AS ENUM ('all', 'self');

CREATE TABLE permissions (
id SERIAL PRIMARY KEY,
role_id INTEGER REFERENCES roles(id) NOT NULL,
resource_name VARCHAR,
action action_type,
modifier action_modifier
);

CREATE TABLE user_roles (
id SERIAL PRIMARY KEY,
user_id INTEGER REFERENCES users(id) NOT NULL,
role_id INTEGER REFERENCES roles(id) NOT NULL
);
