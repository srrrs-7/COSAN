CREATE TABLE IF NOT EXISTS staff_roles (
    staff_role_id BIGSERIAL,
    company_id            BIGINT,
    staff_id              BIGINT,
    staff_role            SMALLINT NOT NULL,
    created_at            TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at            TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (company_id, staff_id),
    UNIQUE (staff_role_id)
);
COMMENT ON TABLE staff_roles IS 'staff authentication information';
COMMENT ON COLUMN staff_roles.company_id IS 'company id';
COMMENT ON COLUMN staff_roles.staff_id IS 'staff id';
COMMENT ON COLUMN staff_roles.staff_role IS 'staff role 4:staff, 2:manager, 1:admin';

CREATE TABLE IF NOT EXISTS certificate_domains (
    certificate_domain_id BIGSERIAL,
    name                  VARCHAR(50) NOT NULL,
    created_at            TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at            TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (certificate_domain_id)
);
COMMENT ON TABLE certificate_domains IS 'service domain information';
COMMENT ON COLUMN certificate_domains.name IS 'service domain name';


CREATE TABLE IF NOT EXISTS staff_certificate_scopes (
    staff_certificate_scope_id  BIGSERIAL,
    company_id            BIGINT,
    staff_id              BIGINT,
    certificate_domain_id BIGINT,
    created_at            TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at            TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (company_id, staff_id, certificate_domain_id),
    UNIQUE (staff_certificate_scope_id),
    FOREIGN KEY (certificate_domain_id) REFERENCES certificate_domains(certificate_domain_id)
);
COMMENT ON TABLE staff_certificate_scopes IS 'available service domain information';
COMMENT ON COLUMN staff_certificate_scopes.company_id IS 'company id';
COMMENT ON COLUMN staff_certificate_scopes.staff_id IS 'staff id';
COMMENT ON COLUMN staff_certificate_scopes.certificate_domain_id IS 'avairable domain id';


INSERT INTO certificate_domains (name)
VALUES ('audit'), ('stamp'), ('holiday'), ('master'), ('shift'), ('summary');
