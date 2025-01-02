CREATE TYPE USER_ROLE AS ENUM ('protagonist', 'supporter');
CREATE TABLE IF NOT EXISTS user_roles (
    user_role_id BIGSERIAL,
    user_id      BIGINT NOT NULL,
    user_role    USER_ROLE NOT NULL DEFAULT 'supporter',
    created_at   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (user_role_id),
    UNIQUE (user_id)
);
COMMENT ON TABLE user_roles IS 'user authentication information';
COMMENT ON COLUMN user_roles.user_id IS 'user id';
COMMENT ON COLUMN user_roles.user_role IS 'user role';

CREATE TYPE DOMAIN_NAME AS ENUM ('support');
CREATE TABLE IF NOT EXISTS certificate_domains (
    certificate_domain_id BIGSERIAL,
    domain_name           DOMAIN_NAME NOT NULL DEFAULT 'support',
    created_at            TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at            TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (certificate_domain_id)
);
COMMENT ON TABLE certificate_domains IS 'service domain information';
COMMENT ON COLUMN certificate_domains.domain_name IS 'service domain name';

CREATE TYPE AUTHORITY AS ENUM ('read', 'write');
CREATE TABLE IF NOT EXISTS user_certificate_scopes (
    user_certificate_scope_id  BIGSERIAL,
    user_id                    BIGINT NOT NULL,
    certificate_domain_id      BIGINT NOT NULL,
    authority                  AUTHORITY NOT NULL DEFAULT 'read',
    created_at                 TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at                 TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (user_certificate_scope_id),
    UNIQUE (user_id, certificate_domain_id),
    FOREIGN KEY (certificate_domain_id) REFERENCES certificate_domains(certificate_domain_id)
);
COMMENT ON TABLE user_certificate_scopes IS 'available service domain information';
COMMENT ON COLUMN user_certificate_scopes.user_id IS 'user id';
COMMENT ON COLUMN user_certificate_scopes.certificate_domain_id IS 'available domain id';
COMMENT ON COLUMN user_certificate_scopes.authority IS 'available authority';

INSERT INTO certificate_domains (domain_name)
VALUES ('support');
