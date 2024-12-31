CREATE TABLE IF NOT EXISTS protagonists (
    protagonist_id BIGSERIAL,
    last_name VARCHAR(50) NOT NULL,
    first_name VARCHAR(50) NOT NULL,
    email VARCHAR(255) NOT NULL,
    country VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (protagonist_id),
    UNIQUE (email)
);
COMMENT ON TABLE protagonists IS 'protagonist is supported supporters';
COMMENT ON COLUMN protagonists.protagonist_id IS 'protagonist id';
COMMENT ON COLUMN protagonists.last_name IS 'protagonist last name';
COMMENT ON COLUMN protagonists.first_name IS 'protagonist first name';
COMMENT ON COLUMN protagonists.email IS 'protagonist email';
COMMENT ON COLUMN protagonists.country IS 'protagonist country';

CREATE TABLE IF NOT EXISTS supporters (
    supporter_id BIGSERIAL,
    last_name VARCHAR(50) NOT NULL,
    first_name VARCHAR(50) NOT NULL,
    email VARCHAR(255) NOT NULL,
    country VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (supporter_id),
    UNIQUE (email)
);
COMMENT ON TABLE supporters IS 'supporter support protagonists';
COMMENT ON COLUMN supporters.supporter_id IS 'supporter id';
COMMENT ON COLUMN supporters.last_name IS 'supporter last name';
COMMENT ON COLUMN supporters.first_name IS 'supporter first name';
COMMENT ON COLUMN supporters.email IS 'supporter email';
COMMENT ON COLUMN supporters.country IS 'supporter country';


CREATE TABLE IF NOT EXISTS protagonist_supporters (
    protagonist_supporter_id BIGSERIAL,
    protagonist_id BIGINT NOT NULL,
    supporter_id BIGINT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (protagonist_supporter_id),
    UNIQUE (protagonist_id, supporter_id),
    FOREIGN KEY (protagonist_id) REFERENCES protagonists(protagonist_id),
    FOREIGN KEY (supporter_id) REFERENCES supporters(supporter_id)
);
COMMENT ON TABLE protagonist_supporters IS 'protagonist supporter relationship';
COMMENT ON COLUMN protagonist_supporters.protagonist_supporter_id IS 'protagonist supporter id';
COMMENT ON COLUMN protagonist_supporters.protagonist_id IS 'protagonist id';
COMMENT ON COLUMN protagonist_supporters.supporter_id IS 'supporter id';
