package service

import (
	"auth/driver/repository"
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

func (c CertService) CreateScope(ctx context.Context) {}

func (c CertService) GetScope(ctx context.Context) {}

func (c CertService) UpdateScope(ctx context.Context) {}

func (c CertService) DeleteScope(ctx context.Context) {}

func (c CertService) CreateRole(ctx context.Context) {}

func (c CertService) GetRole(ctx context.Context) {}

func (c CertService) UpdateRole(ctx context.Context) {}

func (c CertService) DeleteRole(ctx context.Context) {}
