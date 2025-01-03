package repository

import (
	"auth/domain/entity"
	"auth/driver/model"
	"auth/driver/query"
	"context"

	"gorm.io/gorm"
)

type AuthRepo struct {
	db    *gorm.DB
	query query.AuthQuery
}

func NewAuthRepo(db *gorm.DB, q query.AuthQuery) AuthRepo {
	return AuthRepo{
		db:    db,
		query: q,
	}
}

func (r AuthRepo) Login(ctx context.Context, uid int64, secretKey string) (*entity.Token, error) {
	var role model.UserRole
	var scopes []model.UserScope
	if err := r.db.WithContext(ctx).Transaction(func(tx *gorm.DB) error {
		authSql, scopeSql := r.query.LoginQuery(tx, uid)
		res := authSql.Scan(&role)
		if res.Error != nil {
			return res.Error
		}

		res = scopeSql.Scan(&scopes)
		if res.Error != nil {
			return res.Error
		}
		return nil
	}); err != nil {
		return nil, err
	}

	return model.TokenEntity(uid, role, scopes, secretKey), nil
}
