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

func (r CertRepo) CreateRole(ctx context.Context, uid int64, rl string) error {
	if err := r.db.WithContext(ctx).Transaction(func(tx *gorm.DB) error {
		res := r.query.InsertRoleQuery(tx, uid, rl)
		if res.Error != nil {
			return res.Error
		}
		return nil
	}); err != nil {
		return err
	}

	return nil
}

func (r CertRepo) GetRole(ctx context.Context, uid int64, rl string) (*entity.Role, error) {
	var role model.GetRole
	if err := r.db.WithContext(ctx).Transaction(func(tx *gorm.DB) error {
		res := r.query.InsertRoleQuery(tx, uid, rl).Scan(&role)
		if res.Error != nil {
			return res.Error
		}
		return nil
	}); err != nil {
		return nil, err
	}

	return role.Entity(), nil
}

func (r CertRepo) UpdateRole(ctx context.Context, uid int64, rl string) error {
	if err := r.db.WithContext(ctx).Transaction(func(tx *gorm.DB) error {
		res := r.query.UpdateRoleQuery(tx, uid, rl)
		if res.Error != nil {
			return res.Error
		}
		return nil
	}); err != nil {
		return err
	}

	return nil
}

func (r CertRepo) DeleteRole(ctx context.Context, uid int64, rl string) error {
	if err := r.db.WithContext(ctx).Transaction(func(tx *gorm.DB) error {
		res := r.query.DeleteRoleQuery(tx, uid, rl)
		if res.Error != nil {
			return res.Error
		}
		return nil
	}); err != nil {
		return err
	}

	return nil
}

func (r CertRepo) CreateScope(ctx context.Context, uid int64, cid, auth string) error {
	if err := r.db.WithContext(ctx).Transaction(func(tx *gorm.DB) error {
		res := r.query.InsertScopeQuery(tx, uid, cid, auth)
		if res.Error != nil {
			return res.Error
		}
		return nil
	}); err != nil {
		return err
	}

	return nil
}

func (r CertRepo) GetScope(ctx context.Context, uid int64, cid string) (*entity.Scope, error) {
	var scope model.GetScope
	if err := r.db.WithContext(ctx).Transaction(func(tx *gorm.DB) error {
		res := r.query.SelectScopeQuery(tx, uid, cid).Scan(&scope)
		if res.Error != nil {
			return res.Error
		}
		return nil
	}); err != nil {
		return nil, err
	}

	return scope.Entity(), nil
}

func (r CertRepo) UpdateScope(ctx context.Context, uid int64, cid, auth string) error {
	if err := r.db.WithContext(ctx).Transaction(func(tx *gorm.DB) error {
		res := r.query.UpdateScopeQuery(tx, uid, cid, auth)
		if res.Error != nil {
			return res.Error
		}
		return nil
	}); err != nil {
		return err
	}

	return nil
}

func (r CertRepo) DeleteScope(ctx context.Context, uid int64, cid string) error {
	if err := r.db.WithContext(ctx).Transaction(func(tx *gorm.DB) error {
		res := r.query.DeleteScopeQuery(tx, uid, cid)
		if res.Error != nil {
			return res.Error
		}
		return nil
	}); err != nil {
		return err
	}

	return nil
}
