package query

import "gorm.io/gorm"

type CertQuery struct{}

func NewCertQuery() CertQuery {
	return CertQuery{}
}

func (q CertQuery) InsertRoleQuery(db *gorm.DB, uid int64, role string) *gorm.DB {
	sql := db.Exec(`
		INSERT INTO 
			user_roles (user_id, user_role)
		VALUES 
			(?, ?)
	`, uid, role,
	)

	return sql
}

func (q CertQuery) SelectRoleQuery(db *gorm.DB, uid int64, role string) *gorm.DB {
	sql := db.Raw(`
		SELECT
			user_id, user_role
		FROM 
			user_roles
		WHERE 
			user_id = ?
			AND user_role = ?
	`, uid, role,
	)

	return sql
}

func (q CertQuery) UpdateRoleQuery(db *gorm.DB, uid int64, role string) *gorm.DB {
	sql := db.Exec(`
		UPDATE user_roles
			SET user_role = ?
		WHERE 
			user_id = ?
			AND user_role = ?
	`, uid, role,
	)

	return sql
}

func (q CertQuery) DeleteRoleQuery(db *gorm.DB, uid int64, role string) *gorm.DB {
	sql := db.Exec(`
		DELETE FROM 
			user_roles
		WHERE 
			AND user_id = ?
			AND user_role = ?
	`, uid, role,
	)

	return sql
}

func (q CertQuery) InsertScopeQuery(db *gorm.DB, uid int64, cid, auth string) *gorm.DB {
	sql := db.Exec(`
		INSERT INTO
			user_certificate_scopes (user_id, certificate_domain_id, authority)
		VALUES
			(?, ?, ?)
	`, uid, cid, auth,
	)

	return sql
}

func (q CertQuery) SelectScopeQuery(db *gorm.DB, uid int64, cid string) *gorm.DB {
	sql := db.Raw(`
		SELECT
			user_id,
			certificate_domain_id,
			authority
		FROM 
			user_certificate_scopes
		WHERE 
			user_id = ?
			AND certificate_domain_id = ?
	`, uid, cid,
	)

	return sql
}

func (q CertQuery) UpdateScopeQuery(db *gorm.DB, uid int64, cid, auth string) *gorm.DB {
	sql := db.Exec(`
		UPDATE user_certificate_scopes
			SET authority = ?
		WHERE 
			user_id = ?
			AND certificate_domain_id = ?
	`, auth, uid, cid,
	)

	return sql
}

func (q CertQuery) DeleteScopeQuery(db *gorm.DB, uid int64, cid string) *gorm.DB {
	sql := db.Exec(`
		DELETE 
			FROM user_certificate_scopes
		WHERE 
			user_id = ?
			AND certificate_domain_id = ?
	`, uid, cid,
	)

	return sql
}
