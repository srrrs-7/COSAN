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

func (c CertService) CreateScope(ctx context.Context, req request.CreateScopeRequest) error {
	if err := c.certRepo.CreateScope(ctx, req.Uid, req.Cid, req.Auth); err != nil {
		return err
	}

	return nil
}

func (c CertService) GetScope(ctx context.Context, req request.GetScopeRequest) (*response.GetScopeResponse, error) {
	scope, err := c.certRepo.GetScope(ctx, req.Uid, req.Cid)
	if err != nil {
		return nil, err
	}
	return &response.GetScopeResponse{
		Uid:  scope.Uid,
		Cid:  scope.Cid,
		Auth: scope.Auth,
	}, nil
}

func (c CertService) UpdateScope(ctx context.Context, req request.UpdateScopeRequest) error {
	if err := c.certRepo.UpdateScope(ctx, req.Uid, req.Cid, req.Auth); err != nil {
		return err
	}

	return nil
}

func (c CertService) DeleteScope(ctx context.Context, req request.DeleteScopeRequest) error {
	if err := c.certRepo.DeleteScope(ctx, req.Uid, req.Cid); err != nil {
		return err
	}

	return nil
}

func (c CertService) CreateRole(ctx context.Context, req request.CreateRoleRequest) error {
	if err := c.certRepo.CreateRole(ctx, req.Uid, req.Role); err != nil {
		return err
	}

	return nil
}

func (c CertService) GetRole(ctx context.Context, req request.GetRoleRequest) (*response.GetRoleResponse, error) {
	role, err := c.certRepo.GetRole(ctx, req.Uid, req.Role)
	if err != nil {
		return nil, err
	}
	return &response.GetRoleResponse{
		Uid:  role.Uid,
		Role: role.Role,
	}, nil
}

func (c CertService) UpdateRole(ctx context.Context, req request.UpdateRoleRequest) error {
	if err := c.certRepo.UpdateRole(ctx, req.Uid, req.Role); err != nil {
		return err
	}

	return nil
}

func (c CertService) DeleteRole(ctx context.Context, req request.DeleteRoleRequest) error {
	if err := c.certRepo.DeleteRole(ctx, req.Uid, req.Role); err != nil {
		return err
	}

	return nil
}
