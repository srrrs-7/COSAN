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

func (r AuthRepo) Login(ctx context.Context, cid, sid int64, secretKey string) (*entity.Token, error) {
	var auth model.Auth
	var scopes []model.Scope
	if err := r.Db.WithContext(ctx).Transaction(func(tx *gorm.DB) error {
		authSql, scopeSql := r.Query.LoginQuery(tx, cid, sid)
		res := authSql.Scan(&auth)
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

	return model.TokenEntity(auth, scopes, secretKey), nil
}
