package repository

import (
	"auth/domain/entity"
	"auth/driver/model"
	"auth/driver/query"
	"context"

	"gorm.io/gorm"
)

type AuthRepo struct {
	Db    *gorm.DB
	Query query.AuthQuery
}

func NewAuthRepo(db *gorm.DB, q query.AuthQuery) AuthRepo {
	return AuthRepo{
		Db:    db,
		Query: q,
	}
}

func (r AuthRepo) Login(ctx context.Context, uid int64, secretKey string) (*entity.Token, error) {
	var role model.UserRole
	var scopes []model.UserScope
	if err := r.Db.WithContext(ctx).Transaction(func(tx *gorm.DB) error {
		authSql, scopeSql := r.Query.LoginQuery(tx, uid)
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

	return model.TokenEntity(role, scopes, secretKey), nil
}
