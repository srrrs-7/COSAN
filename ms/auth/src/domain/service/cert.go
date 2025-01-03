package service

import (
	"auth/driver/repository"
	"auth/router/request"
	"auth/router/response"
	"context"
)

type CertService struct {
	certRepo repository.Certificater
}

func NewCert(c repository.Certificater) CertService {
	return CertService{
		certRepo: c,
	}
}

func (c CertService) CreateScope(ctx context.Context, req request.CreateScopeRequest) (*response.CreateScopeResponse, error) {
	return nil, nil
}

func (c CertService) GetScope(ctx context.Context, req request.GetScopeRequest) (*response.GetScopeResponse, error) {
	return nil, nil
}

func (c CertService) UpdateScope(ctx context.Context, req request.UpdateScopeRequest) (*response.UpdateScopeResponse, error) {
	return nil, nil
}

func (c CertService) DeleteScope(ctx context.Context, req request.DeleteScopeRequest) (*response.DeleteScopeResponse, error) {
	return nil, nil
}

func (c CertService) CreateRole(ctx context.Context, req request.CreateRoleRequest) (*response.CreateRoleResponse, error) {
	return nil, nil
}

func (c CertService) GetRole(ctx context.Context, req request.GetRoleRequest) (*response.GetRoleResponse, error) {
	return nil, nil
}

func (c CertService) UpdateRole(ctx context.Context, req request.UpdateRoleRequest) (*response.UpdateRoleResponse, error) {
	return nil, nil
}

func (c CertService) DeleteRole(ctx context.Context, req request.DeleteRoleRequest) (*response.DeleteRoleResponse, error) {
	return nil, nil
}
