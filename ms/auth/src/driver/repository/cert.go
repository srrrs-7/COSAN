package repository

import (
	"auth/domain/entity"
	"auth/driver/model"
	"auth/driver/query"
	"context"

	"gorm.io/gorm"
)

type CertRepo struct {
	db    *gorm.DB
	query query.CertQuery
}

func NewCertRepo(db *gorm.DB, q query.CertQuery) CertRepo {
	return CertRepo{
		db:    db,
		query: q,
	}
}

func (r CertRepo) CreateRole(ctx context.Context, m model.InsertRole) error {
	if err := r.db.WithContext(ctx).Transaction(func(tx *gorm.DB) error {
		res := r.query.InsertRoleQuery(tx, m)
		if res.Error != nil {
			return res.Error
		}
		return nil
	}); err != nil {
		return err
	}

	return nil
}

func (r CertRepo) GetRole(ctx context.Context, m model.SelectRole) (*entity.Role, error) {
	var role model.SelectRole
	if err := r.db.WithContext(ctx).Transaction(func(tx *gorm.DB) error {
		res := r.query.SelectRoleQuery(tx, m).Scan(&role)
		if res.Error != nil {
			return res.Error
		}
		return nil
	}); err != nil {
		return nil, err
	}

	return role.Entity(), nil
}

func (r CertRepo) UpdateRole(ctx context.Context, m model.UpdateRole) error {
	if err := r.db.WithContext(ctx).Transaction(func(tx *gorm.DB) error {
		res := r.query.UpdateRoleQuery(tx, m)
		if res.Error != nil {
			return res.Error
		}
		return nil
	}); err != nil {
		return err
	}

	return nil
}

func (r CertRepo) DeleteRole(ctx context.Context, m model.DeleteRole) error {
	if err := r.db.WithContext(ctx).Transaction(func(tx *gorm.DB) error {
		res := r.query.DeleteRoleQuery(tx, m)
		if res.Error != nil {
			return res.Error
		}
		return nil
	}); err != nil {
		return err
	}

	return nil
}

func (r CertRepo) CreateScope(ctx context.Context, m model.InsertScope) error {
	if err := r.db.WithContext(ctx).Transaction(func(tx *gorm.DB) error {
		res := r.query.InsertScopeQuery(tx, m)
		if res.Error != nil {
			return res.Error
		}
		return nil
	}); err != nil {
		return err
	}

	return nil
}

func (r CertRepo) GetScope(ctx context.Context, m model.SelectScope) (*entity.Scope, error) {
	var scope model.SelectScope
	if err := r.db.WithContext(ctx).Transaction(func(tx *gorm.DB) error {
		res := r.query.SelectScopeQuery(tx, m).Scan(&scope)
		if res.Error != nil {
			return res.Error
		}
		return nil
	}); err != nil {
		return nil, err
	}

	return scope.Entity(), nil
}

func (r CertRepo) UpdateScope(ctx context.Context, m model.UpdateScope) error {
	if err := r.db.WithContext(ctx).Transaction(func(tx *gorm.DB) error {
		res := r.query.UpdateScopeQuery(tx, m)
		if res.Error != nil {
			return res.Error
		}
		return nil
	}); err != nil {
		return err
	}

	return nil
}

func (r CertRepo) DeleteScope(ctx context.Context, m model.DeleteScope) error {
	if err := r.db.WithContext(ctx).Transaction(func(tx *gorm.DB) error {
		res := r.query.DeleteScopeQuery(tx, m)
		if res.Error != nil {
			return res.Error
		}
		return nil
	}); err != nil {
		return err
	}

	return nil
}
