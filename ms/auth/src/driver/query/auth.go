package query

import "gorm.io/gorm"

type AuthQuery struct{}

func NewAuthQuery() AuthQuery {
	return AuthQuery{}
}

func (q AuthQuery) LoginQuery(db *gorm.DB, uid int64) (authSql, scopeSql *gorm.DB) {
	authSql = db.Raw(`
		SELECT
			user_role_id,
			user_id,
			user_role
		FROM 
			user_roles
		WHERE 
			user_id = ?
		;
	`, uid,
	)

	scopeSql = db.Raw(`
		SELECT
			domain.certificate_domain_id,
			domain.domain_name,
			scope.authority
		FROM 
			user_certificate_scopes AS scope
		INNER JOIN 
			certificate_domains AS domain
				ON scope.certificate_domain_id = domain.certificate_domain_id
		WHERE
			scope.user_id = ?
		;
	`, uid,
	)

	return authSql, scopeSql
}
