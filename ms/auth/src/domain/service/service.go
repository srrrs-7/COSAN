package service

import (
	"auth/router/request"
	"auth/router/response"
	"context"
	"net/http"
)

type User interface {
	UserLogin(ctx context.Context, lid, psswd, secretKey string) (*response.Login, error)
}

type Autheticator interface {
	ProtagonistLogin(ctx context.Context, lid, psswd, secretKey string) (*response.Login, error)
	SupporterLogin(ctx context.Context, lid, psswd, secretKey string) (*response.Login, error)
	Logout(ctx context.Context, w http.ResponseWriter) error
	Refresh(ctx context.Context, rToken, secretKey string) (*response.Login, error)
}

type Certificator interface {
	CreateScope(ctx context.Context, req request.CreateScopeRequest) error
	GetScope(ctx context.Context, req request.GetScopeRequest) (*response.GetScopeResponse, error)
	UpdateScope(ctx context.Context, req request.UpdateScopeRequest) error
	DeleteScope(ctx context.Context, req request.DeleteScopeRequest) error
	CreateRole(ctx context.Context, req request.CreateRoleRequest) error
	GetRole(ctx context.Context, req request.GetRoleRequest) (*response.GetRoleResponse, error)
	UpdateRole(ctx context.Context, req request.UpdateRoleRequest) error
	DeleteRole(ctx context.Context, req request.DeleteRoleRequest) error
}
