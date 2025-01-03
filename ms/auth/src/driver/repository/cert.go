package repository

import (
	"auth/driver/query"
	"context"

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

func (r CertRepo) CreateScope(ctx context.Context) {}

func (r CertRepo) GetScope(ctx context.Context) {}

func (r CertRepo) UpdateScope(ctx context.Context) {}

func (r CertRepo) DeleteScope(ctx context.Context) {}

func (r CertRepo) CreateRole(ctx context.Context) {}

func (r CertRepo) GetRole(ctx context.Context) {}

func (r CertRepo) UpdateRole(ctx context.Context) {}

func (r CertRepo) DeleteRole(ctx context.Context) {}
