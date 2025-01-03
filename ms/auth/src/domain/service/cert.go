package service

import (
	"auth/driver/model"
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
	if err := c.certRepo.CreateScope(ctx, model.InsertScope{
		Uid:  req.Uid,
		Cid:  req.Cid,
		Auth: req.Auth,
	}); err != nil {
		return err
	}

	return nil
}

func (c CertService) GetScope(ctx context.Context, req request.GetScopeRequest) (*response.GetScopeResponse, error) {
	scope, err := c.certRepo.GetScope(ctx, model.SelectScope{
		Uid: req.Uid,
		Cid: req.Cid,
	})
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
	if err := c.certRepo.UpdateScope(ctx, model.UpdateScope{
		Uid:  req.Uid,
		Cid:  req.Cid,
		Auth: req.Auth,
	}); err != nil {
		return err
	}

	return nil
}

func (c CertService) DeleteScope(ctx context.Context, req request.DeleteScopeRequest) error {
	if err := c.certRepo.DeleteScope(ctx, model.DeleteScope{
		Uid: req.Uid,
		Cid: req.Cid,
	}); err != nil {
		return err
	}

	return nil
}

func (c CertService) CreateRole(ctx context.Context, req request.CreateRoleRequest) error {
	if err := c.certRepo.CreateRole(ctx, model.InsertRole{
		Uid:  req.Uid,
		Role: req.Role,
	}); err != nil {
		return err
	}

	return nil
}

func (c CertService) GetRole(ctx context.Context, req request.GetRoleRequest) (*response.GetRoleResponse, error) {
	role, err := c.certRepo.GetRole(ctx, model.SelectRole{
		Uid:  req.Uid,
		Role: req.Role,
	})
	if err != nil {
		return nil, err
	}

	return &response.GetRoleResponse{
		Uid:  role.Uid,
		Role: role.Role,
	}, nil
}

func (c CertService) UpdateRole(ctx context.Context, req request.UpdateRoleRequest) error {
	if err := c.certRepo.UpdateRole(ctx, model.UpdateRole{
		Uid:  req.Uid,
		Role: req.Role,
	}); err != nil {
		return err
	}

	return nil
}

func (c CertService) DeleteRole(ctx context.Context, req request.DeleteRoleRequest) error {
	if err := c.certRepo.DeleteRole(ctx, model.DeleteRole{
		Uid:  req.Uid,
		Role: req.Role,
	}); err != nil {
		return err
	}

	return nil
}
