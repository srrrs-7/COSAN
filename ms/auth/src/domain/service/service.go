package service

import (
	"auth/domain/entity"
	"context"
)

type Autheticator interface {
	Login(ctx context.Context, uid int64, secretKey string) (*entity.Token, error)
}

type Cacher interface {
	SetAccessToken(ctx context.Context, key string, token entity.Token) error
	SetRefreshToken(ctx context.Context, key string, token entity.Token) error
	Get(ctx context.Context, key string) (*entity.Token, error)
	Delete(ctx context.Context, key string) error
}

type Certificater interface{}
