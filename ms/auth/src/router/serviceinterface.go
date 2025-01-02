package router

import (
	"auth/router/response"
	"context"
)

type Autheticator interface {
	ProtagonistLogin(ctx context.Context, lid, psswd string) (*response.Login, error)
	SupporterLogin(ctx context.Context, lid, psswd string) (*response.Login, error)
	Logout(ctx context.Context, aToken string) error
	Refresh(ctx context.Context, aToken string) error
}

type Certificator interface{}
