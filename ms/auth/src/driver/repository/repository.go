package repository

import (
	"auth/domain/entity"
	"context"
)

type Autheticator interface {
	Login(ctx context.Context, uid int64, secretKey string) (*entity.Token, error)
}

type Certificater interface {
	CreateScope(ctx context.Context, uid int64, cid, auth string) error
	GetScope(ctx context.Context, uid int64, cid string) (*entity.Scope, error)
	UpdateScope(ctx context.Context, uid int64, cid, auth string) error
	DeleteScope(ctx context.Context, uid int64, cid string) error
	CreateRole(ctx context.Context, uid int64, rl string) error
	GetRole(ctx context.Context, uid int64, rl string) (*entity.Role, error)
	UpdateRole(ctx context.Context, uid int64, rl string) error
	DeleteRole(ctx context.Context, uid int64, rl string) error
}
