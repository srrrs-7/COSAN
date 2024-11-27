package repository

import (
	"auth/driver/query"

	"gorm.io/gorm"
)

type CertRepo struct {
	Db    *gorm.DB
	Query query.CertQuery
}

func NewCertRepo(db *gorm.DB, q query.CertQuery) CertRepo {
	return CertRepo{
		Db:    db,
		Query: q,
	}
}
