package router

import (
	"auth/router/response"
	"context"
)

type Autheticator interface {
	Login(ctx context.Context, lid string, psswd string) (*response.Login, error)
	Logout(ctx context.Context, aToken string) error
	Refresh(ctx context.Context, aToken string) error
}

type Certificator interface{}
