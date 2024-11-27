package repository

import (
	"auth/domain/entity"
	"context"
	"utils/cache"
)

type CacheRepo struct {
	repo cache.CacheRepo[entity.Token]
}

func NewCacheRepo(dns string) CacheRepo {
	return CacheRepo{
		repo: cache.NewCacheRepo[entity.Token](dns),
	}
}

func (r CacheRepo) SetAccessToken(ctx context.Context, key string, token entity.Token) error {
	return r.repo.Set(ctx, key, token, entity.ACCESSS_TOKEN_EXPIRED_AT)
}

func (r CacheRepo) SetRefreshToken(ctx context.Context, key string, token entity.Token) error {
	return r.repo.Set(ctx, key, token, entity.REFRESH_TOKEN_EXPIRED_AT)
}

func (r CacheRepo) Get(ctx context.Context, key string) (*entity.Token, error) {
	return r.repo.Get(ctx, key)
}

func (r CacheRepo) Delete(ctx context.Context, key string) error {
	return r.repo.Delete(ctx, key)
}
