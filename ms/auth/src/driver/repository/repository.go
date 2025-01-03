package repository

import (
	"auth/domain/entity"
	"auth/driver/model"
	"context"
)

type Autheticator interface {
	Login(ctx context.Context, uid int64, secretKey string) (*entity.Token, error)
}

type Certificater interface {
	CreateRole(ctx context.Context, m model.InsertRole) error
	GetRole(ctx context.Context, m model.SelectRole) (*entity.Role, error)
	UpdateRole(ctx context.Context, m model.UpdateRole) error
	DeleteRole(ctx context.Context, m model.DeleteRole) error
	CreateScope(ctx context.Context, m model.InsertScope) error
	GetScope(ctx context.Context, m model.SelectScope) (*entity.Scope, error)
	UpdateScope(ctx context.Context, m model.UpdateScope) error
	DeleteScope(ctx context.Context, m model.DeleteScope) error
}
