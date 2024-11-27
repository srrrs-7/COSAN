package query

type CertQuery struct{}

func NewCertQuery() CertQuery {
	return CertQuery{}
}

func (q CertQuery) InsertScopeQuery() string {
	sql := `
		INSERT INTO staff_roles 
			(company_id, staff_id, staff_role)
		VALUES 
			(?, ?, ?)
	`
	return sql
}

func (q CertQuery) UpdateScopeQuery() string {
	sql := `
		UPDATE staff_roles
			SET staff_role = ?
		WHERE company_id = ?
			AND staff_id = ?
	`
	return sql
}

func (q CertQuery) InsertRoleQuery() string {
	sql := `
		INSERT INTO certificate_scopes 
			(company_id, staff_id, certificate_domain_id)
		VALUES 
			(?, ?, ?)
	`
	return sql
}

func (q CertQuery) UpdateRoleQuery() string {
	sql := `
		UPDATE certificate_scopes
			SET certificate_domain_id = ?
		WHERE company_id = ?
			AND staff_id = ?
			AND certificate_domain_id = ?
	`
	return sql
}

func (q CertQuery) InsertCertDomainQuery() string {
	sql := `
		INSERT INTO certificate_domains 
			(company_id, certificate_domain_name)
		VALUES 
			(?, ?)
	`
	return sql
}

func (q CertQuery) UpdateCertDomainQuery() string {
	sql := `
		UPDATE certificate_domains
			SET certificate_domain_name = ?
		WHERE company_id = ?
			AND certificate_domain_id = ?
	`
	return sql
}
