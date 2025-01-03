package query

import (
	"auth/driver/model"

	"gorm.io/gorm"
)

type CertQuery struct{}

func NewCertQuery() CertQuery {
	return CertQuery{}
}

func (q CertQuery) InsertRoleQuery(db *gorm.DB, m model.InsertRole) *gorm.DB {
	sql := db.Exec(`
		INSERT INTO 
			user_roles (user_id, user_role)
		VALUES 
			(?, ?)
	`, m.Uid, m.Role,
	)

	return sql
}

func (q CertQuery) SelectRoleQuery(db *gorm.DB, m model.SelectRole) *gorm.DB {
	sql := db.Raw(`
		SELECT
			user_id, user_role
		FROM 
			user_roles
		WHERE 
			user_id = ?
			AND user_role = ?
	`, m.Uid, m.Role,
	)

	return sql
}

func (q CertQuery) UpdateRoleQuery(db *gorm.DB, m model.UpdateRole) *gorm.DB {
	sql := db.Exec(`
		UPDATE user_roles
			SET user_role = ?
		WHERE 
			user_id = ?
			AND user_role = ?
	`, m.Uid, m.Role,
	)

	return sql
}

func (q CertQuery) DeleteRoleQuery(db *gorm.DB, m model.DeleteRole) *gorm.DB {
	sql := db.Exec(`
		DELETE FROM 
			user_roles
		WHERE 
			AND user_id = ?
			AND user_role = ?
	`, m.Uid, m.Role,
	)

	return sql
}

func (q CertQuery) InsertScopeQuery(db *gorm.DB, m model.InsertScope) *gorm.DB {
	sql := db.Exec(`
		INSERT INTO
			user_certificate_scopes (user_id, certificate_domain_id, authority)
		VALUES
			(?, ?, ?)
	`, m.Uid, m.Cid, m.Auth,
	)

	return sql
}

func (q CertQuery) SelectScopeQuery(db *gorm.DB, m model.SelectScope) *gorm.DB {
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
	`, m.Uid, m.Cid,
	)

	return sql
}

func (q CertQuery) UpdateScopeQuery(db *gorm.DB, m model.UpdateScope) *gorm.DB {
	sql := db.Exec(`
		UPDATE user_certificate_scopes
			SET authority = ?
		WHERE 
			user_id = ?
			AND certificate_domain_id = ?
	`, m.Auth, m.Uid, m.Cid,
	)

	return sql
}

func (q CertQuery) DeleteScopeQuery(db *gorm.DB, m model.DeleteScope) *gorm.DB {
	sql := db.Exec(`
		DELETE 
			FROM user_certificate_scopes
		WHERE 
			user_id = ?
			AND certificate_domain_id = ?
	`, m.Uid, m.Cid,
	)

	return sql
}
