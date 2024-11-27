package query

import "gorm.io/gorm"

type AuthQuery struct{}

func NewAuthQuery() AuthQuery {
	return AuthQuery{}
}

func (q AuthQuery) LoginQuery(db *gorm.DB, cid, sid int64) (authSql, scopeSql *gorm.DB) {
	authSql = db.Raw(`
		SELECT
			staff_role_id,
			company_id,
			staff_id,
			staff_role
		FROM 
			staff_roles
		WHERE 
			company_id = ?
			AND staff_id = ?
		;
	`, cid, sid,
	)

	scopeSql = db.Raw(`
		SELECT
			domain.certificate_domain_id
		FROM 
			staff_certificate_scopes AS scope
		INNER JOIN 
			certificate_domains AS domain
			ON scope.certificate_domain_id = domain.certificate_domain_id
		WHERE
			scope.company_id = ?
			AND scope.staff_id = ?
		;
	`, cid, sid,
	)

	return authSql, scopeSql
}
