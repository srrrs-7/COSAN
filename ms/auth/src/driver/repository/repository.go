package repository

import (
	"auth/domain/entity"
	"context"
)

type Autheticator interface {
	Login(ctx context.Context, uid int64, secretKey string) (*entity.Token, error)
}

type Certificater interface{}
