package service

import (
	"auth/router/response"
	"context"
	"net/http"
)

type Autheticator interface {
	ProtagonistLogin(ctx context.Context, lid, psswd, secretKey string) (*response.Login, error)
	SupporterLogin(ctx context.Context, lid, psswd, secretKey string) (*response.Login, error)
	Logout(ctx context.Context, w http.ResponseWriter) error
	Refresh(ctx context.Context, rToken, secretKey string) (*response.Login, error)
}

type Certificator interface{}
